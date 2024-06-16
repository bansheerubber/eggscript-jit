use std::fmt::Write;

use eggscript_types::TypeHandle;

pub type ValueHandle = usize;

#[derive(Debug)]
pub enum Value {
	Location {
		name: String,
		id: ValueHandle,
		ty: TypeHandle,
	},
	Temp {
		id: ValueHandle,
		ty: TypeHandle,
	},
}

pub struct ValueStore {
	values: Vec<Value>,
}

impl ValueStore {
	pub fn new() -> ValueStore {
		ValueStore { values: vec![] }
	}

	pub fn new_location(&mut self, name: &str, ty: TypeHandle) -> ValueHandle {
		let id = self.values.len();

		self.values.push(Value::Location {
			name: name.into(),
			id,
			ty,
		});

		return id;
	}
}

#[derive(Clone, Debug)]
pub enum PrimitiveValue {
	Double(f64),
	Integer(i64),
	String(String),
}

impl Into<eggscript_interpreter::Value> for &PrimitiveValue {
	fn into(self) -> eggscript_interpreter::Value {
		match self {
			PrimitiveValue::Double(number) => eggscript_interpreter::Value::Double(*number),
			PrimitiveValue::Integer(number) => eggscript_interpreter::Value::Integer(*number),
			PrimitiveValue::String(_) => todo!(),
		}
	}
}

impl std::fmt::Display for PrimitiveValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			PrimitiveValue::Double(value) => f.write_fmt(format_args!("{}", value)),
			PrimitiveValue::Integer(value) => f.write_fmt(format_args!("{}", value)),
			PrimitiveValue::String(value) => f.write_fmt(format_args!("{}", value)),
		}
	}
}
