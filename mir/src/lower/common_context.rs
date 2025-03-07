use eggscript_types::{FunctionType, TypeHandle, TypeStore};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::{MIRInfo, Span, Transition, Unit};

pub struct CommonContext {
	pub file_name: String,
	pub type_store: Arc<Mutex<TypeStore>>,
	pub value_used_by: HashMap<usize, Vec<usize>>,
}

impl CommonContext {
	pub fn new(type_store: Arc<Mutex<TypeStore>>, file_name: &str) -> Self {
		CommonContext {
			file_name: file_name.into(),
			type_store,
			value_used_by: HashMap::new(),
		}
	}

	pub fn type_check_units(&mut self, units: &Vec<Unit>, function: Option<&FunctionType>) {
		let type_store = self.type_store.lock().unwrap();
		for unit in units.iter() {
			match &unit.transition {
				Transition::Return(value) => {
					assert!(function.is_some(), "return found in non-function unit");

					let function = function.unwrap();

					assert!(
						function.return_type.is_some() == value.is_some(),
						"malformed return statement"
					);

					if let Some(value) = value {
						assert!(
							type_store
								.are_types_compatible(function.return_type.unwrap(), value.ty()),
							"return types not compatible"
						);
					}
				}
				_ => {}
			}

			for mir in unit.mir.iter() {
				match &mir.info {
					MIRInfo::Allocate(_, _) => {}
					MIRInfo::BinaryOperation(result, left, right, _) => {
						self.type_check(
							&type_store,
							result.ty(),
							left.ty(),
							&mir.span,
							"result not compatible with left",
						);

						self.type_check(
							&type_store,
							result.ty(),
							right.ty(),
							&mir.span,
							"result not compatible with right",
						);

						self.type_check(
							&type_store,
							left.ty(),
							right.ty(),
							&mir.span,
							"left not compatible with right",
						);
					}
					MIRInfo::CallFunction(function_name, _, arguments, _) => {
						let mut index = 0;
						let function = type_store.get_function(function_name).unwrap();
						for argument in arguments.iter() {
							self.type_check(
								&type_store,
								argument.ty(),
								*function.argument_types.get(index).unwrap(),
								&mir.span,
								&format!("argument #{} not compatible with value", index),
							);
							index += 1;
						}
					}
					MIRInfo::StoreLiteral(lvalue, rvalue) => {
						self.type_check(
							&type_store,
							lvalue.ty(),
							rvalue.get_type_from_type_store(&type_store),
							&mir.span,
							"lvalue not compatible with rvalue",
						);
					}
					MIRInfo::StoreValue(lvalue, rvalue) => {
						self.type_check(
							&type_store,
							lvalue.ty(),
							rvalue.ty(),
							&mir.span,
							"lvalue not compatible with rvalue",
						);
					}
					MIRInfo::Unary(_, _, _) => {}
				}
			}
		}
	}

	fn type_check(
		&self,
		type_store: &TypeStore,
		type1: TypeHandle,
		type2: TypeHandle,
		span: &Span,
		message: &str,
	) {
		if !type_store.are_types_compatible(type1, type2) {
			println!("{} ({} != {})", message, type1, type2);
			println!("{}", self.print_span(span));
			panic!();
		}
	}

	pub fn build_value_dependencies(&mut self, units: &Vec<Unit>) {
		for unit in units.iter() {
			for mir in unit.mir.iter() {
				match &mir.info {
					MIRInfo::BinaryOperation(lvalue, operand1, operand2, _) => {
						self.value_used_by
							.entry(operand1.id())
							.or_default()
							.push(lvalue.id());

						self.value_used_by
							.entry(operand2.id())
							.or_default()
							.push(lvalue.id());
					}
					MIRInfo::CallFunction(_, _, arguments, result) => {
						for argument in arguments.iter() {
							self.value_used_by
								.entry(argument.id())
								.or_default()
								.push(result.id());
						}
					}
					MIRInfo::StoreValue(lvalue, rvalue) => {
						self.value_used_by
							.entry(rvalue.id())
							.or_default()
							.push(lvalue.id());
					}
					MIRInfo::Unary(result, lvalue, _) => {
						self.value_used_by
							.entry(lvalue.id())
							.or_default()
							.push(result.id());
					}
					_ => {}
				}
			}
		}
	}

	fn print_span(&self, span: &Span) -> String {
		let contents = std::fs::read_to_string(&self.file_name).unwrap();
		contents[span.start() as usize..span.end() as usize].into()
	}
}
