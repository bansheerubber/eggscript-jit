use anyhow::{Context, Result};
use eggscript_types::P;
use pest::iterators::Pair;

use crate::expressions::{Expression, ExpressionInfo};
use crate::parser::Rule;
use crate::{AstContext, BinaryOperator, Ident};

impl Expression {
	pub(crate) fn parse_variable_assignment(
		context: &mut AstContext,
		pair: Pair<Rule>,
	) -> Result<P<Expression>> {
		let span = pair.as_span().into();

		let mut inner = pair.into_inner();
		let variable_ident = Ident::new(
			inner
				.next()
				.context("Could not get variable name")?
				.as_str(),
			span,
		);

		let operator = BinaryOperator::parse_assignment(
			inner
				.next()
				.context("Could not get assignment operator")?
				.as_str(),
		)
		.context("Could not parse assignment operator")?;

		let expression = Expression::parse_pair(
			context,
			inner.next().context("Could not get rvalue expression")?,
		)
		.context("Could not parse pair")??;

		Ok(P::new(Expression {
			ty: expression.ty,
			info: ExpressionInfo::Assign(variable_ident, operator, expression),
			span,
		}))
	}
}
