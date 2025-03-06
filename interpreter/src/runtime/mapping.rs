use std::{collections::HashMap, rc::Rc};

use super::print;
use crate::{function::NativeFunction, Value};

pub fn get_native_function_mapping_for_interpreter() -> HashMap<String, NativeFunction> {
	let mut mapping: HashMap<String, NativeFunction> = HashMap::new();

	mapping.insert(
		"printDouble".to_string(),
		Rc::new(|values| {
			print::print_double(values.get(0).unwrap().as_double());
			return Value::Null;
		}),
	);

	mapping.insert(
		"printInt".to_string(),
		Rc::new(|values| {
			print::print_int(values.get(0).unwrap().as_int());
			return Value::Null;
		}),
	);

	return mapping;
}

pub fn get_test_native_function_mapping_for_interpreter() -> HashMap<String, NativeFunction> {
	let mut mapping = get_native_function_mapping_for_interpreter();

	mapping.insert(
		"printDouble".to_string(),
		Rc::new(|values| {
			print::test_print_double(values.get(0).unwrap().as_double());
			return Value::Null;
		}),
	);

	mapping.insert(
		"printInt".to_string(),
		Rc::new(|values| {
			print::test_print_int(values.get(0).unwrap().as_int());
			return Value::Null;
		}),
	);

	return mapping;
}

pub fn get_native_function_mapping_for_jit() -> HashMap<String, usize> {
	let mut mapping: HashMap<String, usize> = HashMap::new();
	mapping.insert("printDouble".to_string(), print::print_double as usize);
	mapping.insert("printInt".to_string(), print::print_int as usize);
	return mapping;
}

pub fn get_test_native_function_mapping_for_jit() -> HashMap<String, usize> {
	let mut mapping: HashMap<String, usize> = HashMap::new();
	mapping.insert("printDouble".to_string(), print::test_print_double as usize);
	mapping.insert("printInt".to_string(), print::test_print_int as usize);
	return mapping;
}
