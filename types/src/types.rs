use std::collections::HashMap;

use crate::FunctionType;

#[derive(Clone, Debug)]
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
	Null,
}

#[derive(Debug)]
pub enum KnownTypeInfo {
	Primitive(Primitive),
}

#[derive(Debug)]
pub enum Type {
	FunctionReturn {
		id: TypeHandle,
		function_name: String,
	},
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
			Type::FunctionReturn { id, .. } => *id = new_id,
			Type::Known { id, .. } => *id = new_id,
			Type::Unknown { id } => *id = new_id,
		}

		self
	}

	pub(crate) fn get_name(&self) -> Option<&str> {
		match self {
			Type::FunctionReturn { function_name, .. } => Some(function_name),
			Type::Known { name, .. } => Some(name),
			Type::Unknown { .. } => None,
		}
	}

	pub(crate) fn get_id(&self) -> TypeHandle {
		match self {
			Type::FunctionReturn { id, .. } => *id,
			Type::Known { id, .. } => *id,
			Type::Unknown { id, .. } => *id,
		}
	}
}

pub type TypeHandle = usize;

#[derive(Debug)]
pub struct TypeStore {
	functions: HashMap<String, FunctionType>,
	name_to_type: HashMap<String, TypeHandle>,
	types: Vec<Type>,
}

impl TypeStore {
	pub fn new() -> TypeStore {
		let mut type_store = TypeStore {
			functions: HashMap::new(),
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

		type_store.create_type(Type::Known {
			id: 0,
			info: KnownTypeInfo::Primitive(Primitive::Null),
			name: "null".into(),
		});

		return type_store;
	}

	pub fn create_type(&mut self, ty: Type) -> TypeHandle {
		let ty = ty.set_id(self.types.len());

		if let Some(name) = ty.get_name() {
			self.name_to_type.insert(name.into(), ty.get_id());
		}

		let id = ty.get_id();
		self.types.push(ty);

		return id;
	}

	pub fn create_function_type(
		&mut self,
		name: &str,
		argument_types: Vec<TypeHandle>,
		return_type: Option<TypeHandle>,
	) -> FunctionType {
		let ty = FunctionType {
			argument_types,
			id: 0, // TODO ugh
			name: name.into(),
			return_type,
		};

		self.functions.insert(name.into(), ty.clone());

		return ty;
	}

	pub fn create_unknown(&mut self) -> TypeHandle {
		let type_handle = self.types.len();
		let ty = Type::Unknown { id: type_handle };

		self.types.push(ty);

		return type_handle;
	}

	pub fn get_function(&self, name: &str) -> Option<&FunctionType> {
		self.functions.get(name)
	}

	pub fn name_to_type(&self, name: &str) -> Option<&Type> {
		let handle = self.name_to_type_handle(name)?;
		self.types.get(handle)
	}

	pub fn name_to_type_handle(&self, name: &str) -> Option<TypeHandle> {
		self.name_to_type.get(name).copied()
	}

	pub fn resolve_type(&self, ty: TypeHandle) -> Option<TypeHandle> {
		let ty = self.types.get(ty).unwrap();
		match ty {
			Type::FunctionReturn { function_name, .. } => {
				let function = self.functions.get(function_name).unwrap();
				function.return_type
			}
			Type::Known { id, .. } => Some(*id),
			Type::Unknown { id } => Some(*id), // TODO
		}
	}

	pub fn get_type(&self, ty: TypeHandle) -> Option<&Type> {
		self.types.get(ty)
	}

	pub fn are_types_compatible(&self, type1: TypeHandle, type2: TypeHandle) -> bool {
		let type1 = self.resolve_type(type1);
		let type2 = self.resolve_type(type2);
		return type1 == type2;
	}
}
