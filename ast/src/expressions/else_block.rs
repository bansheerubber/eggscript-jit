use anyhow::Result;
use eggscript_types::P;
use pest::iterators::Pair;

use crate::expressions::{Expression, ExpressionInfo};
use crate::parser::Rule;
use crate::AstContext;

use super::Block;

impl Expression {
	pub(crate) fn parse_else_block(
		context: &mut AstContext,
		pair: Pair<Rule>,
	) -> Result<P<Expression>> {
		let span = pair.as_span().into();
		let mut pairs = pair.into_inner();

		let block = pairs.next().expect("Could not get next pair");

		let expressions = block
			.into_inner()
			.map(|p| {
				Expression::parse_pair(context, p).expect("Expected expression where there is none")
			})
			.collect::<Result<Vec<P<Expression>>>>()?;

		let block = P::new(Block { expressions, span });

		Ok(P::new(Expression {
			info: ExpressionInfo::Else(block),
			span,
			ty: None,
		}))
	}
}
