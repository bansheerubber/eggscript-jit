use anyhow::{Context, Result};
use eggscript_mir::{MIRInfo, Transition, UnitHandle, Value, MIR};
use eggscript_types::P;

use crate::expressions::{Expression, ExpressionInfo};
use crate::lower::AstLowerContext;

impl AstLowerContext {
	pub(crate) fn lower_binary_operation(
		&mut self,
		expression: &P<Expression>,
	) -> Result<(Vec<UnitHandle>, Option<P<Value>>)> {
		let ExpressionInfo::BinaryOperation(left, right, operator) = &expression.info else {
			unreachable!();
		};

		let (mut left_units, left_value) = self.lower_expression(left)?;
		let (mut right_units, right_value) = self.lower_expression(right)?;

		let left_value = left_value.context("Could not find left value")?;
		let right_value = right_value.context("Could not find right value")?;

		let ty = right_value.ty();

		let result = self.value_store.new_temp(ty); // TODO fill out type handle correctly

		let unit = self.unit_store.new_unit(
			vec![MIR::new(MIRInfo::BinaryOperation(
				result.clone(),
				left_value,
				right_value,
				operator.into(),
			))],
			Transition::Next,
		);

		let mut units = vec![];
		units.append(&mut left_units);
		units.append(&mut right_units);
		units.push(unit);

		Ok((units, Some(result)))
	}
}
