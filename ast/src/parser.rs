use anyhow::{Context, Result};
use eggscript_types::P;
use pest::iterators::Pairs;
use pest::pratt_parser::PrattParser;
use pest::Parser;
use pest_derive::Parser;

use crate::expressions::Expression;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub(crate) struct PestParser<'a> {
	pub(crate) pairs: Pairs<'a, Rule>,
	pub(crate) pratt: PrattParser<Rule>,
}

pub fn parse_file(file_name: &str) -> Result<P<Expression>> {
	let file = std::fs::read_to_string(file_name).context("Could not read file")?;
	match PestParser::parse(Rule::program, &file) {
		Ok(pairs) => Expression::parse_program(pairs),
		Err(error) => panic!("{:?}", error),
	}
}
