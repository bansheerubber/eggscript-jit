use anyhow::Result;
use clap::Parser;
use eggscript_ast::{compile_file, parse_file};
use eggscript_interpreter::Interpreter;
use eggscript_mir::EggscriptLowerContext;
use std::ops::Deref;

#[derive(Parser)]
#[command(name = "eggscript")]
#[command(bin_name = "eggscript")]
enum Args {
	Run(RunArgs),
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
struct RunArgs;

fn main() -> Result<()> {
	let expressions = parse_file("test.egg")?;
	println!("{}", expressions.deref());

	let (ast_context, units) = compile_file(expressions)?;
	for unit in units.iter() {
		println!("{}", unit);
	}

	let mut eggscript_context: EggscriptLowerContext = ast_context.into();
	let instructions = eggscript_context.lower_units(units)?;

	for instruction in instructions.iter() {
		println!("{:?}", instruction);
	}

	let mut interpreter = Interpreter::new(instructions);
	interpreter.run();

	println!("\nResults:");

	interpreter.print_stack();

	Ok(())
}
