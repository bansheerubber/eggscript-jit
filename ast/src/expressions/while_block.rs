use anyhow::{Context, Result};
use eggscript_types::P;
use pest::iterators::Pair;

use crate::expressions::{Expression, ExpressionInfo};
use crate::parser::Rule;
use crate::AstContext;

use super::Block;

impl Expression {
	pub(crate) fn parse_while_block(
		context: &mut AstContext,
		pair: Pair<Rule>,
	) -> Result<P<Expression>> {
		let span = pair.as_span().into();
		let mut pairs = pair.into_inner();

		let conditional = Expression::parse_pair(context, pairs.next().unwrap())
			.context("Could not parse pair")??;

		let block = pairs.next().context("Could not get next pair")?;

		let expressions = block
			.into_inner()
			.map(|p| {
				Expression::parse_pair(context, p)
					.context("Could not parse pair")
					.unwrap()
					.unwrap()
			})
			.collect::<Vec<P<Expression>>>()
			.try_into()
			.unwrap();

		let block = P::new(Block { expressions, span });

		Ok(P::new(Expression {
			info: ExpressionInfo::While(conditional, block),
			span,
			ty: None,
		}))
	}
}
