use eggscript_types::TypeStore;

pub struct AstContext {
	pub type_store: TypeStore,
}

impl AstContext {
	pub fn new() -> AstContext {
		AstContext {
			type_store: TypeStore::new(),
		}
	}
}
