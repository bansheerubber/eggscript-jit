use std::ops::Deref;

use anyhow::Result;
use eggscript_mir::{MIRInfo, Transition, UnitHandle, Value, MIR};
use eggscript_types::P;

use crate::expressions::{Expression, ExpressionInfo};
use crate::lower::AstLowerContext;

impl AstLowerContext {
	pub(crate) fn lower_return_statement(
		&mut self,
		expression: &P<Expression>,
	) -> Result<(Vec<UnitHandle>, Option<P<Value>>)> {
		let ExpressionInfo::Return(value) = &expression.info else {
			unreachable!();
		};

		let mut units = vec![];
		let value = if let Some(value) = value {
			let (mut value_units, value) = self.lower_expression(value)?;
			units.append(&mut value_units);

			match value.as_ref().unwrap().deref() {
				Value::Location { ty, .. } => {
					let temp = self.value_store.new_temp(*ty);
					units.push(self.unit_store.new_unit(
						vec![MIR::new(MIRInfo::StoreValue(
							temp.clone(),
							value.clone().unwrap(),
						))],
						Transition::Next,
					));
				}
				Value::Primitive { ty, value, .. } => {
					let temp = self.value_store.new_temp(*ty);
					units.push(self.unit_store.new_unit(
						vec![MIR::new(MIRInfo::StoreLiteral(temp.clone(), value.clone()))],
						Transition::Next,
					));
				}
				Value::Temp { .. } => {}
			}

			value
		} else {
			None
		};

		units.push(
			self.unit_store
				.new_unit(vec![], Transition::Return(value.clone())),
		);

		Ok((units, value))
	}
}
