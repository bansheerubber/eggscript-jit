use anyhow::{Context, Result};
use eggscript_types::P;
use pest::iterators::Pair;

use crate::expressions::{Expression, ExpressionInfo};
use crate::parser::Rule;
use crate::AstContext;

impl Expression {
	pub(crate) fn parse_return_statement(
		context: &mut AstContext,
		pair: Pair<Rule>,
	) -> Result<P<Expression>> {
		let span = pair.as_span().into();
		let mut pairs = pair.into_inner();

		let value = if let Some(pair) = pairs.next() {
			Some(
				Expression::parse_pair(context, pair)
					.context("Could not parse return statement pair")??,
			)
		} else {
			None
		};

		Ok(P::new(Expression {
			info: ExpressionInfo::Return(value),
			span,
			ty: None,
		}))
	}
}
