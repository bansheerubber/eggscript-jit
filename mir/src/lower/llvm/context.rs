use anyhow::{Context, Result};
use eggscript_types::{FunctionType, KnownTypeInfo, Primitive, Type, TypeHandle, TypeStore, P};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::passes::PassBuilderOptions;
use inkwell::targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetMachine};
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::values::{BasicValueEnum, FloatValue, FunctionValue, PointerValue};
use inkwell::{context, FloatPredicate, OptimizationLevel};
use std::collections::HashMap;
use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::lower::CommonContext;
use crate::{BinaryOperator, MIRInfo, PrimitiveValue, Transition, Unit, Value, MIR};

pub struct LlvmLowerContext<'a, 'ctx> {
	builder: &'a Builder<'ctx>,
	common_context: CommonContext,
	context: &'ctx context::Context,
	module: &'a Module<'ctx>,
	units_to_blocks: HashMap<usize, BasicBlock<'ctx>>,
	value_to_basic_value: HashMap<usize, BasicValueEnum<'ctx>>,
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
			units_to_blocks: HashMap::new(),
			value_to_basic_value: HashMap::new(),
		}
	}

	pub fn compile_to_ir(
		&mut self,
		units: Vec<Unit>,
		function: Option<FunctionType>,
	) -> Result<FunctionValue<'ctx>> {
		self.common_context.build_value_dependencies(&units);
		self.common_context
			.type_check_units(&units, function.as_ref());
		return self.lower_units(units, function.as_ref());
	}

	pub fn write(&self) {
		self.module.write_bitcode_to_path(Path::new("./test.bc"));
	}

	pub fn pre_define_function(&self, function: &FunctionType) {
		let args = function
			.argument_types
			.iter()
			.map(|arg_type| self.type_to_llvm_basic_type(*arg_type).into())
			.collect::<Vec<BasicMetadataTypeEnum>>();

		let fn_type = if let Some(return_type) = function.return_type {
			self.type_to_llvm_basic_type(return_type)
				.fn_type(&args, false)
		} else {
			self.context.void_type().fn_type(&args, false)
		};

		self.module.add_function(&function.name, fn_type, None);
	}

	pub fn optimize_ir(&mut self) {
		Target::initialize_all(&InitializationConfig::default());
		let target_triple = TargetMachine::get_default_triple();
		let target = Target::from_triple(&target_triple).unwrap();
		let target_machine = target
			.create_target_machine(
				&target_triple,
				"generic",
				"",
				OptimizationLevel::None,
				RelocMode::PIC,
				CodeModel::Default,
			)
			.unwrap();

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
			.unwrap();
	}

	fn type_to_llvm_basic_type(&self, ty: TypeHandle) -> BasicTypeEnum<'ctx> {
		let type_store = self.common_context.type_store.lock().unwrap();
		let ty = type_store.get_type(ty);
		match ty {
			Some(Type::FunctionReturn { .. }) => todo!(),
			Some(Type::Known { info, .. }) => match info {
				KnownTypeInfo::Primitive(primitive) => match primitive {
					Primitive::Double => self.context.f64_type().into(),
					Primitive::Char => self.context.i8_type().into(),
					Primitive::I64 => self.context.i64_type().into(),
					Primitive::String => todo!(),
					Primitive::Null => todo!(),
				},
			},
			Some(Type::Unknown { .. }) => todo!(),
			None => unreachable!(),
		}
	}

	fn lower_units(
		&mut self,
		units: Vec<Unit>,
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

		for unit in units.iter() {
			self.lower_unit(&unit, llvm_function)?;
		}

		for i in 0..units.len() {
			let unit = &units[i];
			match &unit.transition {
				Transition::Goto(other) => {
					self.builder
						.position_at_end(*self.units_to_blocks.get(&unit.id).unwrap());

					self.builder
						.build_unconditional_branch(*self.units_to_blocks.get(other).unwrap())?;
				}
				Transition::GotoIfFalse(else_unit, value) => {
					self.builder
						.position_at_end(*self.units_to_blocks.get(&unit.id).unwrap());

					let value = self.value_to_basic_value.get(&value.id()).unwrap();
					let then_block = self.units_to_blocks.get(&units[i + 1].id).unwrap();

					self.builder.build_conditional_branch(
						value.into_int_value(),
						*then_block,
						*self.units_to_blocks.get(else_unit).unwrap(),
					)?;
				}
				Transition::GotoIfTrue(then_unit, value) => {
					self.builder
						.position_at_end(*self.units_to_blocks.get(&unit.id).unwrap());

					let value = self.value_to_basic_value.get(&value.id()).unwrap();
					let else_block = self.units_to_blocks.get(&units[i + 1].id).unwrap();

					self.builder.build_conditional_branch(
						value.into_int_value(),
						*self.units_to_blocks.get(then_unit).unwrap(),
						*else_block,
					)?;
				}
				Transition::Invalid => unreachable!(),
				Transition::Next => {
					self.builder
						.position_at_end(*self.units_to_blocks.get(&unit.id).unwrap());

					if i + 1 >= units.len() {
						if function.is_some() {
							// TODO fix type issue
							self.builder
								.build_return(Some(&self.context.f64_type().const_zero()))?;
						}

						continue;
					}

					self.builder.build_unconditional_branch(
						*self.units_to_blocks.get(&units[i + 1].id).unwrap(),
					)?;
				}
				Transition::Return(value) => {
					self.builder
						.position_at_end(*self.units_to_blocks.get(&unit.id).unwrap());

					if let Some(value) = value {
						// TODO fix type issue
						self.builder
							.build_return(Some(&self.value_to_llvm_float_value(value)?))?;
					} else {
						self.builder.build_return(None)?;
					}
				}
			}
		}

		if function.is_none() {
			self.builder
				.position_at_end(*self.units_to_blocks.get(&units.last().unwrap().id).unwrap());

			self.builder.build_return(None)?;
		}

		assert!(llvm_function.verify(true));

		Ok(llvm_function)
	}

	fn lower_unit(&mut self, unit: &Unit, function: FunctionValue<'ctx>) -> Result<()> {
		let block = self
			.context
			.append_basic_block(function, &format!("unit{}", unit.id));

		self.builder.position_at_end(block);

		for mir in unit.mir.iter() {
			self.lower_mir(mir, function)?;
		}

		self.units_to_blocks.insert(unit.id, block);

		Ok(())
	}

	fn value_to_llvm_float_value(&self, value: &P<Value>) -> Result<FloatValue<'ctx>> {
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
				PrimitiveValue::Double(value) => Ok(self.context.f64_type().const_float(*value)),
				PrimitiveValue::Integer(_) => unreachable!(),
				PrimitiveValue::String(_) => unreachable!(),
			},
			Value::Temp { id, .. } => {
				let basic_value = self.value_to_basic_value.get(id).unwrap();

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
						self.type_to_llvm_basic_type(value.ty()),
						&format!("temp{}_", value.id()),
					)?
					.into(),
			);
		}

		Ok(())
	}

	fn lower_mir(&mut self, mir: &MIR, function: FunctionValue<'ctx>) -> Result<()> {
		match &mir.info {
			MIRInfo::Allocate(value, argument_position) => {
				let alloca = self
					.builder
					.build_alloca(self.context.f64_type(), &format!("variable{}_", value.id()))?;

				if let Some(argument_position) = argument_position {
					let params = function.get_params();
					let argument_value = params
						.get(*argument_position)
						.context("Could not get argument value")?;

					self.builder.build_store(alloca, *argument_value)?;
				}

				self.value_to_basic_value.insert(value.id(), alloca.into());
			}
			MIRInfo::BinaryOperation(value, left_operand, right_operand, operator) => {
				match operator {
					BinaryOperator::Plus => {
						self.value_to_basic_value.insert(
							value.id(),
							self.builder
								.build_float_add(
									self.value_to_llvm_float_value(left_operand)?,
									self.value_to_llvm_float_value(right_operand)?,
									&format!("add_result{}_", value.id()),
								)?
								.into(),
						);
					}
					BinaryOperator::Minus => {
						self.value_to_basic_value.insert(
							value.id(),
							self.builder
								.build_float_sub(
									self.value_to_llvm_float_value(left_operand)?,
									self.value_to_llvm_float_value(right_operand)?,
									&format!("sub_result{}_", value.id()),
								)?
								.into(),
						);
					}
					BinaryOperator::Multiply => {
						self.value_to_basic_value.insert(
							value.id(),
							self.builder
								.build_float_mul(
									self.value_to_llvm_float_value(left_operand)?,
									self.value_to_llvm_float_value(right_operand)?,
									&format!("mul_result{}_", value.id()),
								)?
								.into(),
						);
					}
					BinaryOperator::Divide => {
						self.value_to_basic_value.insert(
							value.id(),
							self.builder
								.build_float_div(
									self.value_to_llvm_float_value(left_operand)?,
									self.value_to_llvm_float_value(right_operand)?,
									&format!("div_result{}_", value.id()),
								)?
								.into(),
						);
					}
					BinaryOperator::Modulus => {
						self.value_to_basic_value.insert(
							value.id(),
							self.builder
								.build_float_rem(
									self.value_to_llvm_float_value(left_operand)?,
									self.value_to_llvm_float_value(right_operand)?,
									&format!("mod_result{}_", value.id()),
								)?
								.into(),
						);
					}
					BinaryOperator::Equal => {
						self.value_to_basic_value.insert(
							value.id(),
							self.builder
								.build_float_compare(
									FloatPredicate::OEQ,
									self.value_to_llvm_float_value(left_operand)?,
									self.value_to_llvm_float_value(right_operand)?,
									&format!("eq_result{}_", value.id()),
								)?
								.into(),
						);
					}
					BinaryOperator::NotEqual => {
						self.value_to_basic_value.insert(
							value.id(),
							self.builder
								.build_float_compare(
									FloatPredicate::ONE,
									self.value_to_llvm_float_value(left_operand)?,
									self.value_to_llvm_float_value(right_operand)?,
									&format!("ne_result{}_", value.id()),
								)?
								.into(),
						);
					}
					BinaryOperator::LessThan => {
						self.value_to_basic_value.insert(
							value.id(),
							self.builder
								.build_float_compare(
									FloatPredicate::OLT,
									self.value_to_llvm_float_value(left_operand)?,
									self.value_to_llvm_float_value(right_operand)?,
									&format!("lt_result{}_", value.id()),
								)?
								.into(),
						);
					}
					BinaryOperator::GreaterThan => {
						self.value_to_basic_value.insert(
							value.id(),
							self.builder
								.build_float_compare(
									FloatPredicate::OGT,
									self.value_to_llvm_float_value(left_operand)?,
									self.value_to_llvm_float_value(right_operand)?,
									&format!("gt_result{}_", value.id()),
								)?
								.into(),
						);
					}
					BinaryOperator::LessThanEqualTo => {
						self.value_to_basic_value.insert(
							value.id(),
							self.builder
								.build_float_compare(
									FloatPredicate::OLE,
									self.value_to_llvm_float_value(left_operand)?,
									self.value_to_llvm_float_value(right_operand)?,
									&format!("le_result{}_", value.id()),
								)?
								.into(),
						);
					}
					BinaryOperator::GreaterThanEqualTo => {
						self.value_to_basic_value.insert(
							value.id(),
							self.builder
								.build_float_compare(
									FloatPredicate::OGE,
									self.value_to_llvm_float_value(left_operand)?,
									self.value_to_llvm_float_value(right_operand)?,
									&format!("ge_result{}_", value.id()),
								)?
								.into(),
						);
					}
					_ => todo!(),
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
					args.push(self.value_to_llvm_float_value(argument)?.into());
				}

				let type_store = self.common_context.type_store.lock().unwrap();
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
						llvm_return_value.try_as_basic_value().left().unwrap(),
					)?;
				}
			}
			MIRInfo::StoreLiteral(value, primitive_value) => {
				self.alloc_llvm_value(value)?;

				match primitive_value {
					PrimitiveValue::Double(number) => {
						self.builder.build_store(
							self.value_to_llvm_pointer_value(&value)?,
							self.context.f64_type().const_float(*number),
						)?;
					}
					PrimitiveValue::Integer(number) => {
						self.builder.build_store(
							self.value_to_llvm_pointer_value(&value)?,
							self.context.i64_type().const_int(*number as u64, false),
						)?;
					}
					PrimitiveValue::String(_) => todo!(),
				}
			}
			MIRInfo::StoreValue(lvalue, rvalue) => {
				self.alloc_llvm_value(lvalue)?;

				let value = self.value_to_basic_value.get(&rvalue.id()).unwrap();
				let value = if value.is_pointer_value() {
					self.builder.build_load(
						self.type_to_llvm_basic_type(rvalue.ty()),
						value.into_pointer_value(),
						&format!("tmpforstore{}_", rvalue.id()),
					)?
				} else {
					*value
				};

				self.builder
					.build_store(self.value_to_llvm_pointer_value(&lvalue)?, value)?;
			}
		}

		Ok(())
	}
}
