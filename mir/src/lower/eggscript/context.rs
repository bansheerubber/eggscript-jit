use anyhow::{Context, Result};
use eggscript_interpreter::Instruction;
use std::collections::HashMap;

use crate::{MIRInfo, Unit, ValueHandle, ValueStore, MIR};

pub struct EggscriptLowerContext {
	allocations: Vec<ValueHandle>,
	value_id_to_stack: HashMap<ValueHandle, usize>,
	value_store: ValueStore,
}

impl EggscriptLowerContext {
	pub fn new(value_store: ValueStore) -> EggscriptLowerContext {
		EggscriptLowerContext {
			allocations: vec![],
			value_id_to_stack: HashMap::new(),
			value_store,
		}
	}

	pub fn lower_units(&mut self, units: Vec<Unit>) -> Result<Vec<Instruction>> {
		let mut instructions = vec![];
		for unit in units.iter() {
			instructions.append(&mut self.lower_unit(unit)?);
		}

		instructions.insert(0, Instruction::Reserve(self.allocations.len()));

		Ok(instructions)
	}

	fn lower_unit(&mut self, unit: &Unit) -> Result<Vec<Instruction>> {
		let mut instructions = vec![];
		for mir in unit.mir.iter() {
			instructions.append(&mut self.lower_mir(mir)?);
		}

		Ok(instructions)
	}

	fn lower_mir(&mut self, mir: &MIR) -> Result<Vec<Instruction>> {
		match &mir.info {
			MIRInfo::Allocate(value) => {
				self.allocations.push(*value);
				self.value_id_to_stack
					.insert(*value, self.value_id_to_stack.len());
				Ok(vec![])
			}
			MIRInfo::StoreLiteral(lvalue, rvalue) => {
				let stack_index = self
					.value_id_to_stack
					.get(lvalue)
					.context(format!("Value {} has not been allocated to stack", lvalue))?;

				Ok(vec![
					Instruction::Push(rvalue.into()),
					Instruction::Store(*stack_index, -1),
				])
			}
		}
	}
}
