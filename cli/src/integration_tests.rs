use anyhow::Result;
use eggscript_ast::{compile_expression, parse_string};
use eggscript_interpreter::{Interpreter, Value};
use eggscript_mir::EggscriptLowerContext;
use std::rc::Rc;
use std::sync::Mutex;

use crate::lower_function;

static TEST_PRINT_BUFFER: Mutex<Vec<String>> = Mutex::new(vec![]);

fn assert_buffer(expected: Vec<&str>) {
	let buffer = TEST_PRINT_BUFFER.lock().unwrap().clone();
	TEST_PRINT_BUFFER.lock().unwrap().clear();

	assert!(
		buffer.iter().eq(expected.iter()),
		"expected = {:?}, buffer = {:?}",
		expected,
		buffer,
	);
}

fn test_print(values: Vec<Value>) -> Value {
	if values.len() == 0 {
		return Value::Null;
	}

	match values.first().unwrap() {
		Value::Boolean(value) => TEST_PRINT_BUFFER.lock().unwrap().push(format!("{}", value)),
		Value::Double(value) => TEST_PRINT_BUFFER.lock().unwrap().push(format!("{}", value)),
		Value::Integer(value) => TEST_PRINT_BUFFER.lock().unwrap().push(format!("{}", value)),
		Value::Null => TEST_PRINT_BUFFER.lock().unwrap().push("null".into()),
	}

	Value::Null
}

fn run_file(contents: &str) -> Result<()> {
	let program = parse_string(contents)?;
	let (ast_content, units) = compile_expression(program.clone(), program.global_scope.clone())?;

	let mut eggscript_context: EggscriptLowerContext = ast_content.into();
	let instructions = eggscript_context.lower_units(units)?;

	let mut interpreter = Interpreter::new(instructions);

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
				Rc::new(test_print),
				&function.name,
			));
		}
	}

	interpreter.run();

	Ok(())
}

#[test]
fn recursion1() -> Result<()> {
	run_file(include_str!("./tests/recursion1.egg"))?;
	assert_buffer(vec!["6765"]);
	Ok(())
}

#[test]
fn for_loop1() -> Result<()> {
	run_file(include_str!("./tests/for_loop1.egg"))?;
	assert_buffer(vec!["1024"]);
	Ok(())
}
