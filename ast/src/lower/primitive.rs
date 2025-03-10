use anyhow::{Context, Result};
use eggscript_mir::{PrimitiveValue, UnitHandle, Value};
use eggscript_types::P;

use crate::expressions::{Expression, ExpressionInfo};
use crate::lower::AstLowerContext;

impl AstLowerContext {
	pub(crate) fn lower_primitive(
		&mut self,
		expression: &Expression,
	) -> Result<(Vec<UnitHandle>, Option<P<Value>>)> {
		let ExpressionInfo::Primitive(ty, value) = &expression.info else {
			unreachable!();
		};

		let value = match ty {
			eggscript_types::Primitive::Number => self.value_store.new_primitive(
				expression.ty.expect("Could not get type"),
				PrimitiveValue::Number(
					value
						.trim()
						.parse::<f64>()
						.context(format!("Could not parse f64 '{}'", value))?,
				),
			),
			_ => todo!(),
		};

		Ok((vec![], Some(value)))
	}
}
