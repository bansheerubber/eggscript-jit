use anyhow::Result;
use serial_test::serial;

use super::{assert_buffer, run_file_in_interpreter, run_file_in_jit};

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
	assert_buffer(
		vec!["6", "-3.5", "-3.5", "18.7", "-15", "-10"],
		"interpreter",
	);

	run_file_in_jit(file_contents, file_name)?;
	assert_buffer(vec!["6", "-3.5", "-3.5", "18.7", "-15", "-10"], "jit");

	Ok(())
}

#[test]
#[serial]
fn math3() -> Result<()> {
	let file_contents = include_str!("./test_cases/math3.egg");
	let file_name = "./test_cases/math3.egg";

	run_file_in_interpreter(file_contents, file_name, 1000)?;
	assert_buffer(vec!["2", "11", "9", "80", "1", "-6", "0", "1", "0"], "interpreter");

	run_file_in_jit(file_contents, file_name)?;
	assert_buffer(vec!["2", "11", "9", "80", "1", "-6", "0", "1", "0"], "jit");

	Ok(())
}
