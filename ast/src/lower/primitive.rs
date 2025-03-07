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
			eggscript_types::Primitive::Double => self.value_store.new_primitive(
				expression.ty.context("Could not unwrap type")?,
				PrimitiveValue::Double(
					value
						.trim()
						.parse::<f64>()
						.context(format!("Could not parse f64 '{}'", value))?,
				),
			),
			eggscript_types::Primitive::I64 => self.value_store.new_primitive(
				expression.ty.context("Could not unwrap type")?,
				PrimitiveValue::Integer(
					value
						.trim()
						.parse::<i64>()
						.context(format!("Could not parse i64 '{}'", value))?,
				),
			),
			eggscript_types::Primitive::String => todo!(),
			_ => todo!(),
		};

		Ok((vec![], Some(value)))
	}
}
