use eggscript_types::{TypeHandle, P};
use std::collections::HashMap;

use crate::PrimitiveValue;

#[derive(Clone, Debug)]
pub enum Value {
	Location {
		id: usize,
		name: String,
		ty: TypeHandle,
	},
	Primitive {
		id: usize,
		ty: TypeHandle,
		value: PrimitiveValue,
	},
	Temp {
		id: usize,
		ty: TypeHandle,
	},
}

impl Value {
	pub fn id(&self) -> usize {
		match self {
			Value::Location { id, .. } => *id,
			Value::Primitive { id, .. } => *id,
			Value::Temp { id, .. } => *id,
		}
	}

	pub fn ty(&self) -> usize {
		match self {
			Value::Location { ty, .. } => *ty,
			Value::Primitive { ty, .. } => *ty,
			Value::Temp { ty, .. } => *ty,
		}
	}
}

pub struct ValueStore {
	name_to_value: HashMap<String, P<Value>>,
	values: Vec<P<Value>>,
}

impl ValueStore {
	pub fn new() -> ValueStore {
		ValueStore {
			name_to_value: HashMap::new(),
			values: vec![],
		}
	}

	pub fn new_location(&mut self, name: &str, ty: TypeHandle) -> (P<Value>, bool) {
		// TODO typecheck this
		if let Some(value) = self.name_to_value.get(name) {
			return (value.clone(), false);
		}

		let id = self.values.len();

		let value = P::new(Value::Location {
			name: name.into(),
			id,
			ty,
		});

		self.name_to_value.insert(name.into(), value.clone());
		self.values.push(value.clone());

		return (value, true);
	}

	pub fn new_temp(&mut self, ty: TypeHandle) -> P<Value> {
		let id = self.values.len();

		let value = P::new(Value::Temp { id, ty });
		self.values.push(value.clone());
		return value;
	}

	pub fn new_primitive(&mut self, ty: TypeHandle, value: PrimitiveValue) -> P<Value> {
		let id = self.values.len();
		let value = P::new(Value::Primitive { id, ty, value });
		self.values.push(value.clone());
		return value;
	}

	pub fn values(&self) -> impl Iterator<Item = &P<Value>> {
		self.values.iter()
	}
}

impl std::fmt::Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Value::Location { id, ty, .. } => f.write_fmt(format_args!("%{}({})", id, ty)),
			Value::Primitive { value, ty, .. } => f.write_fmt(format_args!("#{}({})", value, ty)),
			Value::Temp { id, ty, .. } => f.write_fmt(format_args!("@{}({})", id, ty)),
		}
	}
}
