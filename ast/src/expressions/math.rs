use std::cell::RefCell;
use std::rc::Rc;

use anyhow::{Context, Result};
use eggscript_types::P;
use pest::iterators::Pair;

use crate::expressions::Expression;
use crate::operators::LogicOperator;
use crate::parser::{configure_pratt, Rule};
use crate::{AstContext, BinaryOperator, Span, UnaryOperator};

use super::ExpressionInfo;

impl Expression {
	pub(crate) fn parse_math(context: &mut AstContext, pair: Pair<Rule>) -> Result<P<Expression>> {
		let context = Rc::new(RefCell::new(context));
		let result = configure_pratt()
			.map_primary(|primary| {
				Expression::parse_pair(*(context.borrow_mut()), primary)
					.context("Could not parse pair")?
			})
			.map_prefix(|op, value| {
				let value = value?;
				Ok(P::new(Expression {
					span: value.span.clone(),
					info: ExpressionInfo::UnaryOperation(
						value,
						UnaryOperator::parse_unary(op.as_str())
							.context("Could not parse unary operator")?,
					),
					ty: None,
				}))
			})
			.map_infix(|lhs, op, rhs| {
				let lhs = lhs?;
				let rhs = rhs?;

				let ty = if lhs.ty == rhs.ty {
					lhs.ty
				} else {
					Some(
						context
							.borrow()
							.type_store
							.lock()
							.expect("Could not lock type store")
							.create_unknown(),
					)
				};

				if let Some(operator) = BinaryOperator::parse_binary(op.as_str()) {
					Ok(P::new(Expression {
						span: Span::new(lhs.span.start(), rhs.span.end()),
						info: ExpressionInfo::BinaryOperation(lhs, rhs, operator),
						ty,
					}))
				} else if let Some(operator) = LogicOperator::parse_logic(op.as_str()) {
					Ok(P::new(Expression {
						span: Span::new(lhs.span.start(), rhs.span.end()),
						info: ExpressionInfo::LogicOperation(lhs, rhs, operator),
						ty,
					}))
				} else {
					unreachable!()
				}
			})
			.parse(pair.into_inner());

		return result;
	}
}
