use anyhow::Result;
use eggscript_types::P;
use pest::iterators::Pair;

use crate::expressions::{Expression, ExpressionInfo};
use crate::parser::Rule;
use crate::{AstContext, Ident};

impl Expression {
	pub(crate) fn parse_field_access(
		context: &mut AstContext,
		pair: Pair<Rule>,
	) -> Result<P<Expression>> {
		let span = pair.as_span().into();
		let symbol = pair.as_str().to_string();
		let symbol = symbol.trim();

		let type_handle = context
			.type_store
			.lock()
			.expect("Could not lock type store")
			.name_to_type_handle("number")
			.expect("Could not get 'number' literal type");

		let variable_ident = Ident::new(&symbol, span);

		Ok(P::new(Expression {
			info: ExpressionInfo::FieldAccess(variable_ident),
			span: pair.as_span().into(),
			ty: Some(type_handle),
		}))
	}
}
