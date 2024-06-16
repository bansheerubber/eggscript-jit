use anyhow::Result;
use eggscript_types::{TypeHandle, P};
use pest::iterators::{Pair, Pairs};

use crate::expressions::Block;
use crate::parser::Rule;
use crate::{AstContext, BinaryOperator, Ident, Span};

#[derive(Debug)]
pub struct Expression {
	pub(crate) info: ExpressionInfo,
	pub(crate) span: Span,
	pub(crate) ty: Option<TypeHandle>,
}

#[derive(Debug)]
pub enum ExpressionInfo {
	/// Assigns the resulting value of an expression to a variable.
	Assign(Ident, BinaryOperator, P<Expression>),
	/// A literal value.
	Primitive(String),
	/// Represents variable scope.
	Scope(P<Block>),
}

impl Expression {
	pub(crate) fn parse_program(pairs: Pairs<Rule>) -> Result<P<Expression>> {
		let mut context = AstContext::new();
		let expressions = pairs.filter_map(|pair| Expression::parse_pair(&mut context, pair));
		Expression::new_scope(expressions, Span::new(0, 0))
	}

	pub(crate) fn parse_pair(
		context: &mut AstContext,
		pair: Pair<Rule>,
	) -> Option<Result<P<Expression>>> {
		match pair.as_rule() {
			Rule::number_literal => Some(Expression::parse_number_literal(context, pair)),
			Rule::string_literal => Some(Expression::parse_string_literal(context, pair)),
			Rule::variable_assignment => Some(Expression::parse_variable_assignment(context, pair)),
			Rule::EOI => None,
			rule => unimplemented!("{:?} not implemented", rule),
		}
	}
}
