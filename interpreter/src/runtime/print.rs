use std::sync::Mutex;

pub fn print_number(value: f64) {
	println!("{}", value);
}

static TEST_PRINT_BUFFER: Mutex<Vec<String>> = Mutex::new(vec![]);

pub fn test_print_number(value: f64) {
	TEST_PRINT_BUFFER.lock().unwrap().push(format!("{}", value))
}

pub fn clear_test_print_buffer() {
	TEST_PRINT_BUFFER.lock().unwrap().clear();
}

pub fn get_test_print_buffer() -> Vec<String> {
	return TEST_PRINT_BUFFER.lock().unwrap().clone();
}
