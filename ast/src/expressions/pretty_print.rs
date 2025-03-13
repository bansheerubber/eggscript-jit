use colored::Colorize;
use eggscript_types::P;

use crate::expressions::{Block, Expression, ExpressionInfo};

impl Expression {
	pub(crate) fn pretty_print(
		&self,
		f: &mut std::fmt::Formatter<'_>,
		initial_prefix: &str,
		prefix: &str,
	) -> std::fmt::Result {
		match &self.info {
			ExpressionInfo::Assign(name, operator, expression) => {
				f.write_fmt(format_args!(
					"{}{} name:'{}', op:'{}', type:'{}'\n",
					initial_prefix,
					"Assign".yellow(),
					name.name().cyan(),
					format!("{:?}", operator).cyan(),
					format!("{:?}", self.ty).cyan(),
				))?;

				expression.pretty_print(f, &format!("{}`- ", prefix), &format!("{}   ", prefix))
			}
			ExpressionInfo::BinaryOperation(left, right, operator) => {
				f.write_fmt(format_args!(
					"{}{} op:'{}'\n",
					initial_prefix,
					"BinaryOperation".yellow(),
					format!("{:?}", operator).cyan(),
				))?;

				left.pretty_print(f, &format!("{}|- ", prefix), &format!("{}|  ", prefix))?;
				right.pretty_print(f, &format!("{}`- ", prefix), &format!("{}|  ", prefix))
			}
			ExpressionInfo::Else(block) => {
				f.write_fmt(format_args!("{}{}\n", initial_prefix, "Else".yellow()))?;
				pretty_print_block(
					&block,
					f,
					&format!("{}`- ", prefix),
					&format!("{}|  ", prefix),
				)
			}
			ExpressionInfo::FieldAccess(name) => f.write_fmt(format_args!(
				"{}{} name:'{}'\n",
				initial_prefix,
				"FieldAccess".yellow(),
				name.name().cyan(),
			)),
			ExpressionInfo::For(declaration, conditional, update, block) => {
				f.write_fmt(format_args!("{}{}\n", initial_prefix, "For".yellow()))?;

				f.write_fmt(format_args!("{}|- {}\n", prefix, "(Declaration)".yellow(),))?;

				declaration.pretty_print(
					f,
					&format!("{}|  `- ", prefix),
					&format!("{}|  |  ", prefix),
				)?;

				f.write_fmt(format_args!("{}|- {}\n", prefix, "(Conditional)".yellow(),))?;

				conditional.pretty_print(
					f,
					&format!("{}|  `- ", prefix),
					&format!("{}|  |  ", prefix),
				)?;

				f.write_fmt(format_args!("{}|- {}\n", prefix, "(Update)".yellow(),))?;

				update.pretty_print(
					f,
					&format!("{}|  `- ", prefix),
					&format!("{}|  |  ", prefix),
				)?;

				pretty_print_block(
					&block,
					f,
					&format!("{}`- ", prefix),
					&format!("{}|  ", prefix),
				)
			}
			ExpressionInfo::FunctionCall(name, arguments) => {
				f.write_fmt(format_args!(
					"{}{} name:'{}'\n",
					initial_prefix,
					"Function call".yellow(),
					name.name().cyan()
				))?;

				f.write_fmt(format_args!(
					"{}|- {}\n",
					prefix,
					"(Function args)".yellow(),
				))?;

				for argument in arguments.iter() {
					argument.pretty_print(
						f,
						&format!("{}|  `- ", prefix),
						&format!("{}|  |  ", prefix),
					)?;
				}

				Ok(())
			}
			ExpressionInfo::If(conditional, block, next) => {
				f.write_fmt(format_args!("{}{}\n", initial_prefix, "If".yellow()))?;

				f.write_fmt(format_args!("{}|- {}\n", prefix, "(Conditional)".yellow(),))?;

				conditional.pretty_print(
					f,
					&format!("{}|  `- ", prefix),
					&format!("{}|  |  ", prefix),
				)?;

				let block_prefix = if next.is_some() {
					format!("{}|- ", prefix)
				} else {
					format!("{}`- ", prefix)
				};

				pretty_print_block(&block, f, &block_prefix, &format!("{}|  ", prefix))?;

				// handle printing next else-if/else
				if let Some(next) = next {
					next.pretty_print(f, &format!("{}`- ", prefix), &format!("{}|  ", prefix))?;
				}

				Ok(())
			}
			ExpressionInfo::LogicOperation(left, right, operator) => {
				f.write_fmt(format_args!(
					"{}{} op:'{}'\n",
					initial_prefix,
					"LogicOperation".yellow(),
					format!("{:?}", operator).cyan(),
				))?;

				left.pretty_print(f, &format!("{}|- ", prefix), &format!("{}|  ", prefix))?;
				right.pretty_print(f, &format!("{}`- ", prefix), &format!("{}|  ", prefix))
			}
			ExpressionInfo::Primitive(_, value) => f.write_fmt(format_args!(
				"{}{} type:'{}', value: '{}'\n",
				initial_prefix,
				"Primitive".yellow(),
				format!("{:?}", self.ty).cyan(),
				value.cyan()
			)),
			ExpressionInfo::Return(expression) => {
				f.write_fmt(format_args!("{}{}\n", initial_prefix, "Return".yellow()))?;

				if let Some(expression) = expression {
					expression.pretty_print(
						f,
						&format!("{}`- ", prefix),
						&format!("{}   ", prefix),
					)?;
				}

				Ok(())
			}
			ExpressionInfo::Scope(block) => {
				f.write_fmt(format_args!("{}{}\n", initial_prefix, "Scope".yellow()))?;

				pretty_print_block(
					&block,
					f,
					&format!("{}`- ", prefix),
					&format!("{}|  ", prefix),
				)
			}
			ExpressionInfo::UnaryOperation(expression, operator) => {
				f.write_fmt(format_args!(
					"{}{} op:'{}'\n",
					initial_prefix,
					"UnaryOperator".yellow(),
					format!("{:?}", operator).cyan(),
				))?;

				expression.pretty_print(f, &format!("{}`- ", prefix), &format!("{}|  ", prefix))
			}
			ExpressionInfo::While(conditional, block) => {
				f.write_fmt(format_args!("{}{}\n", initial_prefix, "While".yellow()))?;

				f.write_fmt(format_args!("{}|- {}\n", prefix, "(Conditional)".yellow(),))?;

				conditional.pretty_print(
					f,
					&format!("{}|  `- ", prefix),
					&format!("{}|  |  ", prefix),
				)?;

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

pub(crate) fn pretty_print_block(
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
