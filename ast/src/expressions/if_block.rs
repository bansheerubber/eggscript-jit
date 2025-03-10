use anyhow::Result;
use eggscript_types::P;
use pest::iterators::Pair;

use crate::expressions::{Expression, ExpressionInfo};
use crate::parser::Rule;
use crate::AstContext;

use super::Block;

impl Expression {
	pub(crate) fn parse_if_block(
		context: &mut AstContext,
		pair: Pair<Rule>,
	) -> Result<P<Expression>> {
		let span = pair.as_span().into();
		let mut pairs = pair.into_inner();

		let conditional = Expression::parse_pair(
			context,
			pairs
				.next()
				.expect("Expected next pair where there is none"),
		)
		.expect("Expected expression where there is none")?;

		let block = pairs
			.next()
			.expect("Expected next pair where there is none");

		let expressions = block
			.into_inner()
			.map(|p| {
				Expression::parse_pair(context, p).expect("Expected expression where there is none")
			})
			.collect::<Result<Vec<P<Expression>>>>()?;

		let block = P::new(Block { expressions, span });

		let next = if let Some(pair) = pairs.next() {
			// parse else-if-else chain
			Some(match pair.as_rule() {
				Rule::else_if_block => Expression::parse_if_block(context, pair)?,
				Rule::else_block => Expression::parse_else_block(context, pair)?,
				_ => panic!(
					"expected else_if_statement or else_statement, not {:?}",
					pair.as_rule()
				),
			})
		} else {
			None
		};

		Ok(P::new(Expression {
			info: ExpressionInfo::If(conditional, block, next),
			span,
			ty: None,
		}))
	}
}
