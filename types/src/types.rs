use std::collections::HashMap;

#[derive(Debug)]
pub enum Primitive {
	Char,
	Double,
	U8,
	U16,
	U32,
	U64,
	I8,
	I16,
	I32,
	I64,
	String,
}

#[derive(Debug)]
pub enum KnownTypeInfo {
	Primitive(Primitive),
}

#[derive(Debug)]
pub enum Type {
	Known {
		id: TypeHandle,
		info: KnownTypeInfo,
		name: String,
	},
	Unknown {
		id: TypeHandle,
	},
}

impl Type {
	pub(crate) fn set_id(mut self, new_id: TypeHandle) -> Type {
		match &mut self {
			Type::Known { id, .. } => *id = new_id,
			Type::Unknown { id } => *id = new_id,
		}

		self
	}

	pub(crate) fn get_name(&self) -> Option<&str> {
		match self {
			Type::Known { name, .. } => Some(name),
			Type::Unknown { .. } => None,
		}
	}

	pub(crate) fn get_id(&self) -> TypeHandle {
		match self {
			Type::Known { id, .. } => *id,
			Type::Unknown { id, .. } => *id,
		}
	}
}

pub type TypeHandle = usize;

#[derive(Debug)]
pub struct TypeStore {
	name_to_type: HashMap<String, TypeHandle>,
	types: Vec<Type>,
}

impl TypeStore {
	pub fn new() -> TypeStore {
		let mut type_store = TypeStore {
			name_to_type: HashMap::new(),
			types: Vec::new(),
		};

		type_store.create_type(Type::Known {
			id: 0,
			info: KnownTypeInfo::Primitive(Primitive::Double),
			name: "double".into(),
		});

		type_store.create_type(Type::Known {
			id: 0,
			info: KnownTypeInfo::Primitive(Primitive::String),
			name: "string".into(),
		});

		return type_store;
	}

	pub fn create_type(&mut self, ty: Type) {
		let ty = ty.set_id(self.types.len());

		if let Some(name) = ty.get_name() {
			self.name_to_type.insert(name.into(), ty.get_id());
		}

		self.types.push(ty);
	}

	pub fn name_to_type(&self, name: &str) -> Option<&Type> {
		let handle = self.name_to_type_handle(name)?;
		self.types.get(*handle)
	}

	pub fn name_to_type_handle(&self, name: &str) -> Option<&TypeHandle> {
		self.name_to_type.get(name)
	}
}
