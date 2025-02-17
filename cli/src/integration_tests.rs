use anyhow::Result;
use eggscript_ast::{compile_expression, parse_string};
use eggscript_interpreter::runtime::print::{clear_test_print_buffer, get_test_print_buffer};
use eggscript_interpreter::{get_test_native_function_mapping, Interpreter};
use eggscript_mir::EggscriptLowerContext;

use crate::lower_function;

fn assert_buffer(expected: Vec<&str>) {
	let buffer = get_test_print_buffer();
	clear_test_print_buffer();

	assert!(
		buffer.iter().eq(expected.iter()),
		"expected = {:?}, buffer = {:?}",
		expected,
		buffer,
	);
}

fn run_file(contents: &str, file_name: &str) -> Result<()> {
	let program = parse_string(contents, file_name)?;
	let (ast_content, units) = compile_expression(program.clone(), program.global_scope.clone())?;

	let mut eggscript_context: EggscriptLowerContext = ast_content.into();
	let instructions = eggscript_context.compile_to_eggscript(units)?;

	let mut interpreter = Interpreter::new(instructions);

	let native_function_mapping = get_test_native_function_mapping();

	for function in program.functions.iter() {
		if function.scope.is_some() {
			let instructions = lower_function(program.clone(), function, false)?;
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

	Ok(())
}

#[test]
fn recursion1() -> Result<()> {
	run_file(
		include_str!("./tests/recursion1.egg"),
		"./tests/recursion1.egg",
	)?;

	assert_buffer(vec!["6765"]);
	Ok(())
}

#[test]
fn for_loop1() -> Result<()> {
	run_file(
		include_str!("./tests/for_loop1.egg"),
		"./tests/for_loop1.egg",
	)?;

	assert_buffer(vec!["1024"]);
	Ok(())
}

#[test]
fn math1() -> Result<()> {
	run_file(include_str!("./tests/math1.egg"), "./tests/math1.egg")?;
	assert_buffer(vec!["50159"]);
	Ok(())
}
