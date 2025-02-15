use anyhow::{Context, Result};
use eggscript_types::P;
use pest::iterators::Pair;

use crate::expressions::{Expression, ExpressionInfo};
use crate::parser::Rule;
use crate::{AstContext, BinaryOperator, Ident};

impl Expression {
	pub(crate) fn parse_variable_declaration(
		context: &mut AstContext,
		pair: Pair<Rule>,
	) -> Result<P<Expression>> {
		let span = pair.as_span().into();
		let mut inner = pair.into_inner();
		let mut symbol = inner
			.next()
			.context("Could not get variable name")?
			.as_str();
		symbol = symbol.trim();

		let ty = if let Rule::type_ident = inner
			.peek()
			.context("Could not peek variable type")?
			.as_rule()
		{
			let type_pair = inner.next().context("Could not get variable type")?;
			context
				.type_store
				.name_to_type_handle(type_pair.as_str())
				.context("Could not unwrap variable type")?
		} else {
			context.type_store.create_unknown()
		};

		let variable_ident = Ident::new(symbol, span);

		let expression = Expression::parse_pair(
			context,
			inner.next().context("Could not get rvalue expression")?,
		)
		.context("Could not parse pair")??;

		Ok(P::new(Expression {
			info: ExpressionInfo::Assign(variable_ident, BinaryOperator::Equal, expression),
			span,
			ty: Some(ty),
		}))
	}
}
