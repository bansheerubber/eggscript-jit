use eggscript_types::P;
use std::ops::Deref;

use crate::{BinaryOperator, PrimitiveValue, UnitHandle, Value};

const INDENT: &str = "  ";

#[derive(Debug)]
pub struct MIR {
	pub(crate) info: MIRInfo,
}

impl std::fmt::Display for MIR {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self.info {
			MIRInfo::Allocate(value, _) => {
				f.write_fmt(format_args!("{}+%{};\n", INDENT, value.id()))
			}
			MIRInfo::BinaryOperation(result, lvalue, rvalue, operator) => {
				f.write_fmt(format_args!(
					"{}{} = {} {} {};\n",
					INDENT,
					result.deref(),
					lvalue.deref(),
					operator,
					rvalue.deref(),
				))
			}
			MIRInfo::CallFunction(function_name, function_handle, arguments, result) => {
				f.write_fmt(format_args!(
					"{}{} = {}[{}](",
					INDENT,
					result.deref(),
					function_name,
					function_handle,
				))?;

				let mut index = 0;
				let length = arguments.len();
				for argument in arguments.iter() {
					if index == length - 1 {
						f.write_fmt(format_args!("{}", argument.deref()))?;
					} else {
						f.write_fmt(format_args!("{}, ", argument.deref()))?;
					}

					index += 1;
				}

				f.write_str(");\n")
			}
			MIRInfo::StoreLiteral(value, primitive_value) => f.write_fmt(format_args!(
				"{}{} = #{};\n",
				INDENT,
				value.deref(),
				primitive_value
			)),
			MIRInfo::StoreValue(value, rvalue) => f.write_fmt(format_args!(
				"{}{} = {};\n",
				INDENT,
				value.deref(),
				rvalue.deref()
			)),
		}
	}
}

impl MIR {
	pub fn new(info: MIRInfo) -> MIR {
		MIR { info }
	}
}

#[derive(Debug)]
pub enum MIRInfo {
	Allocate(P<Value>, Option<usize>),
	BinaryOperation(P<Value>, P<Value>, P<Value>, BinaryOperator),
	CallFunction(String, usize, Vec<P<Value>>, P<Value>),
	StoreLiteral(P<Value>, PrimitiveValue),
	StoreValue(P<Value>, P<Value>),
}

#[derive(Clone, Debug)]
pub enum Transition {
	Goto(UnitHandle),
	GotoIfFalse(UnitHandle, P<Value>),
	GotoIfTrue(UnitHandle, P<Value>),
	Invalid,
	Next,
	Return(Option<P<Value>>),
}

impl Transition {
	pub fn jump_target(&self) -> Option<UnitHandle> {
		match self {
			Transition::Goto(target) => Some(*target),
			Transition::GotoIfFalse(target, _) => Some(*target),
			Transition::GotoIfTrue(target, _) => Some(*target),
			_ => None,
		}
	}
}

impl std::fmt::Display for Transition {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Transition::Goto(goto) => f.write_fmt(format_args!("{}goto {};", INDENT, goto)),
			Transition::GotoIfFalse(goto, value) => f.write_fmt(format_args!(
				"{}goto {} if {} == false",
				INDENT,
				goto,
				value.deref(),
			)),
			Transition::GotoIfTrue(goto, value) => f.write_fmt(format_args!(
				"{}goto {} if {} == true",
				INDENT,
				goto,
				value.deref(),
			)),
			Transition::Invalid => f.write_fmt(format_args!("{}invalid;", INDENT)),
			Transition::Next => f.write_fmt(format_args!("{}next;", INDENT)),
			Transition::Return(value) => {
				if let Some(value) = value {
					f.write_fmt(format_args!("{}return {};", INDENT, value.deref()))
				} else {
					f.write_fmt(format_args!("{}return;", INDENT))
				}
			}
		}
	}
}
