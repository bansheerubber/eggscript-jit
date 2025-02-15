use std::rc::Rc;

use crate::{Instruction, Value};

pub type FunctionHandle = usize;

pub type NativeFunction = Rc<dyn Fn(Vec<Value>) -> Value>;

pub enum Function {
	Eggscript {
		argument_count: usize,
		id: FunctionHandle,
		instructions: Rc<Vec<Instruction>>,
		name: String,
	},
	Native {
		argument_count: usize,
		function: NativeFunction,
		id: FunctionHandle,
		name: String,
	},
}

impl Function {
	pub fn new_eggscript_function(
		id: FunctionHandle,
		argument_count: usize,
		instructions: Vec<Instruction>,
		name: &str,
	) -> Function {
		Function::Eggscript {
			argument_count,
			id,
			instructions: Rc::new(instructions),
			name: name.into(),
		}
	}

	pub fn new_native(
		id: FunctionHandle,
		argument_count: usize,
		function: NativeFunction,
		name: &str,
	) -> Function {
		Function::Native {
			argument_count,
			function,
			id,
			name: name.into(),
		}
	}
}
