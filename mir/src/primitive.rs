use eggscript_types::{TypeHandle, TypeStore};

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

impl PrimitiveValue {
	pub fn get_type_from_type_store(&self, type_store: &TypeStore) -> TypeHandle {
		match self {
			PrimitiveValue::Double(_) => type_store.name_to_type_handle("double").unwrap(),
			PrimitiveValue::Integer(_) => type_store.name_to_type_handle("int").unwrap(),
			PrimitiveValue::String(_) => type_store.name_to_type_handle("string").unwrap(),
		}
	}
}
