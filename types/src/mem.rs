use std::{ops::Deref, rc::Rc};

#[derive(Debug)]
pub struct P<T: ?Sized> {
	ptr: Rc<T>,
}

impl<T> P<T> {
	pub fn new(value: T) -> P<T> {
		P {
			ptr: Rc::new(value),
		}
	}
}

impl<T: ?Sized> Deref for P<T> {
	type Target = T;

	fn deref(&self) -> &T {
		&self.ptr
	}
}
