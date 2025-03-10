use eggscript_types::{TypeHandle, TypeStore};

#[derive(Clone, Debug)]
pub enum PrimitiveValue {
	Number(f64),
}

impl Into<eggscript_interpreter::Value> for &PrimitiveValue {
	fn into(self) -> eggscript_interpreter::Value {
		match self {
			PrimitiveValue::Number(number) => eggscript_interpreter::Value::Number(*number),
		}
	}
}

impl std::fmt::Display for PrimitiveValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			PrimitiveValue::Number(value) => f.write_fmt(format_args!("{}", value)),
		}
	}
}

impl PrimitiveValue {
	pub fn get_type_from_type_store(&self, type_store: &TypeStore) -> TypeHandle {
		match self {
			PrimitiveValue::Number(_) => type_store.name_to_type_handle("number").unwrap(),
		}
	}
}
