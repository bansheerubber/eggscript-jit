use anyhow::{Context, Result};
use eggscript_types::{TypeHandle, TypeStore, P};
use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::expressions::Block;
use crate::parser::{Program, Rule};
use crate::{AstContext, BinaryOperator, Ident, Span, UnaryOperator};

use super::FunctionArgument;

#[derive(Clone, Debug)]
pub struct Expression {
	pub(crate) info: ExpressionInfo,
	pub(crate) span: Span,
	pub(crate) ty: Option<TypeHandle>,
}

#[derive(Clone, Debug)]
pub enum ExpressionInfo {
	/// Assigns the resulting value of an expression to a variable.
	Assign(Ident, BinaryOperator, P<Expression>),
	/// Operation between two expressions
	BinaryOperation(P<Expression>, P<Expression>, BinaryOperator),
	/// Else block that follows if/else-if blocks
	Else(P<Block>),
	/// Acessing a variable
	FieldAccess(Ident),
	/// For loop
	For(P<Expression>, P<Expression>, P<Expression>, P<Block>),
	/// Function call
	FunctionCall(Ident, Vec<P<Expression>>),
	/// If or else-if block, with optional continuing else-if/else block
	If(P<Expression>, P<Block>, Option<P<Expression>>),
	/// A literal value.
	Primitive(eggscript_types::Primitive, String),
	/// Return statement
	Return(Option<P<Expression>>),
	/// Represents variable scope.
	Scope(P<Block>),
	/// Operation performed on a single expression
	UnaryOperation(P<Expression>, UnaryOperator),
	/// While loop
	While(P<Expression>, P<Block>),
}

impl Expression {
	pub(crate) fn parse_program(
		file_name: &str,
		type_store: Arc<Mutex<TypeStore>>,
		pairs: Pairs<Rule>,
	) -> Result<Program> {
		let mut context = AstContext::new(type_store.clone());

		let mut functions = vec![];
		let mut function_name_to_function = HashMap::new();
		let mut global_scope = vec![];
		for pair in pairs.into_iter() {
			match pair.as_rule() {
				Rule::function_declaration => {
					let id = functions.len();
					let function = Expression::parse_function_declaration(&mut context, pair, id)
						.context("Could not parse function declaration")?;

					functions.push(function.clone());
					function_name_to_function.insert(function.name.clone(), function);
				}
				_ => {
					if let Some(parsed) = Expression::parse_pair(&mut context, pair) {
						global_scope.push(parsed);
					}
				}
			}
		}

		let mut program = Program {
			file_name: file_name.to_string(),
			function_name_to_function,
			functions,
			global_scope: Expression::new_scope(global_scope, Span::new(0, 0))?,
			type_store,
		};

		let type_store = context.type_store.lock().unwrap();
		let number = type_store.name_to_type_handle("number");
		drop(type_store);

		program.add_native_function(
			vec![FunctionArgument {
				name: "value".into(),
				span: Span::new(0, 0),
				ty: number,
			}],
			"printNumber",
			number.unwrap(),
		);

		Ok(program)
	}

	pub(crate) fn parse_pair(
		context: &mut AstContext,
		pair: Pair<Rule>,
	) -> Option<Result<P<Expression>>> {
		match pair.as_rule() {
			Rule::number_literal => Some(Expression::parse_number_literal(context, pair)),
			Rule::else_block => Some(Expression::parse_else_block(context, pair)),
			Rule::field_access => Some(Expression::parse_field_access(context, pair)),
			Rule::for_block => Some(Expression::parse_for_block(context, pair)),
			Rule::function_call => Some(Expression::parse_function_call(context, pair)),
			Rule::if_block => Some(Expression::parse_if_block(context, pair)),
			Rule::math => Some(Expression::parse_math(context, pair)),
			Rule::return_statement => Some(Expression::parse_return_statement(context, pair)),
			Rule::string_literal => Some(Expression::parse_string_literal(context, pair)),
			Rule::variable_assignment => Some(Expression::parse_variable_assignment(context, pair)),
			Rule::variable_declaration => {
				Some(Expression::parse_variable_declaration(context, pair))
			}
			Rule::while_block => Some(Expression::parse_while_block(context, pair)),
			Rule::EOI => None,
			rule => unimplemented!("{:?} not implemented", rule),
		}
	}
}
