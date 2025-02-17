use anyhow::{Context, Result};
use eggscript_interpreter::{Instruction, RelativeStackAddress};
use eggscript_types::{TypeStore, P};
use std::{
	collections::HashMap,
	ops::Deref,
	sync::{Arc, Mutex},
};

use crate::{MIRInfo, Transition, Unit, UnitHandle, Value, ValueStore, MIR};

pub struct EggscriptLowerContext {
	allocations: Vec<P<Value>>,
	jump_instructions: Vec<usize>,
	type_store: Arc<Mutex<TypeStore>>,
	unit_to_instruction: HashMap<UnitHandle, usize>,
	value_used_by: HashMap<usize, Vec<usize>>,
	value_to_stack: HashMap<usize, usize>,
	#[allow(dead_code)]
	value_store: ValueStore,
}

impl EggscriptLowerContext {
	pub fn new(type_store: Arc<Mutex<TypeStore>>, value_store: ValueStore) -> Self {
		EggscriptLowerContext {
			allocations: Vec::new(),
			jump_instructions: Vec::new(),
			type_store,
			unit_to_instruction: HashMap::new(),
			value_used_by: HashMap::new(),
			value_to_stack: HashMap::new(),
			value_store,
		}
	}

	pub fn compile_to_eggscript(&mut self, units: Vec<Unit>) -> Result<Vec<Instruction>> {
		self.build_value_dependencies(&units);
		self.type_check(&units);
		return self.lower_units(units);
	}

	fn lower_units(&mut self, units: Vec<Unit>) -> Result<Vec<Instruction>> {
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

	fn type_check(&mut self, units: &Vec<Unit>) {
		let type_store = self.type_store.lock().unwrap();
		for unit in units.iter() {
			for mir in unit.mir.iter() {
				match &mir.info {
					MIRInfo::Allocate(_, _) => {}
					MIRInfo::BinaryOperation(result, left, right, _) => {
						assert!(
							type_store.are_types_compatible(result.ty(), left.ty()),
							"result not compatible with left"
						);
						assert!(
							type_store.are_types_compatible(result.ty(), right.ty()),
							"result not compatible with right"
						);
						assert!(
							type_store.are_types_compatible(left.ty(), right.ty()),
							"left not compatible with right"
						);
					}
					MIRInfo::CallFunction(function_name, _, arguments, _) => {
						let mut index = 0;
						let function = type_store.get_function(function_name).unwrap();
						for argument in arguments.iter() {
							assert!(type_store.are_types_compatible(
								argument.ty(),
								*function.argument_types.get(index).unwrap()
							));
							index += 1;
						}
					}
					MIRInfo::StoreLiteral(lvalue, rvalue) => {
						assert!(
							type_store.are_types_compatible(
								lvalue.ty(),
								rvalue.get_type_from_type_store(&type_store)
							),
							"lvalue not compatible with rvalue"
						);
					}
					MIRInfo::StoreValue(lvalue, rvalue) => {
						assert!(
							type_store.are_types_compatible(lvalue.ty(), rvalue.ty()),
							"lvalue not compatible with rvalue"
						);
					}
				}
			}
		}
	}

	fn build_value_dependencies(&mut self, units: &Vec<Unit>) {
		for unit in units.iter() {
			for mir in unit.mir.iter() {
				match &mir.info {
					MIRInfo::BinaryOperation(lvalue, operand1, operand2, _) => {
						self.value_used_by
							.entry(operand1.id())
							.or_default()
							.push(lvalue.id());

						self.value_used_by
							.entry(operand2.id())
							.or_default()
							.push(lvalue.id());
					}
					MIRInfo::CallFunction(_, _, arguments, result) => {
						for argument in arguments.iter() {
							self.value_used_by
								.entry(argument.id())
								.or_default()
								.push(result.id());
						}
					}
					MIRInfo::StoreValue(lvalue, rvalue) => {
						self.value_used_by
							.entry(rvalue.id())
							.or_default()
							.push(lvalue.id());
					}
					_ => {}
				}
			}
		}
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
			MIRInfo::BinaryOperation(_, left, right, operator) => {
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

				if let Value::Primitive { value, .. } = left.deref() {
					instructions.push(Instruction::Push(value.into()));
				}

				if let Value::Primitive { value, .. } = right.deref() {
					instructions.push(Instruction::Push(value.into()));
				}

				instructions.push(Instruction::DoubleMath(
					operator.into(),
					left_stack_address,
					right_stack_address,
				));

				Ok(instructions)
			}
			MIRInfo::CallFunction(_, function_handle, arguments, result) => {
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

				instructions.push(Instruction::CallFunction(*function_handle));

				// if the result isn't used, then pop it from the stack
				if !self.value_used_by.contains_key(&result.id()) {
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
		}
	}
}
