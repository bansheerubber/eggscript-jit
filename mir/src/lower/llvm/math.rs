use anyhow::Result;
use eggscript_types::{KnownTypeInfo, Primitive, P};
use inkwell::{
	values::{BasicValueEnum, FloatValue, InstructionOpcode, IntValue},
	FloatPredicate,
};

use crate::{BinaryOperator, Value};

use super::LlvmLowerContext;

impl<'a, 'ctx> LlvmLowerContext<'a, 'ctx> {
	fn binary_operator_to_float_cmp(&self, operator: &BinaryOperator) -> FloatPredicate {
		match operator {
			BinaryOperator::Equal => FloatPredicate::OEQ,
			BinaryOperator::NotEqual => FloatPredicate::ONE,
			BinaryOperator::LessThan => FloatPredicate::OLT,
			BinaryOperator::GreaterThan => FloatPredicate::OGT,
			BinaryOperator::LessThanEqualTo => FloatPredicate::OLE,
			BinaryOperator::GreaterThanEqualTo => FloatPredicate::OGE,
			_ => unreachable!(),
		}
	}

	pub fn build_double_to_int_cast(&self, value: &P<Value>) -> Result<IntValue<'ctx>> {
		Ok(self
			.builder
			.build_cast(
				InstructionOpcode::FPToSI,
				self.value_to_llvm_float_value(value)?,
				self.context.i64_type(),
				"fptosi_cast_",
			)?
			.into_int_value())
	}

	pub fn build_int_to_double_cast(&self, value: IntValue<'ctx>) -> Result<FloatValue<'ctx>> {
		Ok(self
			.builder
			.build_cast(
				InstructionOpcode::SIToFP,
				value,
				self.context.f64_type(),
				"sitofp_cast_",
			)?
			.into_float_value())
	}

	pub fn build_add(
		&mut self,
		result_value: &P<Value>,
		left_operand: &P<Value>,
		right_operand: &P<Value>,
	) -> Result<BasicValueEnum<'ctx>> {
		let type_store = self.common_context.type_store.lock().unwrap();
		let result_type = type_store.get_type(result_value.ty()).unwrap();
		let Some(info) = result_type.get_info() else {
			unreachable!();
		};

		match info {
			KnownTypeInfo::Primitive(primitive) => match primitive {
				Primitive::Char => todo!(),
				Primitive::Number => Ok(self
					.builder
					.build_float_add(
						self.value_to_llvm_float_value(left_operand)?,
						self.value_to_llvm_float_value(right_operand)?,
						&format!("add_result{}_", result_value.id()),
					)?
					.into()),
				Primitive::String => todo!(),
				Primitive::Null => todo!(),
			},
		}
	}

	pub fn build_sub(
		&mut self,
		result_value: &P<Value>,
		left_operand: &P<Value>,
		right_operand: &P<Value>,
	) -> Result<BasicValueEnum<'ctx>> {
		let type_store = self.common_context.type_store.lock().unwrap();
		let result_type = type_store.get_type(result_value.ty()).unwrap();
		let Some(info) = result_type.get_info() else {
			unreachable!();
		};

		match info {
			KnownTypeInfo::Primitive(primitive) => match primitive {
				Primitive::Char => todo!(),
				Primitive::Number => Ok(self
					.builder
					.build_float_sub(
						self.value_to_llvm_float_value(left_operand)?,
						self.value_to_llvm_float_value(right_operand)?,
						&format!("sub_result{}_", result_value.id()),
					)?
					.into()),
				Primitive::String => todo!(),
				Primitive::Null => todo!(),
			},
		}
	}

	pub fn build_mul(
		&mut self,
		result_value: &P<Value>,
		left_operand: &P<Value>,
		right_operand: &P<Value>,
	) -> Result<BasicValueEnum<'ctx>> {
		let type_store = self.common_context.type_store.lock().unwrap();
		let result_type = type_store.get_type(result_value.ty()).unwrap();
		let Some(info) = result_type.get_info() else {
			unreachable!();
		};

		match info {
			KnownTypeInfo::Primitive(primitive) => match primitive {
				Primitive::Char => todo!(),
				Primitive::Number => Ok(self
					.builder
					.build_float_mul(
						self.value_to_llvm_float_value(left_operand)?,
						self.value_to_llvm_float_value(right_operand)?,
						&format!("mul_result{}_", result_value.id()),
					)?
					.into()),
				Primitive::String => todo!(),
				Primitive::Null => todo!(),
			},
		}
	}

	pub fn build_div(
		&mut self,
		result_value: &P<Value>,
		left_operand: &P<Value>,
		right_operand: &P<Value>,
	) -> Result<BasicValueEnum<'ctx>> {
		let type_store = self.common_context.type_store.lock().unwrap();
		let result_type = type_store.get_type(result_value.ty()).unwrap();
		let Some(info) = result_type.get_info() else {
			unreachable!();
		};

		match info {
			KnownTypeInfo::Primitive(primitive) => match primitive {
				Primitive::Char => todo!(),
				Primitive::Number => Ok(self
					.builder
					.build_float_div(
						self.value_to_llvm_float_value(left_operand)?,
						self.value_to_llvm_float_value(right_operand)?,
						&format!("div_result{}_", result_value.id()),
					)?
					.into()),
				Primitive::String => todo!(),
				Primitive::Null => todo!(),
			},
		}
	}

	pub fn build_rem(
		&mut self,
		result_value: &P<Value>,
		left_operand: &P<Value>,
		right_operand: &P<Value>,
	) -> Result<BasicValueEnum<'ctx>> {
		let type_store = self.common_context.type_store.lock().unwrap();
		let result_type = type_store.get_type(result_value.ty()).unwrap();
		let Some(info) = result_type.get_info() else {
			unreachable!();
		};

		match info {
			KnownTypeInfo::Primitive(primitive) => match primitive {
				Primitive::Char => todo!(),
				Primitive::Number => Ok(self
					.builder
					.build_float_rem(
						self.value_to_llvm_float_value(left_operand)?,
						self.value_to_llvm_float_value(right_operand)?,
						&format!("mod_result{}_", result_value.id()),
					)?
					.into()),
				Primitive::String => todo!(),
				Primitive::Null => todo!(),
			},
		}
	}

	pub fn build_cmp(
		&mut self,
		result_value: &P<Value>,
		left_operand: &P<Value>,
		right_operand: &P<Value>,
		op: &BinaryOperator,
	) -> Result<BasicValueEnum<'ctx>> {
		let type_store = self.common_context.type_store.lock().unwrap();
		let result_type = type_store.get_type(result_value.ty()).unwrap();
		let Some(info) = result_type.get_info() else {
			unreachable!();
		};

		let result = match info {
			KnownTypeInfo::Primitive(primitive) => match primitive {
				Primitive::Char => todo!(),
				Primitive::Number => self.builder.build_float_compare(
					self.binary_operator_to_float_cmp(op),
					self.value_to_llvm_float_value(left_operand)?,
					self.value_to_llvm_float_value(right_operand)?,
					&format!("cmp_result{}_", result_value.id()),
				)?,
				Primitive::String => todo!(),
				Primitive::Null => todo!(),
			},
		};

		return Ok(self
			.builder
			.build_int_z_extend(result, self.context.i64_type(), "z_extend_")?
			.into());
	}

	pub fn build_bitwise_and(
		&mut self,
		result_value: &P<Value>,
		lvalue: &P<Value>,
		rvalue: &P<Value>,
	) -> Result<BasicValueEnum<'ctx>> {
		let type_store = self.common_context.type_store.lock().unwrap();
		let result_type = type_store.get_type(result_value.ty()).unwrap();
		let Some(info) = result_type.get_info() else {
			unreachable!();
		};

		match info {
			KnownTypeInfo::Primitive(primitive) => match primitive {
				Primitive::Char => todo!(),
				Primitive::Number => {
					let lvalue = self.build_double_to_int_cast(lvalue)?;
					let rvalue = self.build_double_to_int_cast(rvalue)?;

					let result = self.builder.build_and(lvalue, rvalue, "bitwise_and_")?;

					Ok(self.build_int_to_double_cast(result)?.into())
				}
				Primitive::String => todo!(),
				Primitive::Null => todo!(),
			},
		}
	}

	pub fn build_bitwise_or(
		&mut self,
		result_value: &P<Value>,
		lvalue: &P<Value>,
		rvalue: &P<Value>,
	) -> Result<BasicValueEnum<'ctx>> {
		let type_store = self.common_context.type_store.lock().unwrap();
		let result_type = type_store.get_type(result_value.ty()).unwrap();
		let Some(info) = result_type.get_info() else {
			unreachable!();
		};

		match info {
			KnownTypeInfo::Primitive(primitive) => match primitive {
				Primitive::Char => todo!(),
				Primitive::Number => {
					let lvalue = self.build_double_to_int_cast(lvalue)?;
					let rvalue = self.build_double_to_int_cast(rvalue)?;

					let result = self.builder.build_or(lvalue, rvalue, "bitwise_or_")?;

					Ok(self.build_int_to_double_cast(result)?.into())
				}
				Primitive::String => todo!(),
				Primitive::Null => todo!(),
			},
		}
	}

	pub fn build_bitwise_xor(
		&mut self,
		result_value: &P<Value>,
		lvalue: &P<Value>,
		rvalue: &P<Value>,
	) -> Result<BasicValueEnum<'ctx>> {
		let type_store = self.common_context.type_store.lock().unwrap();
		let result_type = type_store.get_type(result_value.ty()).unwrap();
		let Some(info) = result_type.get_info() else {
			unreachable!();
		};

		match info {
			KnownTypeInfo::Primitive(primitive) => match primitive {
				Primitive::Char => todo!(),
				Primitive::Number => {
					let lvalue = self.build_double_to_int_cast(lvalue)?;
					let rvalue = self.build_double_to_int_cast(rvalue)?;

					let result = self.builder.build_xor(lvalue, rvalue, "bitwise_xor_")?;

					Ok(self.build_int_to_double_cast(result)?.into())
				}
				Primitive::String => todo!(),
				Primitive::Null => todo!(),
			},
		}
	}

	pub fn build_shift_left(
		&mut self,
		result_value: &P<Value>,
		lvalue: &P<Value>,
		rvalue: &P<Value>,
	) -> Result<BasicValueEnum<'ctx>> {
		let type_store = self.common_context.type_store.lock().unwrap();
		let result_type = type_store.get_type(result_value.ty()).unwrap();
		let Some(info) = result_type.get_info() else {
			unreachable!();
		};

		match info {
			KnownTypeInfo::Primitive(primitive) => match primitive {
				Primitive::Char => todo!(),
				Primitive::Number => {
					let lvalue = self.build_double_to_int_cast(lvalue)?;
					let rvalue = self.build_double_to_int_cast(rvalue)?;

					let result = self
						.builder
						.build_left_shift(lvalue, rvalue, "shift_left")?;

					Ok(self.build_int_to_double_cast(result)?.into())
				}
				Primitive::String => todo!(),
				Primitive::Null => todo!(),
			},
		}
	}

	pub fn build_shift_right(
		&mut self,
		result_value: &P<Value>,
		lvalue: &P<Value>,
		rvalue: &P<Value>,
	) -> Result<BasicValueEnum<'ctx>> {
		let type_store = self.common_context.type_store.lock().unwrap();
		let result_type = type_store.get_type(result_value.ty()).unwrap();
		let Some(info) = result_type.get_info() else {
			unreachable!();
		};

		match info {
			KnownTypeInfo::Primitive(primitive) => match primitive {
				Primitive::Char => todo!(),
				Primitive::Number => {
					let lvalue = self.build_double_to_int_cast(lvalue)?;
					let rvalue = self.build_double_to_int_cast(rvalue)?;

					let result =
						self.builder
							.build_right_shift(lvalue, rvalue, false, "shift_right")?;

					Ok(self.build_int_to_double_cast(result)?.into())
				}
				Primitive::String => todo!(),
				Primitive::Null => todo!(),
			},
		}
	}

	pub fn build_bitwise_not(
		&mut self,
		result_value: &P<Value>,
		rvalue: &P<Value>,
	) -> Result<BasicValueEnum<'ctx>> {
		let type_store = self.common_context.type_store.lock().unwrap();
		let result_type = type_store.get_type(result_value.ty()).unwrap();
		let Some(info) = result_type.get_info() else {
			unreachable!();
		};

		match info {
			KnownTypeInfo::Primitive(primitive) => match primitive {
				Primitive::Char => todo!(),
				Primitive::Number => {
					let not_result = self.builder.build_not(
						self.build_double_to_int_cast(rvalue)?,
						&format!("not_result{}_", result_value.id()),
					)?;

					Ok(self.build_int_to_double_cast(not_result)?.into())
				}
				Primitive::String => todo!(),
				Primitive::Null => todo!(),
			},
		}
	}

	pub fn build_neg(
		&mut self,
		result_value: &P<Value>,
		rvalue: &P<Value>,
	) -> Result<BasicValueEnum<'ctx>> {
		let type_store = self.common_context.type_store.lock().unwrap();
		let result_type = type_store.get_type(result_value.ty()).unwrap();
		let Some(info) = result_type.get_info() else {
			unreachable!();
		};

		match info {
			KnownTypeInfo::Primitive(primitive) => match primitive {
				Primitive::Char => todo!(),
				Primitive::Number => Ok(self
					.builder
					.build_float_neg(
						self.value_to_llvm_float_value(rvalue)?,
						&format!("neg_result{}_", result_value.id()),
					)?
					.into()),
				Primitive::String => todo!(),
				Primitive::Null => todo!(),
			},
		}
	}

	pub fn build_not(
		&mut self,
		result_value: &P<Value>,
		rvalue: &P<Value>,
	) -> Result<BasicValueEnum<'ctx>> {
		let type_store = self.common_context.type_store.lock().unwrap();
		let result_type = type_store.get_type(result_value.ty()).unwrap();
		let Some(info) = result_type.get_info() else {
			unreachable!();
		};

		let result = match info {
			KnownTypeInfo::Primitive(primitive) => match primitive {
				Primitive::Char => todo!(),
				Primitive::Number => self.builder.build_float_compare(
					FloatPredicate::OEQ,
					self.context.f64_type().const_zero(),
					self.value_to_llvm_float_value(rvalue)?,
					&format!("not_result{}_", result_value.id()),
				)?,
				Primitive::String => todo!(),
				Primitive::Null => todo!(),
			},
		};

		return Ok(self
			.builder
			.build_int_z_extend(result, self.context.i64_type(), "z_extend_")?
			.into());
	}
}
