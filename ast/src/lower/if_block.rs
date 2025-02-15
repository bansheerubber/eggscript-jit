use anyhow::{Context, Result};
use eggscript_mir::{Transition, UnitHandle, Value};
use eggscript_types::P;
use std::ops::Deref;

use crate::expressions::{Block, Expression, ExpressionInfo};
use crate::lower::AstLowerContext;

impl AstLowerContext {
	pub(crate) fn lower_if_block(
		&mut self,
		expression: &P<Expression>,
	) -> Result<(Vec<UnitHandle>, Option<P<Value>>)> {
		let mut units = vec![];

		let unit_after = self.unit_store.new_unit(vec![], Transition::Next);

		let mut last_jump_unit = 0;
		let mut last_conditional_value = None;

		let mut next = Some(expression.clone());
		while let Some(expression) = next.as_ref() {
			match &expression.deref().info {
				ExpressionInfo::If(conditional, code, new_next) => {
					let (mut if_units, conditional_value, conditional_unit_start, jump_unit) =
						self.lower_if_block_impl(conditional, code, unit_after)?;

					if last_jump_unit != 0 {
						self.unit_store.set_transition(
							last_jump_unit,
							Transition::GotoIfFalse(
								conditional_unit_start,
								last_conditional_value
									.clone()
									.context("Could not get conditional value")?,
							),
						)?;
					}

					last_jump_unit = jump_unit;
					last_conditional_value = conditional_value;

					units.append(&mut if_units);
					next = new_next.clone();
				}
				ExpressionInfo::Else(code) => {
					let (mut else_units, else_units_start) =
						self.lower_else_block_impl(code, unit_after)?;

					if last_jump_unit != 0 {
						self.unit_store.set_transition(
							last_jump_unit,
							Transition::GotoIfFalse(
								else_units_start,
								last_conditional_value
									.clone()
									.context("Could not get conditional value")?,
							),
						)?;
					}

					units.append(&mut else_units);
					next = None;
				}
				_ => unreachable!(),
			}
		}

		units.push(unit_after);

		Ok((units, None))
	}

	fn lower_if_block_impl(
		&mut self,
		conditional: &P<Expression>,
		code: &P<Block>,
		unit_after: UnitHandle,
	) -> Result<(Vec<UnitHandle>, Option<P<Value>>, UnitHandle, UnitHandle)> {
		let (mut conditional_units, conditional_value) = self.lower_expression(conditional)?;
		let (mut block_units, _) = self.lower_block(code).context("Could not lower block")?;

		let conditional_units_start = conditional_units[0];

		let mut units = vec![];
		units.append(&mut conditional_units);

		let jump_unit = self.unit_store.new_unit(
			vec![],
			Transition::GotoIfFalse(
				unit_after,
				conditional_value
					.clone()
					.context("Could not get conditional value")?,
			),
		);

		units.push(jump_unit);

		units.append(&mut block_units);
		units.push(
			self.unit_store
				.new_unit(vec![], Transition::Goto(unit_after)),
		);

		Ok((units, conditional_value, conditional_units_start, jump_unit))
	}

	fn lower_else_block_impl(
		&mut self,
		code: &P<Block>,
		unit_after: UnitHandle,
	) -> Result<(Vec<UnitHandle>, UnitHandle)> {
		let (mut block_units, _) = self.lower_block(code).context("Could not lower block")?;

		let else_units_start = block_units[0];

		let mut units = vec![];
		units.append(&mut block_units);
		units.push(
			self.unit_store
				.new_unit(vec![], Transition::Goto(unit_after)),
		);

		Ok((units, else_units_start))
	}
}
