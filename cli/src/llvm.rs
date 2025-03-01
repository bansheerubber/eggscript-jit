use anyhow::Result;
use colored::Colorize;
use eggscript_ast::{compile_expression, compile_function, parse_string, Function, Program};
use eggscript_interpreter::get_native_function_mapping_for_jit;
use eggscript_mir::Unit;
use eggscript_types::P;
use inkwell::{
	builder::Builder,
	context::Context,
	execution_engine::JitFunction,
	module::Module,
	values::{AnyValue, FunctionValue},
	OptimizationLevel,
};
use serde::Serialize;
use std::ops::Deref;

pub fn llvm_to_vector_string(function: &FunctionValue<'_>) -> Vec<String> {
	let mut result = Vec::new();
	let string = function.print_to_string().to_string_lossy().to_string();
	result.extend(string.split("\n").map(str::to_string));

	if result.last().unwrap().len() == 0 {
		result.pop();
	}

	return result;
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

pub fn lower_function<'a, 'ctx>(
	context: &'ctx Context,
	builder: &'a Builder<'ctx>,
	module: &'a Module<'ctx>,
	program: P<Program>,
	function: &P<Function>,
	debug: bool,
) -> Result<(Vec<Unit>, FunctionValue<'ctx>)> {
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
	let function_value = llvm_context.compile_to_ir(&units, Some(function.ty.clone()))?;
	llvm_context.optimize_ir();
	return Ok((units, function_value));
}

#[derive(Debug, Serialize)]
pub struct LLVMFunctionResult {
	arguments: Vec<(String, String)>,
	name: String,
	return_ty: String,

	llvm_ir: Vec<String>,
	mir: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct LLVMCompilationResult {
	functions: Vec<LLVMFunctionResult>,
}

pub fn compile_llvm_program(contents: &str, file_name: &str) -> Result<LLVMCompilationResult> {
	let program = parse_string(contents, file_name)?;

	let mut result = LLVMCompilationResult {
		functions: Vec::new(),
	};

	let (ast_context, units) = compile_expression(program.clone(), program.global_scope.clone())?;

	let context = Context::create();
	let builder = context.create_builder();
	let module = context.create_module("main");
	let mut llvm_context = ast_context.into_llvm_lower_context(&context, &builder, &module);

	for function in program.functions.iter() {
		llvm_context.pre_define_function(&function.ty);
	}

	let entry = llvm_context.compile_to_ir(&units, None)?;
	llvm_context.optimize_ir();

	result.functions.push(LLVMFunctionResult {
		arguments: vec![],
		name: "entry".into(),
		return_ty: "void".into(),

		llvm_ir: llvm_to_vector_string(&entry),
		mir: mir_to_vector_string(&units),
	});

	for function in program.functions.iter() {
		if function.scope.is_some() {
			let (units, llvm_function) = lower_function(
				&context,
				&builder,
				&module,
				program.clone(),
				function,
				false,
			)?;

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

			result.functions.push(LLVMFunctionResult {
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

				llvm_ir: llvm_to_vector_string(&llvm_function),
				mir: mir_to_vector_string(&units),
			});
		}
	}

	drop(llvm_context);

	Ok(result)
}

type EntryFunction = unsafe extern "C" fn();

#[allow(dead_code)]
pub fn run_llvm_program(contents: &str, file_name: &str, debug: bool) -> Result<()> {
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

	let context = Context::create();
	let builder = context.create_builder();
	let module = context.create_module("main");
	let mut llvm_context = ast_context.into_llvm_lower_context(&context, &builder, &module);

	for function in program.functions.iter() {
		llvm_context.pre_define_function(&function.ty);
	}

	let entry = llvm_context.compile_to_ir(&units, None)?;
	llvm_context.optimize_ir();

	if debug {
		println!("{}", "LLVM IR".yellow());
		println!("{}", entry.print_to_string().to_string_lossy());
	}

	let engine = module
		.create_jit_execution_engine(OptimizationLevel::Default)
		.unwrap();

	let function_mapping = get_native_function_mapping_for_jit();

	for function in program.functions.iter() {
		if function.scope.is_some() {
			let (_, function) = lower_function(
				&context,
				&builder,
				&module,
				program.clone(),
				function,
				debug,
			)?;

			if debug {
				println!("{}", "LLVM IR".yellow());
				println!("{}", function.print_to_string().to_string_lossy());
			}
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
