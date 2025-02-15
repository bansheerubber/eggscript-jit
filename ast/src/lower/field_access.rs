use anyhow::{Context, Result};
use eggscript_mir::{UnitHandle, Value};
use eggscript_types::P;

use crate::expressions::{Expression, ExpressionInfo};
use crate::lower::AstLowerContext;

impl AstLowerContext {
	pub(crate) fn lower_field_access(
		&mut self,
		expression: &P<Expression>,
	) -> Result<(Vec<UnitHandle>, Option<P<Value>>)> {
		let ExpressionInfo::FieldAccess(name) = &expression.info else {
			unreachable!();
		};

		let (variable_value, is_new) = self.value_store.new_location(
			name.name(),
			expression
				.ty
				.context("Variable assignment does not have type")?,
		);

		assert!(!is_new, "Undefined variable access {}", name.name());

		Ok((vec![], Some(variable_value)))
	}
}
