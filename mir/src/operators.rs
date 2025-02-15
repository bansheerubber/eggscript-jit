use eggscript_interpreter::{DoubleMathOperation, IntegerMathOperation};

#[derive(Clone, Eq, Debug, PartialEq)]
pub enum BinaryOperator {
	Plus,
	Minus,
	Multiply,
	Divide,
	Modulus,
	BitwiseAnd,
	BitwiseOr,
	BitwiseXor,
	ShiftLeft,
	ShiftRight,
	LogicalAnd,
	LogicalOr,
	Equal,
	NotEqual,
	LessThan,
	GreaterThan,
	LessThanEqualTo,
	GreaterThanEqualTo,
}

impl std::fmt::Display for BinaryOperator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			BinaryOperator::Plus => f.write_str("+"),
			BinaryOperator::Minus => f.write_str("-"),
			BinaryOperator::Multiply => f.write_str("*"),
			BinaryOperator::Divide => f.write_str("/"),
			BinaryOperator::Modulus => f.write_str("%"),
			BinaryOperator::BitwiseAnd => f.write_str("&"),
			BinaryOperator::BitwiseOr => f.write_str("|"),
			BinaryOperator::BitwiseXor => f.write_str("^"),
			BinaryOperator::ShiftLeft => f.write_str("<<"),
			BinaryOperator::ShiftRight => f.write_str(">>"),
			BinaryOperator::LogicalAnd => f.write_str("&&"),
			BinaryOperator::LogicalOr => f.write_str("||"),
			BinaryOperator::Equal => f.write_str("=="),
			BinaryOperator::NotEqual => f.write_str("!="),
			BinaryOperator::LessThan => f.write_str("<"),
			BinaryOperator::GreaterThan => f.write_str(">"),
			BinaryOperator::LessThanEqualTo => f.write_str("<="),
			BinaryOperator::GreaterThanEqualTo => f.write_str(">="),
		}
	}
}

impl Into<IntegerMathOperation> for &BinaryOperator {
	fn into(self) -> IntegerMathOperation {
		match self {
			BinaryOperator::Plus => IntegerMathOperation::Plus,
			BinaryOperator::Minus => IntegerMathOperation::Minus,
			BinaryOperator::Multiply => IntegerMathOperation::Multiply,
			BinaryOperator::Divide => IntegerMathOperation::Divide,
			BinaryOperator::Modulus => IntegerMathOperation::Minus,
			BinaryOperator::BitwiseAnd => IntegerMathOperation::BitwiseAnd,
			BinaryOperator::BitwiseOr => IntegerMathOperation::BitwiseOr,
			BinaryOperator::BitwiseXor => IntegerMathOperation::BitwiseXor,
			BinaryOperator::ShiftLeft => IntegerMathOperation::ShiftLeft,
			BinaryOperator::ShiftRight => IntegerMathOperation::ShiftRight,
			BinaryOperator::LogicalAnd => todo!(),
			BinaryOperator::LogicalOr => todo!(),
			BinaryOperator::Equal => IntegerMathOperation::Equal,
			BinaryOperator::NotEqual => IntegerMathOperation::NotEqual,
			BinaryOperator::LessThan => IntegerMathOperation::LessThan,
			BinaryOperator::GreaterThan => IntegerMathOperation::GreaterThan,
			BinaryOperator::LessThanEqualTo => IntegerMathOperation::LessThanEqualTo,
			BinaryOperator::GreaterThanEqualTo => IntegerMathOperation::GreaterThanEqualTo,
		}
	}
}

impl Into<DoubleMathOperation> for &BinaryOperator {
	fn into(self) -> DoubleMathOperation {
		match self {
			BinaryOperator::Plus => DoubleMathOperation::Plus,
			BinaryOperator::Minus => DoubleMathOperation::Minus,
			BinaryOperator::Multiply => DoubleMathOperation::Multiply,
			BinaryOperator::Divide => DoubleMathOperation::Divide,
			BinaryOperator::Modulus => DoubleMathOperation::Minus,
			BinaryOperator::BitwiseAnd => DoubleMathOperation::BitwiseAnd,
			BinaryOperator::BitwiseOr => DoubleMathOperation::BitwiseOr,
			BinaryOperator::BitwiseXor => DoubleMathOperation::BitwiseXor,
			BinaryOperator::ShiftLeft => DoubleMathOperation::ShiftLeft,
			BinaryOperator::ShiftRight => DoubleMathOperation::ShiftRight,
			BinaryOperator::LogicalAnd => todo!(),
			BinaryOperator::LogicalOr => todo!(),
			BinaryOperator::Equal => DoubleMathOperation::Equal,
			BinaryOperator::NotEqual => DoubleMathOperation::NotEqual,
			BinaryOperator::LessThan => DoubleMathOperation::LessThan,
			BinaryOperator::GreaterThan => DoubleMathOperation::GreaterThan,
			BinaryOperator::LessThanEqualTo => DoubleMathOperation::LessThanEqualTo,
			BinaryOperator::GreaterThanEqualTo => DoubleMathOperation::GreaterThanEqualTo,
		}
	}
}
