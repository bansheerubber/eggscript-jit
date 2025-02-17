use anyhow::{Context, Result};
use eggscript_types::{Type, P};
use pest::iterators::Pair;

use crate::expressions::{Expression, ExpressionInfo};
use crate::parser::Rule;
use crate::{AstContext, Ident};

impl Expression {
	pub(crate) fn parse_function_call(
		context: &mut AstContext,
		pair: Pair<Rule>,
	) -> Result<P<Expression>> {
		let span = pair.as_span().into();
		let mut pairs = pair.into_inner();

		let name_pair = pairs.next().context("Could not get function name")?;

		let name = name_pair.as_str();
		let name_span = name_pair.as_span().into();

		let mut arguments = vec![];

		let argument_pairs = pairs.next().context("Could not get function args")?;
		for argument_pair in argument_pairs.into_inner().into_iter() {
			arguments.push(
				Expression::parse_pair(context, argument_pair)
					.context("Could not parse pair")??,
			);
		}

		let ty = context
			.type_store
			.lock()
			.unwrap()
			.create_type(Type::FunctionReturn {
				id: 0,
				function_name: name.into(),
			});

		Ok(P::new(Expression {
			info: ExpressionInfo::FunctionCall(Ident::new(name, name_span), arguments),
			span,
			ty: Some(ty),
		}))
	}
}
