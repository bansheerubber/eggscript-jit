use std::{collections::HashMap, rc::Rc};

use super::print;
use crate::function::NativeFunction;

pub fn get_native_function_mapping_for_interpreter() -> HashMap<String, NativeFunction> {
	let mut mapping: HashMap<String, NativeFunction> = HashMap::new();
	mapping.insert("printDouble".to_string(), Rc::new(print::print_double));
	return mapping;
}

pub fn get_test_native_function_mapping_for_interpreter() -> HashMap<String, NativeFunction> {
	let mut mapping = get_native_function_mapping_for_interpreter();
	mapping.insert("printDouble".to_string(), Rc::new(print::test_print));
	return mapping;
}

pub fn get_native_function_mapping_for_jit() -> HashMap<String, usize> {
	let mut mapping: HashMap<String, usize> = HashMap::new();
	mapping.insert("printDouble".to_string(), print::print_double2 as usize);
	return mapping;
}

pub fn get_test_native_function_mapping_for_jit() -> HashMap<String, usize> {
	let mut mapping: HashMap<String, usize> = HashMap::new();
	mapping.insert("printDouble".to_string(), print::test_print2 as usize);
	return mapping;
}
