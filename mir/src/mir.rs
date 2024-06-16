use std::fmt::Write;

use eggscript_types::P;

use crate::unit::UnitHandle;
use crate::value::{PrimitiveValue, ValueHandle};

const INDENT: &str = "  ";

#[derive(Clone, Debug)]
pub struct MIR {
	pub(crate) info: MIRInfo,
}

impl std::fmt::Display for MIR {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self.info {
			MIRInfo::Allocate(value) => f.write_fmt(format_args!("{}%{};\n", INDENT, value)),
			MIRInfo::StoreLiteral(value, primitive_value) => {
				f.write_fmt(format_args!("{}%{} = {};\n", INDENT, value, primitive_value))
			}
		}
	}
}

impl MIR {
	pub fn new(info: MIRInfo) -> MIR {
		MIR { info }
	}
}

#[derive(Clone, Debug)]
pub enum MIRInfo {
	Allocate(ValueHandle),
	StoreLiteral(ValueHandle, PrimitiveValue),
}

#[derive(Debug)]
pub enum Transition {
	Goto(UnitHandle),
	GotoIfTrue(UnitHandle, ValueHandle),
	Invalid,
	Next,
	Return,
}

impl std::fmt::Display for Transition {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Transition::Goto(goto) => f.write_fmt(format_args!("{}goto {};", INDENT, goto)),
			Transition::GotoIfTrue(_, _) => todo!(),
			Transition::Invalid => f.write_fmt(format_args!("{}invalid;", INDENT)),
			Transition::Next => f.write_fmt(format_args!("{}next;", INDENT)),
			Transition::Return => f.write_fmt(format_args!("{}return;", INDENT)),
		}
	}
}
