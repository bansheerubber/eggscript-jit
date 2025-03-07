use anyhow::{Context, Result};
use eggscript_interpreter::{Instruction, RelativeStackAddress};
use eggscript_types::{FunctionType, TypeStore, P};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use crate::lower::CommonContext;
use crate::{MIRInfo, Transition, Unit, UnitHandle, Value, MIR};

pub struct EggscriptLowerContext {
	allocations: Vec<P<Value>>,
	common_context: CommonContext,
	jump_instructions: Vec<usize>,
	unit_to_instruction: HashMap<UnitHandle, usize>,
	value_to_stack: HashMap<usize, usize>,
}

impl EggscriptLowerContext {
	pub fn new(type_store: Arc<Mutex<TypeStore>>, file_name: &str) -> Self {
		EggscriptLowerContext {
			allocations: Vec::new(),
			common_context: CommonContext::new(type_store, file_name),
			jump_instructions: Vec::new(),
			unit_to_instruction: HashMap::new(),
			value_to_stack: HashMap::new(),
		}
	}

	pub fn compile_to_eggscript(
		&mut self,
		units: &Vec<Unit>,
		function: Option<FunctionType>,
	) -> Result<Vec<Instruction>> {
		self.common_context.build_value_dependencies(&units);
		self.common_context
			.type_check_units(&units, function.as_ref());
		return self.lower_units(units);
	}

	fn lower_units(&mut self, units: &Vec<Unit>) -> Result<Vec<Instruction>> {
		let mut instructions = vec![];
		for unit in units.iter() {
			let start = instructions.len();
			self.unit_to_instruction.insert(unit.id, start);
			instructions.append(&mut self.lower_unit(unit, start)?);
		}

		for jump_instruction in self.jump_instructions.iter() {
			let instruction = instructions
				.get(*jump_instruction)
				.context("Could not get jump instruction")?;

			match instruction {
				Instruction::Jump(unit_handle) => {
					let index = *unit_handle as usize;
					let relative_jump = *self
						.unit_to_instruction
						.get(&index)
						.context("Could not find unit to jump to")? as isize
						- *jump_instruction as isize;

					instructions[*jump_instruction] = Instruction::Jump(relative_jump);
				}
				Instruction::JumpIfFalse(unit_handle, value) => {
					let index = *unit_handle as usize;
					let relative_jump = *self
						.unit_to_instruction
						.get(&index)
						.context("Could not find unit to jump to")? as isize
						- *jump_instruction as isize;

					instructions[*jump_instruction] =
						Instruction::JumpIfFalse(relative_jump, *value);
				}
				Instruction::JumpIfTrue(unit_handle, value) => {
					let index = *unit_handle as usize;
					let relative_jump = *self
						.unit_to_instruction
						.get(&index)
						.context("Could not find unit to jump to")? as isize
						- *jump_instruction as isize;

					instructions[*jump_instruction] =
						Instruction::JumpIfTrue(relative_jump, *value);
				}
				_ => unreachable!("{:?}", instruction),
			}
		}

		instructions.insert(0, Instruction::Reserve(self.allocations.len()));

		Ok(instructions)
	}

	fn lower_unit(&mut self, unit: &Unit, instruction_index: usize) -> Result<Vec<Instruction>> {
		let mut instructions = vec![];
		for mir in unit.mir.iter() {
			instructions.append(&mut self.lower_mir(mir)?);
		}

		if unit.mir.len() == 0 {
			instructions.push(Instruction::Noop);
		}

		match &unit.transition {
			Transition::Goto(position) => {
				self.jump_instructions
					.push(instruction_index + instructions.len());
				instructions.push(Instruction::Jump(*position as isize));
			}
			Transition::GotoIfFalse(position, value) => {
				let stack_address = match value.deref() {
					Value::Location { id, .. } => *self
						.value_to_stack
						.get(id)
						.context("Could not get left value stack index")?
						as RelativeStackAddress,

					Value::Primitive { .. } => -1,
					Value::Temp { .. } => -1,
				};

				self.jump_instructions
					.push(instruction_index + instructions.len());
				instructions.push(Instruction::JumpIfFalse(*position as isize, stack_address));
			}
			Transition::GotoIfTrue(position, value) => {
				let stack_address = match value.deref() {
					Value::Location { id, .. } => *self
						.value_to_stack
						.get(id)
						.context("Could not get left value stack index")?
						as RelativeStackAddress,

					Value::Primitive { .. } => -1,
					Value::Temp { .. } => -1,
				};

				self.jump_instructions
					.push(instruction_index + instructions.len());

				instructions.push(Instruction::JumpIfTrue(*position as isize, stack_address));
			}
			Transition::Invalid => todo!(),
			Transition::Next => {}
			Transition::Return(value) => {
				instructions.push(Instruction::Return(value.is_some()));
			}
		}

		Ok(instructions)
	}

	fn lower_mir(&mut self, mir: &MIR) -> Result<Vec<Instruction>> {
		match &mir.info {
			MIRInfo::Allocate(value, stack_position) => {
				if stack_position.is_none() {
					self.allocations.push(value.clone());
				}

				self.value_to_stack.insert(
					value.id(),
					if let Some(stack_position) = stack_position {
						*stack_position
					} else {
						self.value_to_stack.len()
					},
				);

				Ok(vec![])
			}
			MIRInfo::BinaryOperation(result, left, right, operator) => {
				let mut instructions: Vec<Instruction> = Vec::new();

				let left_stack_address = match left.deref() {
					Value::Location { id, .. } => *self
						.value_to_stack
						.get(id)
						.context("Could not get left value stack index")?
						as RelativeStackAddress,
					Value::Primitive { .. } => -1,
					Value::Temp { .. } => -1, // TODO probably wrong
				};

				let right_stack_address = match right.deref() {
					Value::Location { id, .. } => *self
						.value_to_stack
						.get(id)
						.context("Could not get left value stack index")?
						as RelativeStackAddress,
					Value::Primitive { .. } => -1,
					Value::Temp { .. } => -1, // TODO probably wrong
				};

				if let Value::Primitive { value, .. } = right.deref() {
					instructions.push(Instruction::Push(value.into()));
				}

				let type_store = self.common_context.type_store.lock().unwrap();
				let result_ty = type_store.get_type(result.ty()).unwrap();
				let name = result_ty.get_name().unwrap();

				// TODO clean up nested bullshit
				if let Value::Primitive { value, .. } = left.deref() {
					if name == "double" {
						instructions.push(Instruction::ImmediateDoubleMath(
							operator.into(),
							value.into(),
							right_stack_address,
						));
					} else if name == "int" {
						instructions.push(Instruction::ImmediateIntegerMath(
							operator.into(),
							value.into(),
							right_stack_address,
						));
					} else {
						unreachable!();
					}
				} else {
					if name == "double" {
						instructions.push(Instruction::DoubleMath(
							operator.into(),
							left_stack_address,
							right_stack_address,
						));
					} else if name == "int" {
						instructions.push(Instruction::IntegerMath(
							operator.into(),
							left_stack_address,
							right_stack_address,
						));
					} else {
						unreachable!();
					}
				}

				Ok(instructions)
			}
			MIRInfo::CallFunction(name, function_handle, arguments, result) => {
				let mut instructions = vec![];
				for argument in arguments.iter() {
					match argument.deref() {
						Value::Location { id, .. } => {
							let stack_address = self
								.value_to_stack
								.get(id)
								.context("Could not get argument stack index")?;

							instructions.push(Instruction::CopyPush(*stack_address));
						}
						Value::Primitive { value, .. } => {
							instructions.push(Instruction::Push(value.into()));
						}
						Value::Temp { .. } => { /* do nothing */ }
					}
				}

				let type_store = self.common_context.type_store.lock().unwrap();
				let function_type = type_store.get_function(name).unwrap();

				instructions.push(Instruction::CallFunction(*function_handle));

				// if the result isn't used, then pop it from the stack
				if !self.common_context.value_used_by.contains_key(&result.id())
					&& function_type.return_type.is_some()
				{
					instructions.push(Instruction::Pop);
				}

				Ok(instructions)
			}
			MIRInfo::StoreLiteral(lvalue, rvalue) => {
				let left_stack_address = match lvalue.deref() {
					Value::Location { .. } => *self.value_to_stack.get(&lvalue.id()).context(
						format!("Value {} has not been allocated to stack", lvalue.id()),
					)? as isize,
					Value::Primitive { .. } => unreachable!(),
					Value::Temp { .. } => -1,
				};

				if left_stack_address == -1 {
					Ok(vec![Instruction::Push(rvalue.into())])
				} else {
					Ok(vec![
						Instruction::Push(rvalue.into()),
						Instruction::Store(left_stack_address as usize, -1),
					])
				}
			}
			MIRInfo::StoreValue(lvalue, rvalue) => {
				let left_stack_address = match lvalue.deref() {
					Value::Location { .. } => *self.value_to_stack.get(&lvalue.id()).context(
						format!("Value {} has not been allocated to stack", lvalue.id()),
					)? as isize,
					Value::Primitive { .. } => unreachable!(),
					Value::Temp { .. } => -1,
				};

				let right_stack_address = match rvalue.deref() {
					Value::Location { id, .. } => *self
						.value_to_stack
						.get(id)
						.context("Could not get left value stack index")?
						as RelativeStackAddress,
					Value::Primitive { .. } => unreachable!(),
					Value::Temp { .. } => -1, // TODO probably wrong
				};

				if left_stack_address == -1 {
					assert!(right_stack_address != -1);
					Ok(vec![Instruction::CopyPush(right_stack_address as usize)])
				} else {
					Ok(vec![Instruction::Store(
						left_stack_address as usize,
						right_stack_address,
					)])
				}
			}
			MIRInfo::Unary(result, rvalue, operator) => {
				let type_store = self.common_context.type_store.lock().unwrap();
				let result_ty = type_store.get_type(result.ty()).unwrap();
				let name = result_ty.get_name().unwrap();

				let right_stack_address = match rvalue.deref() {
					Value::Location { id, .. } => *self
						.value_to_stack
						.get(id)
						.context("Could not get left value stack index")?
						as RelativeStackAddress,
					Value::Primitive { .. } => -1,
					Value::Temp { .. } => -1, // TODO probably wrong
				};

				let mut instructions = vec![];

				if let Value::Primitive { value, .. } = rvalue.deref() {
					instructions.push(Instruction::Push(value.into()));
				}

				if name == "double" {
					instructions.push(Instruction::DoubleUnary(
						operator.into(),
						right_stack_address,
					));
				} else if name == "int" {
					instructions.push(Instruction::IntegerUnary(
						operator.into(),
						right_stack_address,
					));
				} else {
					unreachable!();
				}

				return Ok(instructions);
			}
		}
	}
}
