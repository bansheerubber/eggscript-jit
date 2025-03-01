use anyhow::{Context, Result};
use colored::Colorize;
use eggscript_types::{FunctionType, TypeHandle, P};
use pest::iterators::Pair;

use crate::expressions::Expression;
use crate::parser::Rule;
use crate::{AstContext, Span};

#[derive(Clone, Debug)]
pub struct FunctionArgument {
	pub name: String,
	#[allow(dead_code)]
	pub span: Span,
	pub ty: Option<TypeHandle>,
}

#[derive(Debug)]
pub struct Function {
	pub arguments: Vec<FunctionArgument>,
	pub id: usize,
	pub name: String,
	pub return_ty: Option<TypeHandle>,
	pub scope: Option<P<Expression>>,
	#[allow(dead_code)]
	pub span: Span,
	pub ty: FunctionType,
}

impl Function {
	fn pretty_print(
		&self,
		f: &mut std::fmt::Formatter<'_>,
		initial_prefix: &str,
		prefix: &str,
	) -> std::fmt::Result {
		f.write_fmt(format_args!(
			"{}{} name:'{}', type:'{}'\n",
			initial_prefix,
			"Function".yellow(),
			self.name.cyan(),
			format!("{:?}", self.return_ty).cyan(),
		))?;

		f.write_fmt(format_args!(
			"{}|- {}\n",
			prefix,
			"(Function args)".yellow(),
		))?;

		let length = self.arguments.len();
		let mut index = 0;
		for argument in self.arguments.iter() {
			let symbol = if index == length - 1 { "`-" } else { "|-" };

			f.write_fmt(format_args!(
				"{}|  {} {} name:'{}', type:'{}'\n",
				prefix,
				symbol,
				"Argument".yellow(),
				argument.name.cyan(),
				format!("{:?}", argument.ty).cyan(),
			))?;

			index += 1;
		}

		if let Some(scope) = self.scope.as_ref() {
			scope.pretty_print(f, &format!("{}`- ", prefix), &format!("{}|  ", prefix))?;
		}

		Ok(())
	}
}

impl std::fmt::Display for Function {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.pretty_print(f, "", "")
	}
}

impl Expression {
	pub(crate) fn parse_function_declaration(
		context: &mut AstContext,
		pair: Pair<Rule>,
		id: usize,
	) -> Result<P<Function>> {
		let span = pair.as_span().into();
		let mut pairs = pair.into_inner();

		let name_pair = pairs.next().context("Could not get function name")?;
		let name = name_pair.as_str();

		let function_arg_list = pairs
			.next()
			.context("Could not get function arg list")?
			.into_inner();

		let mut arguments = vec![];
		for function_arg in function_arg_list.into_iter() {
			let span = function_arg.as_span().into();
			let mut function_arg_inner = function_arg.into_inner();

			let name = function_arg_inner
				.next()
				.context("Could not unwrap function arg name")?
				.as_str()
				.to_string();

			let type_name = function_arg_inner
				.next()
				.context("Could not unwrap function arg type")?
				.as_str();

			let ty = context
				.type_store
				.lock()
				.unwrap()
				.name_to_type_handle(type_name);

			arguments.push(FunctionArgument { name, span, ty });
		}

		let return_type = if let Rule::function_return_type =
			pairs.peek().context("Could not peek next")?.as_rule()
		{
			let return_type = pairs
				.next()
				.context("Could not get return type")?
				.into_inner()
				.next()
				.context("Could not get return type")?
				.as_str();

			Some(
				context
					.type_store
					.lock()
					.unwrap()
					.name_to_type_handle(return_type)
					.context("Could not find return type")?,
			)
		} else {
			None
		};

		let block = pairs.next().context("Could not get next pair")?;
		let block_span = block.as_span().into();

		let expressions = block
			.into_inner()
			.map(|p| {
				Expression::parse_pair(context, p)
					.context("Could not parse pair")
					.unwrap()
			})
			.collect::<Vec<Result<P<Expression>>>>();

		let function_ty = context.type_store.lock().unwrap().create_function_type(
			name,
			arguments
				.iter()
				.map(|argument| argument.ty.unwrap())
				.collect::<Vec<TypeHandle>>(),
			return_type,
		);

		Ok(P::new(Function {
			arguments,
			id,
			name: name.into(),
			return_ty: return_type,
			scope: Some(Expression::new_scope(expressions, block_span)?),
			span,
			ty: function_ty,
		}))
	}
}
