use anyhow::Result;
use colored::Colorize;
use eggscript_ast::{compile_expression, compile_function, parse_file, Function, Program};
use eggscript_types::P;
use inkwell::{
	builder::Builder,
	context::Context,
	module::Module,
	values::{AnyValue, FunctionValue},
};
use std::ops::Deref;

fn lower_function<'a, 'ctx>(
	context: &'ctx Context,
	builder: &'a Builder<'ctx>,
	module: &'a Module<'ctx>,
	program: P<Program>,
	function: &P<Function>,
	debug: bool,
) -> Result<FunctionValue<'ctx>> {
	let (ast_context, units) = compile_function(
		function.clone(),
		program,
		function.scope.as_ref().unwrap().clone(),
	)?;

	if debug {
		println!(
			"{} '{}', {}",
			"Function".yellow(),
			function.name.cyan(),
			function.id
		);

		for unit in units.iter() {
			println!("{}", unit);
		}
	}

	let mut llvm_context = ast_context.into_llvm_lower_context(context, builder, module);
	let function_value = llvm_context.compile_to_ir(units, Some(function.ty.clone()));
	llvm_context.optimize_ir();
	return function_value;
}

pub fn run_llvm_program() -> Result<()> {
	let program = parse_file("test.egg")?;

	println!("{}", program.global_scope.deref());

	for function in program.functions.iter() {
		println!("{}", function.deref());
	}

	println!("{}", "Global program".yellow());

	let (ast_context, units) = compile_expression(program.clone(), program.global_scope.clone())?;
	for unit in units.iter() {
		println!("{}", unit);
	}

	let context = Context::create();
	let builder = context.create_builder();
	let module = context.create_module("main");
	let mut llvm_context = ast_context.into_llvm_lower_context(&context, &builder, &module);

	for function in program.functions.iter() {
		llvm_context.pre_define_function(&function.ty);
	}

	let entry = llvm_context.compile_to_ir(units, None)?;
	llvm_context.optimize_ir();
	println!("{}", "LLVM IR".yellow());
	println!("{}", entry.print_to_string().to_string_lossy());

	for function in program.functions.iter() {
		if function.scope.is_some() {
			let function =
				lower_function(&context, &builder, &module, program.clone(), function, true)?;

			println!("{}", "LLVM IR".yellow());
			println!("{}", function.print_to_string().to_string_lossy());
		}
	}

	llvm_context.write();

	drop(llvm_context);

	Ok(())
}
