use anyhow::Result;
use eggscript_mir::{
	EggscriptLowerContext, Unit, UnitHandle, ValueHandle,
};
use eggscript_mir::{UnitStore, ValueStore};
use eggscript_types::P;

use crate::expressions::{Expression, ExpressionInfo};

pub struct AstLowerContext {
	pub unit_store: UnitStore,
	pub value_store: ValueStore,
}

impl Into<EggscriptLowerContext> for AstLowerContext {
	fn into(self) -> EggscriptLowerContext {
		EggscriptLowerContext::new(self.value_store)
	}
}

impl AstLowerContext {
	pub fn new() -> AstLowerContext {
		AstLowerContext {
			unit_store: UnitStore::new(),
			value_store: ValueStore::new(),
		}
	}

	pub fn lower_expression(
		&mut self,
		expression: &P<Expression>,
	) -> Result<(Vec<UnitHandle>, Option<ValueHandle>)> {
		match expression.info {
			ExpressionInfo::Assign(_, _, _) => self.lower_variable_assignment(expression),
			ExpressionInfo::Primitive(_) => todo!(),
			ExpressionInfo::Scope(_) => self.lower_scope(expression),
		}
	}
}

pub fn compile_file(expression: P<Expression>) -> Result<(AstLowerContext, Vec<Unit>)> {
	let mut lower_context = AstLowerContext::new();
	lower_context.lower_expression(&expression)?;
	let units = lower_context.unit_store.take_units();
	Ok((lower_context, units))
}
