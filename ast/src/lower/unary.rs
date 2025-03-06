use anyhow::Result;
use eggscript_mir::{MIRInfo, Transition, UnitHandle, Value, MIR};
use eggscript_types::P;

use crate::expressions::{Expression, ExpressionInfo};
use crate::lower::AstLowerContext;

impl AstLowerContext {
	pub(crate) fn lower_unary(
		&mut self,
		expression: &P<Expression>,
	) -> Result<(Vec<UnitHandle>, Option<P<Value>>)> {
		let ExpressionInfo::UnaryOperation(value, operator) = &expression.info else {
			unreachable!();
		};

		let (mut value_units, value) = self.lower_expression(value)?;
		let value = value.unwrap();

		let result = self.value_store.new_temp(value.ty());

		let mut units = vec![];
		units.append(&mut value_units);
		units.push(self.unit_store.new_unit(
			vec![MIR::new(
				MIRInfo::Unary(result.clone(), value.clone(), operator.into()),
				expression.span,
			)],
			Transition::Next,
		));

		Ok((units, Some(result)))
	}
}
