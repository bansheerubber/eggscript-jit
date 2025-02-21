use anyhow::{Context, Result};
use eggscript_mir::{MIRInfo, Transition, UnitHandle, Value, MIR};
use eggscript_types::P;
use std::ops::Deref;

use crate::expressions::{Expression, ExpressionInfo};
use crate::lower::AstLowerContext;

impl AstLowerContext {
	pub(crate) fn lower_function_call(
		&mut self,
		expression: &P<Expression>,
	) -> Result<(Vec<UnitHandle>, Option<P<Value>>)> {
		let ExpressionInfo::FunctionCall(name, arguments) = &expression.info else {
			unreachable!();
		};

		let function = self
			.program
			.function_name_to_function
			.get(name.name())
			.context(format!("Could not get function '{}'", name.name()))?
			.clone();

		let mut units = vec![];
		let mut argument_values = vec![];
		for argument in arguments.iter() {
			let (mut argument_units, value) = self.lower_expression(argument)?;
			let mut extra_unit = None;
			if let Some(value) = &value {
				match value.deref() {
					Value::Location { ty, .. } => {
						let temp = self.value_store.new_temp(*ty);
						extra_unit = Some(self.unit_store.new_unit(
							vec![MIR::new(
								MIRInfo::StoreValue(temp.clone(), value.clone()),
								name.span(),
							)],
							Transition::Next,
						));

						argument_values.push(temp);
					}
					Value::Primitive { ty, value, .. } => {
						let temp = self.value_store.new_temp(*ty);
						extra_unit = Some(self.unit_store.new_unit(
							vec![MIR::new(
								MIRInfo::StoreLiteral(temp.clone(), value.clone()),
								name.span(),
							)],
							Transition::Next,
						));

						argument_values.push(temp);
					}
					_ => argument_values.push(value.clone()),
				}
			} else {
				unreachable!()
			}

			units.append(&mut argument_units);

			if let Some(extra_unit) = extra_unit {
				units.push(extra_unit);
			}
		}

		let result = self.value_store.new_temp(
			function.return_ty.unwrap_or(
				self.program
					.type_store
					.lock()
					.unwrap()
					.name_to_type_handle("null")
					.unwrap(),
			),
		);

		units.push(self.unit_store.new_unit(
			vec![MIR::new(
				MIRInfo::CallFunction(
					function.name.clone(),
					function.id,
					argument_values,
					result.clone(),
				),
				name.span(),
			)],
			Transition::Next,
		));

		Ok((units, Some(result)))
	}
}
