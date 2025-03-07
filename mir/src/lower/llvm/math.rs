use anyhow::Result;
use eggscript_types::{KnownTypeInfo, Primitive, P};
use inkwell::{values::BasicValueEnum, FloatPredicate, IntPredicate};

use crate::{BinaryOperator, Value};

use super::LlvmLowerContext;

impl<'a, 'ctx> LlvmLowerContext<'a, 'ctx> {
	fn binary_operator_to_int_cmp(&self, operator: &BinaryOperator) -> IntPredicate {
		match operator {
			BinaryOperator::Equal => IntPredicate::EQ,
			BinaryOperator::NotEqual => IntPredicate::NE,
			BinaryOperator::LessThan => IntPredicate::SLT,
			BinaryOperator::GreaterThan => IntPredicate::SGT,
			BinaryOperator::LessThanEqualTo => IntPredicate::SLE,
			BinaryOperator::GreaterThanEqualTo => IntPredicate::SGE,
			_ => unreachable!(),
		}
	}

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
				Primitive::Double => Ok(self
					.builder
					.build_float_add(
						self.value_to_llvm_float_value(left_operand)?,
						self.value_to_llvm_float_value(right_operand)?,
						&format!("add_result{}_", result_value.id()),
					)?
					.into()),
				Primitive::I64 => Ok(self
					.builder
					.build_int_add(
						self.value_to_llvm_int_value(left_operand)?,
						self.value_to_llvm_int_value(right_operand)?,
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
				Primitive::Double => Ok(self
					.builder
					.build_float_sub(
						self.value_to_llvm_float_value(left_operand)?,
						self.value_to_llvm_float_value(right_operand)?,
						&format!("sub_result{}_", result_value.id()),
					)?
					.into()),
				Primitive::I64 => Ok(self
					.builder
					.build_int_sub(
						self.value_to_llvm_int_value(left_operand)?,
						self.value_to_llvm_int_value(right_operand)?,
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
				Primitive::Double => Ok(self
					.builder
					.build_float_mul(
						self.value_to_llvm_float_value(left_operand)?,
						self.value_to_llvm_float_value(right_operand)?,
						&format!("mul_result{}_", result_value.id()),
					)?
					.into()),
				Primitive::I64 => Ok(self
					.builder
					.build_int_mul(
						self.value_to_llvm_int_value(left_operand)?,
						self.value_to_llvm_int_value(right_operand)?,
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
				Primitive::Double => Ok(self
					.builder
					.build_float_div(
						self.value_to_llvm_float_value(left_operand)?,
						self.value_to_llvm_float_value(right_operand)?,
						&format!("div_result{}_", result_value.id()),
					)?
					.into()),
				Primitive::I64 => Ok(self
					.builder
					.build_int_signed_div(
						self.value_to_llvm_int_value(left_operand)?,
						self.value_to_llvm_int_value(right_operand)?,
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
				Primitive::Double => Ok(self
					.builder
					.build_float_rem(
						self.value_to_llvm_float_value(left_operand)?,
						self.value_to_llvm_float_value(right_operand)?,
						&format!("mod_result{}_", result_value.id()),
					)?
					.into()),
				Primitive::I64 => Ok(self
					.builder
					.build_int_signed_rem(
						self.value_to_llvm_int_value(left_operand)?,
						self.value_to_llvm_int_value(right_operand)?,
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
				Primitive::Double => self.builder.build_float_compare(
					self.binary_operator_to_float_cmp(op),
					self.value_to_llvm_float_value(left_operand)?,
					self.value_to_llvm_float_value(right_operand)?,
					&format!("cmp_result{}_", result_value.id()),
				)?,
				Primitive::I64 => self.builder.build_int_compare(
					self.binary_operator_to_int_cmp(op),
					self.value_to_llvm_int_value(left_operand)?,
					self.value_to_llvm_int_value(right_operand)?,
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
				Primitive::Double => todo!(),
				Primitive::I64 => Ok(self
					.builder
					.build_not(
						self.value_to_llvm_int_value(rvalue)?,
						&format!("not_result{}_", result_value.id()),
					)?
					.into()),
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
				Primitive::Double => Ok(self
					.builder
					.build_float_neg(
						self.value_to_llvm_float_value(rvalue)?,
						&format!("neg_result{}_", result_value.id()),
					)?
					.into()),
				Primitive::I64 => Ok(self
					.builder
					.build_int_neg(
						self.value_to_llvm_int_value(rvalue)?,
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
				Primitive::Double => self.builder.build_float_compare(
					FloatPredicate::OEQ,
					self.context.f64_type().const_zero(),
					self.value_to_llvm_float_value(rvalue)?,
					&format!("not_result{}_", result_value.id()),
				)?,
				Primitive::I64 => self.builder.build_int_compare(
					IntPredicate::EQ,
					self.context.i64_type().const_zero(),
					self.value_to_llvm_int_value(rvalue)?,
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
