use anyhow::{Context, Result};
use eggscript_mir::{Transition, UnitHandle, Value};
use eggscript_types::P;

use crate::expressions::{Expression, ExpressionInfo};
use crate::lower::AstLowerContext;

impl AstLowerContext {
	pub(crate) fn lower_while_block(
		&mut self,
		expression: &P<Expression>,
	) -> Result<(Vec<UnitHandle>, Option<P<Value>>)> {
		let ExpressionInfo::While(conditional, block) = &expression.info else {
			unreachable!();
		};

		let (mut conditional_units, conditional_value) = self.lower_expression(conditional)?;
		let first_conditional_unit = *conditional_units.iter().nth(0).unwrap();

		let (mut block_units, _) = self.lower_block(block)?;

		let unit_after = self.unit_store.new_unit(vec![], Transition::Next);

		let mut units = vec![];
		units.append(&mut conditional_units);
		units.push(self.unit_store.new_unit(
			vec![],
			Transition::GotoIfFalse(
				unit_after,
				conditional_value.context("Could not unwrap conditional units")?,
			),
		));
		units.append(&mut block_units);
		units.push(
			self.unit_store
				.new_unit(vec![], Transition::Goto(first_conditional_unit)),
		);
		units.push(unit_after);

		Ok((units, None))
	}
}
