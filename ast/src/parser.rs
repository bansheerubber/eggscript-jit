use anyhow::{bail, Context, Result};
use eggscript_types::{TypeHandle, TypeStore, P};
use pest::error::{Error, ErrorVariant, LineColLocation};
use pest::iterators::Pairs;
use pest::pratt_parser::{Assoc, Op, PrattParser};
use pest::Parser;
use pest_derive::Parser;
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};

use crate::expressions::Expression;
use crate::pretty_error::{
	print_blank, print_dots, print_error_header, print_line_with_correction,
	print_line_with_squiggle,
};
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
	pub file_name: String,
	pub function_name_to_function: HashMap<String, P<Function>>,
	pub functions: Vec<P<Function>>,
	pub global_scope: P<Expression>,
	pub type_store: Arc<Mutex<TypeStore>>,
}

impl Program {
	pub fn add_native_function(
		&mut self,
		arguments: Vec<FunctionArgument>,
		name: &str,
		return_type: TypeHandle,
	) {
		let argument_types = arguments
			.iter()
			.map(|argument| argument.ty.unwrap())
			.collect::<Vec<TypeHandle>>();

		let ty = self.type_store.lock().unwrap().create_function_type(
			name,
			argument_types,
			Some(return_type),
		);

		let id = self.functions.len();
		let function = P::new(Function {
			arguments,
			id,
			name: name.to_string(),
			return_ty: Some(return_type),
			scope: None,
			span: Span::new(0, 0),
			ty,
		});

		self.functions.push(function.clone());
		self.function_name_to_function
			.insert(name.to_string(), function);
	}
}

pub fn parse_string(contents: &str, file_name: &str) -> Result<P<Program>> {
	match PestParser::parse(Rule::program, &contents) {
		Ok(pairs) => Ok(P::new(Expression::parse_program(
			file_name,
			Arc::new(Mutex::new(TypeStore::new())),
			pairs,
		)?)),
		Err(error) => {
			attempt_print_pest_error(error, contents, file_name);
			bail!("Could not parse string")
		}
	}
}

pub fn parse_file(file_name: &str) -> Result<P<Program>> {
	let contents = std::fs::read_to_string(file_name).context("Could not read file")?;
	return parse_string(&contents, file_name);
}

fn get_lines(contents: &str, start: usize, stop: usize) -> BTreeMap<usize, String> {
	let lines = contents
		.split('\n')
		.map(str::to_string)
		.collect::<Vec<String>>();

	let mut mapping = BTreeMap::new();

	let start = isize::max(0, start as isize - 1) as usize;
	let stop = isize::min(lines.len() as isize - 1, stop as isize - 1) as usize;

	let mut key = start;
	for line in &lines[start..=stop] {
		mapping.insert(key + 1, line.into());
		key += 1;
	}

	return mapping;
}

fn get_line_number(location: &LineColLocation) -> usize {
	match location {
		LineColLocation::Pos((start_line, _)) => *start_line,
		LineColLocation::Span((start_line, _), _) => *start_line,
	}
}

fn attempt_print_pest_error(error: Error<Rule>, contents: &str, file_name: &str) {
	let ErrorVariant::ParsingError {
		positives,
		negatives: _,
	} = &error.variant
	else {
		panic!("unknown error {:?}", error);
	};

	if positives[0] == Rule::function_return_type_ident {
		print_error_header(
			"missing return type after colon in function declaration",
			file_name,
		);

		let line_number = get_line_number(&error.line_col);
		let lines = get_lines(contents, line_number - 1, line_number + 1);
		print_line_with_squiggle(lines, &error.line_col, "type name expected here", -2);

		print_dots();
		print_line_with_correction(error.line(), &error.line_col, " type_name ", "like this");
		print_blank();
	} else if positives[0] == Rule::type_ident {
		print_error_header(
			"missing type after colon in variable declaration",
			file_name,
		);

		let line_number = get_line_number(&error.line_col);
		let lines = get_lines(contents, line_number - 1, line_number + 1);
		print_line_with_squiggle(lines, &error.line_col, "type name expected here", -2);

		print_dots();
		print_line_with_correction(
			error.line(),
			&error.line_col,
			" type_name ",
			"like this (or remove colon for implicit type declaration)",
		);
		print_blank();
	} else {
		print_error_header("unknown error during parsing", file_name);

		let line_number = get_line_number(&error.line_col);
		let lines = get_lines(contents, line_number - 1, line_number + 1);
		print_line_with_squiggle(lines, &error.line_col, "syntax parser stopped here", 0);

		print_blank();
	}
}
