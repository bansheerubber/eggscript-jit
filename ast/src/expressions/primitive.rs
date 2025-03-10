use anyhow::{Context, Result};
use eggscript_types::P;
use pest::iterators::Pair;

use crate::expressions::{Expression, ExpressionInfo};
use crate::parser::Rule;
use crate::AstContext;

impl Expression {
	pub(crate) fn parse_number_literal(
		context: &mut AstContext,
		pair: Pair<Rule>,
	) -> Result<P<Expression>> {
		let type_handle = context
			.type_store
			.lock()
			.unwrap()
			.name_to_type_handle("number")
			.context("Could not get 'number' literal type")?;

		let symbol = pair.as_str().to_string();
		Ok(P::new(Expression {
			info: ExpressionInfo::Primitive(eggscript_types::Primitive::Number, symbol),
			span: pair.as_span().into(),
			ty: Some(type_handle),
		}))
	}
}
