use crate::TypeHandle;

#[derive(Debug)]
pub struct FunctionType {
	pub argument_types: Vec<TypeHandle>,
	pub id: TypeHandle,
	pub name: String,
	pub return_type: TypeHandle,
}
