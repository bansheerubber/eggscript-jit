use std::{collections::HashMap, rc::Rc};

use super::print;
use crate::{function::NativeFunction, Value};

pub fn get_native_function_mapping_for_interpreter() -> HashMap<String, NativeFunction> {
	let mut mapping: HashMap<String, NativeFunction> = HashMap::new();

	mapping.insert(
		"printNumber".to_string(),
		Rc::new(|values| {
			print::print_number(values.get(0).unwrap().as_number());
			return Value::Null;
		}),
	);

	return mapping;
}

pub fn get_test_native_function_mapping_for_interpreter() -> HashMap<String, NativeFunction> {
	let mut mapping = get_native_function_mapping_for_interpreter();

	mapping.insert(
		"printNumber".to_string(),
		Rc::new(|values| {
			print::test_print_number(values.get(0).unwrap().as_number());
			return Value::Null;
		}),
	);

	return mapping;
}

pub fn get_native_function_mapping_for_jit() -> HashMap<String, usize> {
	let mut mapping: HashMap<String, usize> = HashMap::new();
	mapping.insert("printNumber".to_string(), print::print_number as usize);
	return mapping;
}

pub fn get_test_native_function_mapping_for_jit() -> HashMap<String, usize> {
	let mut mapping: HashMap<String, usize> = HashMap::new();
	mapping.insert("printNumber".to_string(), print::test_print_number as usize);
	return mapping;
}
