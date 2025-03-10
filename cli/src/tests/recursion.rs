use anyhow::Result;
use serial_test::serial;

use super::{assert_buffer, run_file_in_interpreter, run_file_in_jit};

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
