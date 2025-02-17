use std::sync::{Arc, Mutex};

use eggscript_types::TypeStore;

pub struct AstContext {
	pub type_store: Arc<Mutex<TypeStore>>,
}

impl AstContext {
	pub fn new(type_store: Arc<Mutex<TypeStore>>) -> AstContext {
		AstContext { type_store }
	}
}
