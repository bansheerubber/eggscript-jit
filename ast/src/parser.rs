use anyhow::{Context, Result};
use eggscript_types::{TypeHandle, P};
use pest::iterators::Pairs;
use pest::pratt_parser::{Assoc, Op, PrattParser};
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

use crate::expressions::Expression;
use crate::{Function, FunctionArgument, Span};

pub fn configure_pratt() -> PrattParser<Rule> {
	PrattParser::new()
		.op(Op::infix(Rule::logical_or, Assoc::Left))
		.op(Op::infix(Rule::logical_and, Assoc::Left))
		.op(Op::infix(Rule::bitwise_and, Assoc::Left)
			| Op::infix(Rule::bitwise_or, Assoc::Left)
			| Op::infix(Rule::bitwise_xor, Assoc::Left))
		.op(Op::infix(Rule::equals, Assoc::Left) | Op::infix(Rule::not_equals, Assoc::Left))
		.op(Op::infix(Rule::less_than_equal_to, Assoc::Left)
			| Op::infix(Rule::greater_than_equal_to, Assoc::Left))
		.op(Op::infix(Rule::less_than, Assoc::Left) | Op::infix(Rule::greater_than, Assoc::Left))
		.op(Op::infix(Rule::addition, Assoc::Left) | Op::infix(Rule::subtraction, Assoc::Left))
		.op(Op::infix(Rule::multiplication, Assoc::Left) | Op::infix(Rule::division, Assoc::Left))
		.op(Op::prefix(Rule::negative)
			| Op::prefix(Rule::logical_not)
			| Op::prefix(Rule::bitwise_not))
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub(crate) struct PestParser<'a> {
	#[allow(dead_code)]
	pub(crate) pairs: Pairs<'a, Rule>,
	#[allow(dead_code)]
	pub(crate) pratt: PrattParser<Rule>,
}

pub struct Program {
	pub function_name_to_function: HashMap<String, P<Function>>,
	pub functions: Vec<P<Function>>,
	pub global_scope: P<Expression>,
}

impl Program {
	pub fn add_native_function(
		&mut self,
		arguments: Vec<FunctionArgument>,
		name: &str,
		return_type: TypeHandle,
	) {
		let id = self.functions.len();
		let function = P::new(Function {
			arguments,
			id,
			name: name.to_string(),
			scope: None,
			span: Span::new(0, 0),
			ty: return_type,
		});

		self.functions.push(function.clone());
		self.function_name_to_function
			.insert(name.to_string(), function);
	}
}

pub fn parse_string(contents: &str) -> Result<P<Program>> {
	match PestParser::parse(Rule::program, &contents) {
		Ok(pairs) => Ok(P::new(Expression::parse_program(pairs)?)),
		Err(error) => panic!("{:?}", error),
	}
}

pub fn parse_file(file_name: &str) -> Result<P<Program>> {
	let contents = std::fs::read_to_string(file_name).context("Could not read file")?;
	return parse_string(&contents);
}
