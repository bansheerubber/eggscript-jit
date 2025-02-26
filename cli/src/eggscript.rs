use anyhow::Result;
use colored::Colorize;
use eggscript_ast::{compile_expression, compile_function, parse_file, Function, Program};
use eggscript_interpreter::{get_native_function_mapping_for_interpreter, Instruction, Interpreter};
use eggscript_mir::EggscriptLowerContext;
use eggscript_types::P;
use std::ops::Deref;

#[allow(dead_code)]
pub fn lower_function(
	program: P<Program>,
	function: &P<Function>,
	debug: bool,
) -> Result<Vec<Instruction>> {
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

	let mut eggscript_context: EggscriptLowerContext = ast_context.into();
	let instructions = eggscript_context.compile_to_eggscript(units, Some(function.ty.clone()))?;

	Ok(instructions)
}

#[allow(dead_code)]
pub fn run_eggscript_program() -> Result<()> {
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

	let mut eggscript_context: EggscriptLowerContext = ast_context.into();
	let instructions = eggscript_context.compile_to_eggscript(units, None)?;

	for instruction in instructions.iter() {
		println!("{:?}", instruction);
	}

	println!("");

	let mut interpreter = Interpreter::new(instructions);

	let native_function_mapping = get_native_function_mapping_for_interpreter();

	for function in program.functions.iter() {
		if function.scope.is_some() {
			let instructions = lower_function(program.clone(), function, true)?;
			for instruction in instructions.iter() {
				println!("{:?}", instruction);
			}
			println!("");

			interpreter.add_function(eggscript_interpreter::Function::new_eggscript_function(
				function.id,
				function.arguments.len(),
				instructions,
				&function.name,
			));
		} else {
			interpreter.add_function(eggscript_interpreter::Function::new_native(
				function.id,
				function.arguments.len(),
				native_function_mapping.get(&function.name).unwrap().clone(),
				&function.name,
			));
		}
	}

	interpreter.run();

	println!("Results:");

	interpreter.print_stack();

	Ok(())
}
