mod eggscript;
mod llvm;

use anyhow::{Context, Result};
use clap::Parser;

#[cfg(test)]
mod tests;

#[derive(Debug, Parser)]
#[command(name = "eggscript")]
#[command(bin_name = "eggscript")]
enum Args {
	Compile(CompileArgs),
	Run(RunArgs),
}

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
struct CompileArgs {
	#[clap(flatten)]
	contents_or_file_name: ContentsOrFileNameGroup,

	#[clap(flatten)]
	interpreter_or_llvm: InterpreterOrLLVMGroup,
}

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
struct RunArgs {
	#[clap(flatten)]
	contents_or_file_name: ContentsOrFileNameGroup,

	#[clap(flatten)]
	interpreter_or_llvm: InterpreterOrLLVMGroup,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
struct ContentsOrFileNameGroup {
	#[arg(short, long)]
	contents: Option<String>,

	#[arg(short, long)]
	file_name: Option<String>,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
struct InterpreterOrLLVMGroup {
	#[arg(short, long)]
	interpreter: bool,

	#[arg(short, long)]
	llvm: bool,
}

fn main() -> Result<()> {
	let args = Args::parse();
	match args {
		Args::Compile(CompileArgs {
			contents_or_file_name: ContentsOrFileNameGroup {
				contents,
				file_name,
			},
			interpreter_or_llvm: InterpreterOrLLVMGroup { interpreter, .. },
		}) => {
			let (contents, file_name) = if let Some(contents) = contents {
				(contents, "main.egg".to_string())
			} else if let Some(file_name) = file_name {
				(
					std::fs::read_to_string(&file_name).context("Could not read file")?,
					file_name.clone(),
				)
			} else {
				unreachable!();
			};

			if interpreter {
				let result = eggscript::compile_eggscript_program(&contents, &file_name)?;
				println!("{}", serde_json::to_string_pretty(&result)?);
			} else {
				let result = llvm::compile_llvm_program(&contents, &file_name)?;
				println!("{}", serde_json::to_string_pretty(&result)?);
			}
		}
		Args::Run(RunArgs {
			contents_or_file_name: ContentsOrFileNameGroup {
				contents,
				file_name,
			},
			interpreter_or_llvm: InterpreterOrLLVMGroup { interpreter, .. },
		}) => {
			let (contents, file_name) = if let Some(contents) = contents {
				(contents, "main.egg".to_string())
			} else if let Some(file_name) = file_name {
				(
					std::fs::read_to_string(&file_name).context("Could not read file")?,
					file_name.clone(),
				)
			} else {
				unreachable!();
			};

			if interpreter {
				eggscript::run_eggscript_program(&contents, &file_name)?;
			} else {
				llvm::run_llvm_program(&contents, &file_name)?;
			}
		}
	}

	Ok(())
}
