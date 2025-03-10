use anyhow::Result;
use serial_test::serial;

use super::{assert_buffer, run_file_in_interpreter, run_file_in_jit};

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
fn while_loop1() -> Result<()> {
	let file_contents = include_str!("./test_cases/while_loop1.egg");
	let file_name = "./test_cases/while_loop1.egg";

	run_file_in_interpreter(file_contents, file_name, 1000)?;
	assert_buffer(vec!["1024"], "interpreter");

	run_file_in_jit(file_contents, file_name)?;
	assert_buffer(vec!["1024"], "jit");

	Ok(())
}
