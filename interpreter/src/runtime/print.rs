use std::sync::Mutex;

use crate::Value;

pub fn print_double(values: Vec<Value>) -> Value {
	if values.len() == 0 {
		return Value::Null;
	}

	match values.first().unwrap() {
		Value::Boolean(value) => println!("{}", value),
		Value::Double(value) => println!("{}", value),
		Value::Integer(value) => println!("{}", value),
		Value::Null => println!("null"),
	}

	Value::Null
}

static TEST_PRINT_BUFFER: Mutex<Vec<String>> = Mutex::new(vec![]);

pub fn test_print(values: Vec<Value>) -> Value {
	if values.len() == 0 {
		return Value::Null;
	}

	match values.first().unwrap() {
		Value::Boolean(value) => TEST_PRINT_BUFFER.lock().unwrap().push(format!("{}", value)),
		Value::Double(value) => TEST_PRINT_BUFFER.lock().unwrap().push(format!("{}", value)),
		Value::Integer(value) => TEST_PRINT_BUFFER.lock().unwrap().push(format!("{}", value)),
		Value::Null => TEST_PRINT_BUFFER.lock().unwrap().push("null".into()),
	}

	Value::Null
}

pub fn clear_test_print_buffer() {
	TEST_PRINT_BUFFER.lock().unwrap().clear();
}

pub fn get_test_print_buffer() -> Vec<String> {
	return TEST_PRINT_BUFFER.lock().unwrap().clone();
}
