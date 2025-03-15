use anyhow::{Context, Result};
use eggscript_mir::{MIRInfo, PrimitiveValue, Transition, UnitHandle, Value, MIR};
use eggscript_types::P;

use crate::expressions::{Expression, ExpressionInfo};
use crate::lower::AstLowerContext;
use crate::operators::LogicOperator;

use super::context::Logic;

impl AstLowerContext {
	pub(crate) fn lower_logic_operation(
		&mut self,
		expression: &P<Expression>,
	) -> Result<(Vec<UnitHandle>, Option<P<Value>>)> {
		let ExpressionInfo::LogicOperation(left, right, operator) = &expression.info else {
			unreachable!();
		};

		let is_base_logic = if self.logic_stack.len() == 0
			|| self
				.logic_stack
				.last()
				.expect("Could not get last logic")
				.operator != *operator
		{
			let short_circuit_unit = self.unit_store.new_unit(vec![], Transition::Next);
			self.logic_stack.push(Logic {
				operator: *operator,
				short_circuit_unit,
				units_jumping_to_phi: vec![],
			});

			true
		} else {
			false
		};

		let (mut left_units, left_value) = self.lower_expression(left)?;
		let (mut right_units, right_value) = self.lower_expression(right)?;

		let logic = self
			.logic_stack
			.last_mut()
			.expect("Could not get last on logic stack");

		let left_value = left_value.context("Could not find left value")?;
		let right_value = right_value.context("Could not find right value")?;

		let mut units = vec![];
		units.append(&mut left_units);

		if !left.info.is_logic_operation() || &left.info.get_logic_operator().unwrap() != operator {
			if operator == &LogicOperator::And {
				let left_goto_unit = self.unit_store.new_unit(
					vec![],
					Transition::GotoIfFalse(logic.short_circuit_unit, left_value),
				);

				logic.units_jumping_to_phi.push((
					left_goto_unit,
					self.value_store
						.new_primitive(0, PrimitiveValue::Number(0.0)),
				));

				units.push(left_goto_unit);
			} else {
				let left_goto_unit = self.unit_store.new_unit(
					vec![],
					Transition::GotoIfTrue(logic.short_circuit_unit, left_value.clone()),
				);

				logic
					.units_jumping_to_phi
					.push((left_goto_unit, left_value));

				units.push(left_goto_unit);
			}
		}

		units.append(&mut right_units);

		if !right.info.is_logic_operation() || &right.info.get_logic_operator().unwrap() != operator
		{
			if operator == &LogicOperator::And {
				let right_goto_unit = self.unit_store.new_unit(
					vec![],
					Transition::GotoIfFalse(logic.short_circuit_unit, right_value.clone()),
				);

				logic.units_jumping_to_phi.push((
					right_goto_unit,
					self.value_store
						.new_primitive(0, PrimitiveValue::Number(0.0)),
				));

				units.push(right_goto_unit);
			} else {
				let right_goto_unit = self.unit_store.new_unit(
					vec![],
					Transition::GotoIfTrue(logic.short_circuit_unit, right_value.clone()),
				);

				logic
					.units_jumping_to_phi
					.push((right_goto_unit, right_value.clone()));

				units.push(right_goto_unit);
			}
		}

		let result = if is_base_logic {
			let next_unit = self.unit_store.new_unit(vec![], Transition::Next);
			units.push(next_unit);
			units.push(logic.short_circuit_unit);

			let result = self.value_store.new_temp(
				self.program
					.type_store
					.lock()
					.expect("Could not lock type store")
					.name_to_type_handle("number")
					.expect("Could not find 'number' type"),
			);

			if operator == &LogicOperator::And {
				logic
					.units_jumping_to_phi
					.push((next_unit, right_value.clone()));
			} else {
				logic.units_jumping_to_phi.push((
					next_unit,
					self.value_store
						.new_primitive(0, PrimitiveValue::Number(0.0)),
				));
			}

			self.unit_store
				.get_unit_mut(&logic.short_circuit_unit)
				.expect("Could not find unit")
				.add_mir(vec![MIR::new(
					MIRInfo::LogicPhi(
						result.clone(),
						eggscript_mir::LogicOperator::And,
						logic.units_jumping_to_phi.clone(),
					),
					expression.span,
				)]);

			self.logic_stack.pop();

			result
		} else {
			right_value
		};

		Ok((units, Some(result)))
	}
}
