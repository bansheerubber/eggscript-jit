use anyhow::{Context, Result};
use eggscript_types::{FunctionType, KnownTypeInfo, Primitive, Type, TypeHandle, TypeStore, P};
use indexmap::IndexMap;
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::passes::PassBuilderOptions;
use inkwell::targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{BasicValueEnum, FloatValue, FunctionValue, PhiValue, PointerValue};
use inkwell::{context, FloatPredicate, OptimizationLevel};
use std::collections::HashMap;
use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::lower::CommonContext;
use crate::{
	BinaryOperator, MIRInfo, PrimitiveValue, Transition, UnaryOperator, Unit, UnitHandle, Value,
	MIR,
};

pub struct LlvmLowerContext<'a, 'ctx> {
	pub(crate) builder: &'a Builder<'ctx>,
	pub(crate) common_context: CommonContext,
	pub(crate) context: &'ctx context::Context,
	pub(crate) module: &'a Module<'ctx>,
	pub(crate) phi_value_for_unit: HashMap<usize, PhiValue<'ctx>>,
	pub(crate) units_to_blocks: HashMap<usize, BasicBlock<'ctx>>,
	pub(crate) value_to_basic_value: HashMap<usize, BasicValueEnum<'ctx>>,
}

impl<'a, 'ctx> LlvmLowerContext<'a, 'ctx> {
	pub fn new(
		context: &'ctx context::Context,
		builder: &'a Builder<'ctx>,
		module: &'a Module<'ctx>,
		type_store: Arc<Mutex<TypeStore>>,
		file_name: &str,
	) -> Self {
		LlvmLowerContext {
			builder,
			common_context: CommonContext::new(type_store, file_name),
			context,
			module,
			phi_value_for_unit: HashMap::new(),
			units_to_blocks: HashMap::new(),
			value_to_basic_value: HashMap::new(),
		}
	}

	pub fn compile_to_ir(
		&mut self,
		units: &IndexMap<UnitHandle, Unit>,
		function: Option<FunctionType>,
	) -> Result<FunctionValue<'ctx>> {
		self.common_context.build_value_dependencies(&units);
		self.common_context
			.type_check_units(&units, function.as_ref())?;

		return self.lower_units(units, function.as_ref());
	}

	pub fn write(&self) {
		self.module.write_bitcode_to_path(Path::new("./test.bc"));
	}

	pub fn pre_define_function(&self, function: &FunctionType) -> Result<()> {
		let args = function
			.argument_types
			.iter()
			.map(|arg_type| self.type_to_llvm_basic_type(*arg_type))
			.collect::<Result<Vec<BasicTypeEnum<'ctx>>>>()?
			.iter()
			.map(|ty| (*ty).into())
			.collect::<Vec<BasicMetadataTypeEnum>>();

		let fn_type = if let Some(return_type) = function.return_type {
			self.type_to_llvm_basic_type(return_type)?
				.fn_type(&args, false)
		} else {
			self.context.void_type().fn_type(&args, false)
		};

		self.module.add_function(&function.name, fn_type, None);

		Ok(())
	}

	// TODO if we fail to optimize the LLVM code, then maybe the machine isn't capable of running
	// JIT'ed code?
	pub fn optimize_ir(&mut self) {
		Target::initialize_all(&InitializationConfig::default());
		let target_triple = TargetMachine::get_default_triple();
		let target = Target::from_triple(&target_triple).expect("Could not find target triple");
		let target_machine = target
			.create_target_machine(
				&target_triple,
				"generic",
				"",
				OptimizationLevel::None,
				RelocMode::PIC,
				CodeModel::Default,
			)
			.expect("Could not create target machine");

		let passes: &[&str] = &[
			"instcombine<no-verify-fixpoint>", // combine redundant instructions
			"reassociate", // rearrange math order of operations to be a little more CPU efficient
			"gvn",         // remove redundant loads
			"simplifycfg", // perform dead code elimination and basic block merging
			"mem2reg",     // rewrite some types of loads/stores
		];

		self.module
			.run_passes(
				passes.join(",").as_str(),
				&target_machine,
				PassBuilderOptions::create(),
			)
			.expect("Could not run optimization passes");
	}

	fn type_to_llvm_basic_type(&self, ty: TypeHandle) -> Result<BasicTypeEnum<'ctx>> {
		let type_store = self
			.common_context
			.type_store
			.lock()
			.expect("Could not lock type store");
		let ty = type_store.get_type(
			type_store
				.resolve_type(ty)
				.context("Could not resolve type")?,
		);

		match ty {
			Some(Type::FunctionReturn { .. }) => todo!(),
			Some(Type::Known { info, .. }) => match info {
				KnownTypeInfo::Primitive(primitive) => match primitive {
					Primitive::Number => Ok(self.context.f64_type().into()),
					Primitive::Null => todo!(),
				},
			},
			Some(Type::Unknown { .. }) => todo!(),
			None => unreachable!(),
		}
	}

	fn lower_units(
		&mut self,
		units: &IndexMap<UnitHandle, Unit>,
		function: Option<&FunctionType>,
	) -> Result<FunctionValue<'ctx>> {
		let function_name = if let Some(function) = function {
			&function.name
		} else {
			"entry"
		};

		let llvm_function = if let Some(llvm_function) = self.module.get_function(function_name) {
			llvm_function
		} else {
			self.module.add_function(
				function_name,
				self.context.void_type().fn_type(&[], false),
				None,
			)
		};

		for unit in units.values() {
			self.lower_unit(&unit, llvm_function)?;
		}

		let units_vector = units.values().collect::<Vec<&Unit>>();
		for i in 0..units_vector.len() {
			let unit = units_vector.get(i).expect("Could not get unit");
			match &unit.transition {
				Transition::Goto(other) => {
					self.builder.position_at_end(
						*self
							.units_to_blocks
							.get(&unit.id)
							.expect("Could not find unit"),
					);

					self.builder.build_unconditional_branch(
						*self
							.units_to_blocks
							.get(other)
							.expect("Could not find target unit"),
					)?;
				}
				Transition::GotoIfFalse(else_unit, value) => {
					self.builder.position_at_end(
						*self
							.units_to_blocks
							.get(&unit.id)
							.expect("Could not find unit"),
					);

					let then_block = self
						.units_to_blocks
						.get(&units_vector[i + 1].id)
						.expect("Could not find 'then' unit");

					let value = self.builder.build_float_compare(
						FloatPredicate::ONE,
						self.value_to_llvm_float_value(value)?,
						self.context.f64_type().const_float(0.0),
						"cast_",
					)?;

					self.builder.build_conditional_branch(
						value,
						*then_block,
						*self
							.units_to_blocks
							.get(else_unit)
							.expect("Could not find 'else' unit"),
					)?;
				}
				Transition::GotoIfTrue(then_unit, value) => {
					self.builder.position_at_end(
						*self
							.units_to_blocks
							.get(&unit.id)
							.expect("Could not find unit"),
					);

					let else_block = self
						.units_to_blocks
						.get(&units_vector[i + 1].id)
						.expect("Could not find 'else' unit");

					let value = self.builder.build_float_compare(
						FloatPredicate::ONE,
						self.value_to_llvm_float_value(value)?,
						self.context.f64_type().const_float(0.0),
						"cast_",
					)?;

					self.builder.build_conditional_branch(
						value,
						*self
							.units_to_blocks
							.get(then_unit)
							.expect("Could not find 'then' unit"),
						*else_block,
					)?;
				}
				Transition::Invalid => unreachable!(),
				Transition::Next => {
					self.builder.position_at_end(
						*self
							.units_to_blocks
							.get(&unit.id)
							.expect("Could not find unit"),
					);

					if i + 1 >= units_vector.len() {
						if let Some(function) = function {
							// TODO fix type issue
							self.builder.build_return(Some(
								&self
									.type_to_llvm_basic_type(function.return_type.expect(
										"Expected function return type where there is none",
									))?
									.const_zero(),
							))?;
						}

						continue;
					}

					self.builder.build_unconditional_branch(
						*self
							.units_to_blocks
							.get(&units_vector[i + 1].id)
							.expect("Could not find branch target unit"),
					)?;
				}
				Transition::Return(value) => {
					self.builder.position_at_end(
						*self
							.units_to_blocks
							.get(&unit.id)
							.expect("Could not find unit"),
					);

					if let Some(value) = value {
						// TODO fix type issue
						self.builder
							.build_return(Some(&self.maybe_deref_llvm_value(value)?))?;
					} else {
						self.builder.build_return(None)?;
					}
				}
			}
		}

		for current_unit in units.values() {
			if !current_unit.starts_with_phi() {
				continue;
			}

			let first_instruction = self
				.units_to_blocks
				.get(&current_unit.id)
				.expect("Could not find block")
				.get_first_instruction()
				.expect("Could not get instruction");

			self.builder.position_before(&first_instruction);

			match &current_unit
				.mir
				.first()
				.expect("Could not get first instruction")
				.info
			{
				MIRInfo::LogicPhi(_, _, units_and_values) => {
					for (unit, value) in units_and_values.iter() {
						let is_pointer_value = if value.is_primitive() {
							false
						} else {
							self.value_to_basic_value
								.get(&value.id())
								.expect("Could not find value")
								.is_pointer_value()
						};

						// need to dereference pointer
						let float_value = if is_pointer_value {
							let block = *self
								.units_to_blocks
								.get(unit)
								.expect("Could not find block");

							let last_instruction = block.get_last_instruction();
							if let Some(last_instruction) = last_instruction {
								self.builder.position_before(&last_instruction);
							} else {
								self.builder.position_at_end(block);
							}

							let float_value = self.value_to_llvm_float_value(value)?;

							self.builder.position_at_end(
								*self
									.units_to_blocks
									.get(&current_unit.id)
									.expect("Could not find block"),
							);

							float_value
						} else {
							self.value_to_llvm_float_value(value)?
						};

						self.phi_value_for_unit
							.get(&current_unit.id)
							.expect("Could not find phi instruction for unit")
							.add_incoming(&[(
								&float_value,
								*self
									.units_to_blocks
									.get(unit)
									.expect("Could not find block"),
							)]);
					}
				}
				_ => {}
			}
		}

		if function.is_none() {
			self.builder.position_at_end(
				*self
					.units_to_blocks
					.get(&units_vector.last().expect("Could not get last unit").id)
					.expect("Could not find unit"),
			);

			self.builder.build_return(None)?;
		}

		assert!(llvm_function.verify(true));

		Ok(llvm_function)
	}

	fn lower_unit(&mut self, unit: &Unit, function: FunctionValue<'ctx>) -> Result<()> {
		let block = self
			.context
			.append_basic_block(function, &format!("unit{}", unit.id));

		self.units_to_blocks.insert(unit.id, block);
		self.builder.position_at_end(block);

		for mir in unit.mir.iter() {
			self.lower_mir(unit.id, mir, function)?;
		}

		Ok(())
	}

	pub(crate) fn value_to_llvm_float_value(&self, value: &P<Value>) -> Result<FloatValue<'ctx>> {
		match value.deref() {
			Value::Location { .. } => Ok(self
				.builder
				.build_load(
					self.context.f64_type(),
					self.value_to_llvm_pointer_value(value)?,
					"temp_",
				)?
				.into_float_value()),
			Value::Primitive { value, .. } => match value {
				PrimitiveValue::Number(value) => Ok(self.context.f64_type().const_float(*value)),
			},
			Value::Temp { id, .. } => {
				let basic_value = self
					.value_to_basic_value
					.get(id)
					.expect("Could not find find basic value");

				if basic_value.is_pointer_value() {
					Ok(self
						.builder
						.build_load(
							self.context.f64_type(),
							self.value_to_llvm_pointer_value(value)?,
							"temp_",
						)?
						.into_float_value())
				} else {
					Ok(basic_value.into_float_value())
				}
			}
		}
	}

	pub(crate) fn maybe_deref_llvm_value(&self, value: &P<Value>) -> Result<BasicValueEnum<'ctx>> {
		let type_store = self
			.common_context
			.type_store
			.lock()
			.expect("Could not lock type store");

		let is_primitive = type_store
			.get_type(value.ty())
			.expect("Could not find value type")
			.is_primitive();

		drop(type_store);

		match value.deref() {
			Value::Location { id, .. } | Value::Temp { id, .. } => {
				let basic_value = self
					.value_to_basic_value
					.get(id)
					.expect("Could not find basic value");

				if is_primitive && basic_value.is_pointer_value() {
					return Ok(self
						.builder
						.build_load(
							self.type_to_llvm_basic_type(value.ty())?,
							self.value_to_llvm_pointer_value(value)?,
							"temp_",
						)?
						.into());
				}

				return Ok(basic_value.clone());
			}
			Value::Primitive { value, .. } => match value {
				PrimitiveValue::Number(value) => {
					Ok(self.context.f64_type().const_float(*value).into())
				}
			},
		}
	}

	fn value_to_llvm_pointer_value(&self, value: &P<Value>) -> Result<PointerValue<'ctx>> {
		Ok(self
			.value_to_basic_value
			.get(&value.id())
			.context("Could not convert to pointer value")?
			.into_pointer_value())
	}

	fn alloc_llvm_value(&mut self, value: &P<Value>) -> Result<()> {
		if !self.value_to_basic_value.contains_key(&value.id()) {
			self.value_to_basic_value.insert(
				value.id(),
				self.builder
					.build_alloca(
						self.type_to_llvm_basic_type(value.ty())?,
						&format!("temp{}_", value.id()),
					)?
					.into(),
			);
		}

		Ok(())
	}

	fn lower_mir(
		&mut self,
		current_unit: UnitHandle,
		mir: &MIR,
		function: FunctionValue<'ctx>,
	) -> Result<()> {
		match &mir.info {
			MIRInfo::Allocate(value, argument_position) => {
				let alloca = self.builder.build_alloca(
					self.type_to_llvm_basic_type(value.ty())?,
					&format!("variable{}_", value.id()),
				)?;

				if let Some(argument_position) = argument_position {
					let params = function.get_params();
					let argument_value = params
						.get(*argument_position)
						.context("Could not get argument value")?;

					self.builder.build_store(alloca, *argument_value)?;
				}

				self.value_to_basic_value.insert(value.id(), alloca.into());
			}
			MIRInfo::BinaryOperation(result_value, left_operand, right_operand, operator) => {
				match operator {
					BinaryOperator::Plus => {
						let result = self.build_add(result_value, left_operand, right_operand)?;
						self.value_to_basic_value.insert(result_value.id(), result);
					}
					BinaryOperator::Minus => {
						let result = self.build_sub(result_value, left_operand, right_operand)?;
						self.value_to_basic_value.insert(result_value.id(), result);
					}
					BinaryOperator::Multiply => {
						let result = self.build_mul(result_value, left_operand, right_operand)?;
						self.value_to_basic_value.insert(result_value.id(), result);
					}
					BinaryOperator::Divide => {
						let result = self.build_div(result_value, left_operand, right_operand)?;
						self.value_to_basic_value.insert(result_value.id(), result);
					}
					BinaryOperator::Modulus => {
						let result = self.build_rem(result_value, left_operand, right_operand)?;
						self.value_to_basic_value.insert(result_value.id(), result);
					}
					BinaryOperator::BitwiseAnd => {
						let result =
							self.build_bitwise_and(result_value, left_operand, right_operand)?;

						self.value_to_basic_value.insert(result_value.id(), result);
					}
					BinaryOperator::BitwiseOr => {
						let result =
							self.build_bitwise_or(result_value, left_operand, right_operand)?;

						self.value_to_basic_value.insert(result_value.id(), result);
					}
					BinaryOperator::BitwiseXor => {
						let result =
							self.build_bitwise_xor(result_value, left_operand, right_operand)?;

						self.value_to_basic_value.insert(result_value.id(), result);
					}
					BinaryOperator::ShiftLeft => {
						let result =
							self.build_shift_left(result_value, left_operand, right_operand)?;

						self.value_to_basic_value.insert(result_value.id(), result);
					}
					BinaryOperator::ShiftRight => {
						let result =
							self.build_shift_right(result_value, left_operand, right_operand)?;

						self.value_to_basic_value.insert(result_value.id(), result);
					}
					BinaryOperator::Equal
					| BinaryOperator::NotEqual
					| BinaryOperator::LessThan
					| BinaryOperator::GreaterThan
					| BinaryOperator::LessThanEqualTo
					| BinaryOperator::GreaterThanEqualTo => {
						let result =
							self.build_cmp(result_value, left_operand, right_operand, operator)?;
						self.value_to_basic_value.insert(result_value.id(), result);
					}
					BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr => unreachable!(),
				}
			}
			MIRInfo::CallFunction(name, _, arguments, return_value) => {
				let llvm_function = self
					.module
					.get_function(name)
					.context("Could not find LLVM function")?;

				let mut args = vec![];
				for argument in arguments.iter() {
					// TODO fix type issue
					args.push(self.maybe_deref_llvm_value(argument)?.into());
				}

				let type_store = self
					.common_context
					.type_store
					.lock()
					.expect("Could not lock type store");
				let function_type = type_store
					.get_function(name)
					.context("Could not find function")?;

				let llvm_return_value = self.builder.build_call(
					llvm_function,
					&args,
					&format!("returnval{}_", return_value.id()),
				)?;

				if function_type.return_type.is_some() {
					drop(type_store);

					self.alloc_llvm_value(return_value)?;

					self.builder.build_store(
						self.value_to_llvm_pointer_value(&return_value)?,
						llvm_return_value
							.try_as_basic_value()
							.left()
							.expect("Expected return basic value where there is none"),
					)?;
				}
			}
			MIRInfo::LogicPhi(result, _, _) => {
				// TODO type stuff???
				let phi_result = self.builder.build_phi(self.context.f64_type(), "phi_")?;

				self.value_to_basic_value
					.insert(result.id(), phi_result.as_basic_value());

				self.phi_value_for_unit.insert(current_unit, phi_result);
			}
			MIRInfo::StoreLiteral(value, primitive_value) => {
				self.alloc_llvm_value(value)?;

				match primitive_value {
					PrimitiveValue::Number(number) => {
						self.builder.build_store(
							self.value_to_llvm_pointer_value(&value)?,
							self.context.f64_type().const_float(*number),
						)?;
					}
				}
			}
			MIRInfo::StoreValue(lvalue, rvalue) => {
				self.alloc_llvm_value(lvalue)?;

				let value = self
					.value_to_basic_value
					.get(&rvalue.id())
					.expect("Could not find basic value");

				let value = if value.is_pointer_value() {
					self.builder.build_load(
						self.type_to_llvm_basic_type(rvalue.ty())?,
						value.into_pointer_value(),
						&format!("tmpforstore{}_", rvalue.id()),
					)?
				} else {
					*value
				};

				self.builder
					.build_store(self.value_to_llvm_pointer_value(&lvalue)?, value)?;
			}
			MIRInfo::Unary(result_value, rvalue, operator) => {
				let result = match operator {
					UnaryOperator::BitwiseNot => self.build_bitwise_not(result_value, rvalue)?,
					UnaryOperator::Minus => self.build_neg(result_value, rvalue)?,
					UnaryOperator::Not => self.build_not(result_value, rvalue)?,
				};

				self.value_to_basic_value.insert(result_value.id(), result);
			}
		}

		Ok(())
	}
}
