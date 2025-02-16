use std::{collections::HashMap, rc::Rc};

use super::print;
use crate::function::NativeFunction;

pub fn get_native_function_mapping() -> HashMap<String, NativeFunction> {
	let mut mapping: HashMap<String, NativeFunction> = HashMap::new();
	mapping.insert("printDouble".to_string(), Rc::new(print::print_double));
	return mapping;
}

pub fn get_test_native_function_mapping() -> HashMap<String, NativeFunction> {
	let mut mapping = get_native_function_mapping();
	mapping.insert("printDouble".to_string(), Rc::new(print::test_print));
	return mapping;
}
