use anyhow::Result;
use eggscript_ast::{compile_expression, parse_string};
use eggscript_interpreter::runtime::print::{clear_test_print_buffer, get_test_print_buffer};
use eggscript_interpreter::{
	get_test_native_function_mapping_for_interpreter, get_test_native_function_mapping_for_jit,
	Interpreter,
};
use eggscript_mir::EggscriptLowerContext;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::OptimizationLevel;
use serial_test::serial;

use crate::eggscript;
use crate::llvm;

fn assert_buffer(expected: Vec<&str>, message: &str) {
	let buffer = get_test_print_buffer();
	clear_test_print_buffer();

	assert!(
		buffer.iter().eq(expected.iter()),
		"expected = {:?}, buffer = {:?}; ({})",
		expected,
		buffer,
		message,
	);
}

fn run_file_in_interpreter(contents: &str, file_name: &str, timeout: u128) -> Result<()> {
	let program = parse_string(contents, file_name)?;
	let (ast_content, units) = compile_expression(program.clone(), program.global_scope.clone())?;

	let mut eggscript_context: EggscriptLowerContext = ast_content.into();
	let instructions = eggscript_context.compile_to_eggscript(&units, None)?;

	let mut interpreter = Interpreter::new(instructions);

	let native_function_mapping = get_test_native_function_mapping_for_interpreter();

	for function in program.functions.iter() {
		if function.scope.is_some() {
			let (_, instructions) = eggscript::lower_function(program.clone(), function, false)?;
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

	interpreter.run_with_timeout(timeout);

	Ok(())
}

type EntryFunction = unsafe extern "C" fn() -> ();

fn run_file_in_jit(contents: &str, file_name: &str) -> Result<()> {
	let program = parse_string(contents, file_name)?;

	let (ast_context, units) = compile_expression(program.clone(), program.global_scope.clone())?;

	let context = Context::create();
	let builder = context.create_builder();
	let module = context.create_module("main");
	let mut llvm_context = ast_context.into_llvm_lower_context(&context, &builder, &module);

	for function in program.functions.iter() {
		llvm_context.pre_define_function(&function.ty);
	}

	llvm_context.compile_to_ir(&units, None)?;
	llvm_context.optimize_ir();

	let engine = module
		.create_jit_execution_engine(OptimizationLevel::Default)
		.unwrap();

	let function_mapping = get_test_native_function_mapping_for_jit();

	for function in program.functions.iter() {
		if function.scope.is_some() {
			llvm::lower_function(
				&context,
				&builder,
				&module,
				program.clone(),
				function,
				false,
			)?;
		} else {
			let function_declaration = module.get_function(&function.name).unwrap();
			engine.add_global_mapping(
				&function_declaration,
				*function_mapping.get(&function.name).unwrap(),
			);
		}
	}

	unsafe {
		let function: JitFunction<EntryFunction> = engine.get_function("entry")?;
		function.call();
	}

	drop(llvm_context);

	Ok(())
}

#[test]
#[serial]
fn recursion1() -> Result<()> {
	let file_contents = include_str!("./test_cases/recursion1.egg");
	let file_name = "./test_cases/recursion1.egg";

	run_file_in_interpreter(file_contents, file_name, 1000)?;
	assert_buffer(vec!["6765"], "interpreter");

	run_file_in_jit(file_contents, file_name)?;
	assert_buffer(vec!["6765"], "jit");

	Ok(())
}

#[test]
#[serial]
fn for_loop1() -> Result<()> {
	let file_contents = include_str!("./test_cases/for_loop1.egg");
	let file_name = "./test_cases/for_loop1.egg";

	run_file_in_interpreter(file_contents, file_name, 1000)?;
	assert_buffer(vec!["1024"], "interpreter");

	run_file_in_jit(file_contents, file_name)?;
	assert_buffer(vec!["1024"], "jit");

	Ok(())
}

#[test]
#[serial]
fn math1() -> Result<()> {
	let file_contents = include_str!("./test_cases/math1.egg");
	let file_name = "./test_cases/math1.egg";

	run_file_in_interpreter(file_contents, file_name, 1000)?;
	assert_buffer(vec!["50159"], "interpreter");

	run_file_in_jit(file_contents, file_name)?;
	assert_buffer(vec!["50159"], "jit");

	Ok(())
}

#[test]
#[serial]
fn math2() -> Result<()> {
	let file_contents = include_str!("./test_cases/math2.egg");
	let file_name = "./test_cases/math2.egg";

	run_file_in_interpreter(file_contents, file_name, 1000)?;
	assert_buffer(vec!["6", "-3", "-3", "18.7", "-15", "-10"], "interpreter");

	run_file_in_jit(file_contents, file_name)?;
	assert_buffer(vec!["6", "-3", "-3", "18.7", "-15", "-10"], "jit");

	Ok(())
}
