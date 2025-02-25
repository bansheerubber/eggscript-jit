mod eggscript;
mod llvm;

use anyhow::Result;
use clap::Parser;

#[cfg(test)]
mod integration_tests;

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
	// eggscript::run_eggscript_program()
	llvm::run_llvm_program()
}
