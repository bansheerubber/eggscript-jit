use anyhow::Result;
use serial_test::serial;

use super::{assert_buffer, run_file_in_interpreter, run_file_in_jit};

#[test]
#[serial]
fn conditionals() -> Result<()> {
	let file_contents = include_str!("./test_cases/conditionals.egg");
	let file_name = "./test_cases/conditionals.egg";

	run_file_in_interpreter(file_contents, file_name, 1000)?;
	assert_buffer(
		vec![
			"0", "1", "1", "0", "1", "0", "0", "0", "1", "1", "0", "1", "2", "3", "2", "4", "5",
		],
		"interpreter",
	);

	run_file_in_jit(file_contents, file_name)?;
	assert_buffer(
		vec![
			"0", "1", "1", "0", "1", "0", "0", "0", "1", "1", "0", "1", "2", "3", "2", "4", "5",
		],
		"jit",
	);

	Ok(())
}

#[test]
#[serial]
fn logic() -> Result<()> {
	let file_contents = include_str!("./test_cases/logic.egg");
	let file_name = "./test_cases/logic.egg";

	run_file_in_interpreter(file_contents, file_name, 1000)?;
	assert_buffer(vec!["12", "4", "16", "5"], "interpreter");

	run_file_in_jit(file_contents, file_name)?;
	assert_buffer(vec!["12", "4", "16", "5"], "jit");

	Ok(())
}
