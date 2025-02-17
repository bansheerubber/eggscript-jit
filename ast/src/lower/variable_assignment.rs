use anyhow::{Context, Result};
use eggscript_mir::{MIRInfo, Transition, UnitHandle, Value, MIR};
use eggscript_types::P;
use std::ops::Deref;

use crate::expressions::{Expression, ExpressionInfo};
use crate::lower::AstLowerContext;
use crate::BinaryOperator;

impl AstLowerContext {
	pub(crate) fn lower_variable_assignment(
		&mut self,
		expression: &P<Expression>,
	) -> Result<(Vec<UnitHandle>, Option<P<Value>>)> {
		let ExpressionInfo::Assign(name, operator, value) = &expression.info else {
			unreachable!();
		};

		let (variable_value, is_new) = self.value_store.new_location(
			name.name(),
			expression
				.ty
				.context("Variable assignment does not have type")?,
		);

		let (mut rvalue_units, rvalue) = self.lower_expression(value)?;
		let Some(rvalue) = rvalue else {
			unreachable!();
		};

		// is_new is only allowed with equal operator. otherwise, panic
		assert!(
			(!is_new && operator != &BinaryOperator::Equal) || operator == &BinaryOperator::Equal
		);

		let mut mir: Vec<MIR> = Vec::new();
		if is_new {
			mir.push(MIR::new(MIRInfo::Allocate(variable_value.clone(), None)));
		}

		let mut units = vec![];

		if operator == &BinaryOperator::Equal {
			match rvalue.deref() {
				Value::Location { .. } => {
					mir.push(MIR::new(MIRInfo::StoreValue(
						variable_value.clone(),
						rvalue,
					)));
				}
				Value::Primitive { value, .. } => {
					mir.push(MIR::new(MIRInfo::StoreLiteral(
						variable_value.clone(),
						value.clone(),
					)));
				}
				Value::Temp { .. } => {
					mir.push(MIR::new(MIRInfo::StoreValue(
						variable_value.clone(),
						rvalue,
					)));
				}
			}

			units.append(&mut rvalue_units);
			units.push(self.unit_store.new_unit(mir, Transition::Next));
		} else {
			let result = self.value_store.new_temp(variable_value.ty());
			mir.push(MIR::new(MIRInfo::BinaryOperation(
				result.clone(),
				variable_value.clone(),
				rvalue.clone(),
				operator.into(),
			)));

			mir.push(MIR::new(MIRInfo::StoreValue(
				variable_value.clone(),
				result,
			)));

			units.append(&mut rvalue_units);
			units.push(self.unit_store.new_unit(mir, Transition::Next));
		}

		Ok((units, Some(variable_value)))
	}
}
