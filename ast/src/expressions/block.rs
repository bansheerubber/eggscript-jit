use eggscript_types::P;

use crate::expressions::Expression;
use crate::Span;

#[derive(Debug)]
pub struct Block {
	pub(crate) expressions: Vec<P<Expression>>,
	pub(crate) span: Span,
}

impl Block {
	pub fn expressions(&self) -> &[P<Expression>] {
		&self.expressions
	}
}
