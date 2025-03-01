use anyhow::Result;
use colored::Colorize;
use eggscript_ast::{compile_expression, compile_function, parse_string, Function, Program};
use eggscript_interpreter::{
	get_native_function_mapping_for_interpreter, Instruction, Interpreter,
};
use eggscript_mir::{EggscriptLowerContext, Unit};
use eggscript_types::P;
use serde::Serialize;
use std::ops::Deref;

pub fn instructions_to_vector_string(instructions: &Vec<Instruction>) -> Vec<String> {
	instructions
		.iter()
		.map(|instruction| format!("{:?}", instruction))
		.collect::<Vec<String>>()
}

pub fn mir_to_vector_string(units: &Vec<Unit>) -> Vec<String> {
	let mut result = Vec::new();

	for unit in units.iter() {
		let block = format!("{}", unit);
		result.extend(block.split("\n").map(str::to_string));
	}

	if result.last().unwrap().len() == 0 {
		result.pop();
	}

	return result;
}

#[allow(dead_code)]
pub fn lower_function(
	program: P<Program>,
	function: &P<Function>,
	debug: bool,
) -> Result<(Vec<Unit>, Vec<Instruction>)> {
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
	let instructions = eggscript_context.compile_to_eggscript(&units, Some(function.ty.clone()))?;

	Ok((units, instructions))
}

#[derive(Debug, Serialize)]
pub struct InterpreterFunctionResult {
	arguments: Vec<(String, String)>,
	name: String,
	return_ty: String,

	instructions: Vec<String>,
	mir: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct InterpreterCompilationResult {
	functions: Vec<InterpreterFunctionResult>,
}

pub fn compile_eggscript_program(
	contents: &str,
	file_name: &str,
) -> Result<InterpreterCompilationResult> {
	let program = parse_string(contents, file_name)?;

	let mut result = InterpreterCompilationResult {
		functions: Vec::new(),
	};

	let (ast_context, units) = compile_expression(program.clone(), program.global_scope.clone())?;

	let mut eggscript_context: EggscriptLowerContext = ast_context.into();
	let instructions = eggscript_context.compile_to_eggscript(&units, None)?;

	result.functions.push(InterpreterFunctionResult {
		arguments: Vec::new(),
		name: "entry".into(),
		return_ty: "void".into(),

		instructions: instructions_to_vector_string(&instructions),
		mir: mir_to_vector_string(&units),
	});

	for function in program.functions.iter() {
		if function.scope.is_some() {
			let (units, instructions) = lower_function(program.clone(), function, false)?;
			let type_store = program.type_store.lock().unwrap();

			let return_ty = if let Some(return_ty) = function.return_ty {
				type_store
					.get_type(return_ty)
					.unwrap()
					.get_name()
					.unwrap()
					.to_string()
			} else {
				"void".to_string()
			};

			result.functions.push(InterpreterFunctionResult {
				arguments: function
					.arguments
					.iter()
					.map(|argument| {
						let type_name = type_store
							.get_type(argument.ty.unwrap())
							.unwrap()
							.get_name()
							.unwrap();

						return (argument.name.clone(), type_name.to_string());
					})
					.collect::<Vec<(String, String)>>(),
				name: function.name.clone(),
				return_ty,

				instructions: instructions_to_vector_string(&instructions),
				mir: mir_to_vector_string(&units),
			});
		}
	}

	Ok(result)
}

#[allow(dead_code)]
pub fn run_eggscript_program(contents: &str, file_name: &str, debug: bool) -> Result<()> {
	let program = parse_string(contents, file_name)?;

	if debug {
		println!("{}", program.global_scope.deref());

		for function in program.functions.iter() {
			println!("{}", function.deref());
		}

		println!("{}", "Global program".yellow());
	}

	let (ast_context, units) = compile_expression(program.clone(), program.global_scope.clone())?;

	if debug {
		for unit in units.iter() {
			println!("{}", unit);
		}
	}

	let mut eggscript_context: EggscriptLowerContext = ast_context.into();
	let instructions = eggscript_context.compile_to_eggscript(&units, None)?;

	if debug {
		for instruction in instructions.iter() {
			println!("{:?}", instruction);
		}

		println!("");
	}

	let mut interpreter = Interpreter::new(instructions);

	let native_function_mapping = get_native_function_mapping_for_interpreter();

	for function in program.functions.iter() {
		if function.scope.is_some() {
			let (_, instructions) = lower_function(program.clone(), function, debug)?;
			if debug {
				for instruction in instructions.iter() {
					println!("{:?}", instruction);
				}
				println!("");
			}

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

	if debug {
		println!("Results:");
		interpreter.print_stack();
	}

	Ok(())
}
