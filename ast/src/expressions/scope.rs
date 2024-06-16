use anyhow::Result;
use eggscript_types::P;

use crate::expressions::{Block, Expression, ExpressionInfo};
use crate::Span;

impl Expression {
	pub fn new_scope<T: IntoIterator<Item = Result<P<Expression>>>>(
		expressions: T,
		span: Span,
	) -> Result<P<Expression>> {
		let expressions = expressions
			.into_iter()
			.collect::<Result<Vec<P<Expression>>>>()?;

		Ok(P::new(Expression {
			info: ExpressionInfo::Scope(P::new(Block { expressions, span })),
			span,
			ty: None,
		}))
	}
}
