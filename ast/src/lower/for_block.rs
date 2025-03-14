use anyhow::Result;
use eggscript_mir::{Transition, UnitHandle, Value};
use eggscript_types::P;

use crate::expressions::{Expression, ExpressionInfo};
use crate::lower::AstLowerContext;

impl AstLowerContext {
	pub(crate) fn lower_for_block(
		&mut self,
		expression: &P<Expression>,
	) -> Result<(Vec<UnitHandle>, Option<P<Value>>)> {
		let ExpressionInfo::For(declaration, conditional, update, block) = &expression.info else {
			unreachable!();
		};

		let (mut declaration_units, _) = self.lower_expression(declaration)?;

		let (mut conditional_units, conditional_value) = self.lower_expression(conditional)?;
		let first_conditional_unit = *conditional_units
			.iter()
			.nth(0)
			.expect("Could not get first conditional unit");

		let (mut update_units, _) = self.lower_expression(update)?;

		let (mut block_units, _) = self.lower_block(block)?;

		let unit_after = self.unit_store.new_unit(vec![], Transition::Next);

		let mut units = vec![];
		units.append(&mut declaration_units);
		units.append(&mut conditional_units);
		units.push(self.unit_store.new_unit(
			vec![],
			Transition::GotoIfFalse(
				unit_after,
				conditional_value.expect("Expected conditional value where there is none"),
			),
		));

		units.append(&mut block_units);
		units.append(&mut update_units);
		units.push(
			self.unit_store
				.new_unit(vec![], Transition::Goto(first_conditional_unit)),
		);
		units.push(unit_after);

		Ok((units, None))
	}
}
