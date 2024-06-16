#[derive(Debug)]
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

impl BinaryOperator {
	pub(crate) fn parse_assignment(operator: &str) -> Option<BinaryOperator> {
		Some(match operator {
			"+=" => BinaryOperator::Plus,
			"-=" => BinaryOperator::Minus,
			"*=" => BinaryOperator::Multiply,
			"/=" => BinaryOperator::Divide,
			"%=" => BinaryOperator::Modulus,
			"&=" => BinaryOperator::BitwiseAnd,
			"|=" => BinaryOperator::BitwiseOr,
			"^=" => BinaryOperator::BitwiseXor,
			"<<=" => BinaryOperator::ShiftLeft,
			">>=" => BinaryOperator::ShiftRight,
			"&&=" => BinaryOperator::LogicalAnd,
			"||=" => BinaryOperator::LogicalOr,
			"=" => BinaryOperator::Equal,
			_ => return None,
		})
	}
}

#[derive(Debug)]
pub enum UnaryOperator {
	BitwiseNot,
	Minus,
	Not,
}
