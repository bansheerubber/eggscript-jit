use anyhow::Result;
use eggscript_mir::{UnitHandle, Value};
use eggscript_types::P;

use crate::expressions::{Expression, ExpressionInfo};
use crate::lower::AstLowerContext;

impl AstLowerContext {
	pub(crate) fn lower_scope(
		&mut self,
		scope: &P<Expression>,
	) -> Result<(Vec<UnitHandle>, Option<P<Value>>)> {
		let ExpressionInfo::Scope(block) = &scope.info else {
			unreachable!();
		};

		let mut unit_handles = vec![];
		for expression in block.expressions() {
			let (mut unit_handles2, _) = self.lower_expression(expression)?;
			unit_handles.append(&mut unit_handles2);
		}

		Ok((unit_handles, None))
	}
}
