use colored::Colorize;
use eggscript_types::P;

use crate::expressions::{Block, Expression, ExpressionInfo};

impl Expression {
	fn pretty_print(
		&self,
		f: &mut std::fmt::Formatter<'_>,
		initial_prefix: &str,
		prefix: &str,
	) -> std::fmt::Result {
		match &self.info {
			ExpressionInfo::Assign(name, operator, expression) => {
				f.write_fmt(format_args!(
					"{}{} name:'{}', op:'{}'\n",
					initial_prefix,
					"Assign".yellow(),
					name.name().cyan(),
					format!("{:?}", operator).cyan()
				))?;

				expression.pretty_print(f, &format!("{}`- ", prefix), &format!("{}   ", prefix))
			}
			ExpressionInfo::Primitive(value) => f.write_fmt(format_args!(
				"{}{} type:'{}', value: '{}'\n",
				initial_prefix,
				"Primitive".yellow(),
				format!("{:?}", self.ty).cyan(),
				value.cyan()
			)),
			ExpressionInfo::Scope(block) => {
				f.write_fmt(format_args!("{}{}\n", initial_prefix, "Scope".yellow()))?;

				pretty_print_block(
					&block,
					f,
					&format!("{}`- ", prefix),
					&format!("{}|  ", prefix),
				)
			}
		}
	}
}

impl std::fmt::Display for Expression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.pretty_print(f, "", "")
	}
}

fn pretty_print_block(
	block: &P<Block>,
	f: &mut std::fmt::Formatter,
	initial_prefix: &str,
	prefix: &str,
) -> std::fmt::Result {
	f.write_fmt(format_args!("{}{}\n", initial_prefix, "(Block)".yellow()))?;

	let length = block.expressions().len();
	let mut index = 0;
	for expression in block.expressions().iter() {
		if length - 1 != index {
			expression.pretty_print(f, &format!("{}|- ", prefix), &format!("{}|  ", prefix))?;
		} else {
			expression.pretty_print(f, &format!("{}`- ", prefix), &format!("{}|  ", prefix))?;
		}
		index += 1;
	}

	Ok(())
}
