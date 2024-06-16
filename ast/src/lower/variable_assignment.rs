use anyhow::{Context, Result};
use eggscript_mir::{MIRInfo, PrimitiveValue, Transition, UnitHandle, ValueHandle, MIR};
use eggscript_types::P;

use crate::expressions::{Expression, ExpressionInfo};
use crate::lower::AstLowerContext;

impl AstLowerContext {
	pub(crate) fn lower_variable_assignment(
		&mut self,
		expression: &P<Expression>,
	) -> Result<(Vec<UnitHandle>, Option<ValueHandle>)> {
		let ExpressionInfo::Assign(name, _, value) = &expression.info else {
			unreachable!();
		};

		let variable_value = self.value_store.new_location(
			name.name(),
			expression
				.ty
				.context("Variable assignment does not have type")?,
		);

		let primitive_value = self.lower_number_literal(value)?;

		let mir1 = MIR::new(MIRInfo::Allocate(variable_value));
		let mir2 = MIR::new(MIRInfo::StoreLiteral(variable_value, primitive_value));

		let unit = self
			.unit_store
			.new_unit(vec![mir1, mir2], Transition::Next);

		Ok((vec![unit], Some(variable_value)))
	}

	pub(crate) fn lower_number_literal(
		&mut self,
		expression: &Expression,
	) -> Result<PrimitiveValue> {
		let ExpressionInfo::Primitive(value) = &expression.info else {
			unreachable!();
		};

		Ok(PrimitiveValue::Double(
			value.parse::<f64>().context(format!("Could not parse f64 {}", value))?,
		))
	}
}
