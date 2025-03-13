use eggscript_interpreter::{NumberMathOperation, NumberUnaryOperation};

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

impl Into<NumberMathOperation> for &BinaryOperator {
	fn into(self) -> NumberMathOperation {
		match self {
			BinaryOperator::Plus => NumberMathOperation::Plus,
			BinaryOperator::Minus => NumberMathOperation::Minus,
			BinaryOperator::Multiply => NumberMathOperation::Multiply,
			BinaryOperator::Divide => NumberMathOperation::Divide,
			BinaryOperator::Modulus => NumberMathOperation::Minus,
			BinaryOperator::BitwiseAnd => NumberMathOperation::BitwiseAnd,
			BinaryOperator::BitwiseOr => NumberMathOperation::BitwiseOr,
			BinaryOperator::BitwiseXor => NumberMathOperation::BitwiseXor,
			BinaryOperator::ShiftLeft => NumberMathOperation::ShiftLeft,
			BinaryOperator::ShiftRight => NumberMathOperation::ShiftRight,
			BinaryOperator::LogicalAnd => todo!(),
			BinaryOperator::LogicalOr => todo!(),
			BinaryOperator::Equal => NumberMathOperation::Equal,
			BinaryOperator::NotEqual => NumberMathOperation::NotEqual,
			BinaryOperator::LessThan => NumberMathOperation::LessThan,
			BinaryOperator::GreaterThan => NumberMathOperation::GreaterThan,
			BinaryOperator::LessThanEqualTo => NumberMathOperation::LessThanEqualTo,
			BinaryOperator::GreaterThanEqualTo => NumberMathOperation::GreaterThanEqualTo,
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnaryOperator {
	BitwiseNot,
	Minus,
	Not,
}

impl std::fmt::Display for UnaryOperator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			UnaryOperator::BitwiseNot => f.write_str("~"),
			UnaryOperator::Minus => f.write_str("-"),
			UnaryOperator::Not => f.write_str("!"),
		}
	}
}

impl Into<NumberUnaryOperation> for &UnaryOperator {
	fn into(self) -> NumberUnaryOperation {
		match self {
			UnaryOperator::BitwiseNot => NumberUnaryOperation::BitwiseNot,
			UnaryOperator::Minus => NumberUnaryOperation::Minus,
			UnaryOperator::Not => NumberUnaryOperation::Not,
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LogicOperator {
	And,
	Or
}
