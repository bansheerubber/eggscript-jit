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
			"=" => BinaryOperator::Equal,
			_ => return None,
		})
	}

	pub(crate) fn parse_binary(operator: &str) -> Option<BinaryOperator> {
		Some(match operator {
			"+" => BinaryOperator::Plus,
			"-" => BinaryOperator::Minus,
			"*" => BinaryOperator::Multiply,
			"/" => BinaryOperator::Divide,
			"%" => BinaryOperator::Modulus,
			"&" => BinaryOperator::BitwiseAnd,
			"|" => BinaryOperator::BitwiseOr,
			"^" => BinaryOperator::BitwiseXor,
			"<<" => BinaryOperator::ShiftLeft,
			">>" => BinaryOperator::ShiftRight,
			"==" => BinaryOperator::Equal,
			"!=" => BinaryOperator::NotEqual,
			"<" => BinaryOperator::LessThan,
			">" => BinaryOperator::GreaterThan,
			"<=" => BinaryOperator::LessThanEqualTo,
			">=" => BinaryOperator::GreaterThanEqualTo,
			_ => return None,
		})
	}
}

impl Into<eggscript_mir::BinaryOperator> for &BinaryOperator {
	fn into(self) -> eggscript_mir::BinaryOperator {
		match self {
			BinaryOperator::Plus => eggscript_mir::BinaryOperator::Plus,
			BinaryOperator::Minus => eggscript_mir::BinaryOperator::Minus,
			BinaryOperator::Multiply => eggscript_mir::BinaryOperator::Multiply,
			BinaryOperator::Divide => eggscript_mir::BinaryOperator::Divide,
			BinaryOperator::Modulus => eggscript_mir::BinaryOperator::Modulus,
			BinaryOperator::BitwiseAnd => eggscript_mir::BinaryOperator::BitwiseAnd,
			BinaryOperator::BitwiseOr => eggscript_mir::BinaryOperator::BitwiseOr,
			BinaryOperator::BitwiseXor => eggscript_mir::BinaryOperator::BitwiseXor,
			BinaryOperator::ShiftLeft => eggscript_mir::BinaryOperator::ShiftLeft,
			BinaryOperator::ShiftRight => eggscript_mir::BinaryOperator::ShiftRight,
			BinaryOperator::Equal => eggscript_mir::BinaryOperator::Equal,
			BinaryOperator::NotEqual => eggscript_mir::BinaryOperator::NotEqual,
			BinaryOperator::LessThan => eggscript_mir::BinaryOperator::LessThan,
			BinaryOperator::GreaterThan => eggscript_mir::BinaryOperator::GreaterThan,
			BinaryOperator::LessThanEqualTo => eggscript_mir::BinaryOperator::LessThanEqualTo,
			BinaryOperator::GreaterThanEqualTo => eggscript_mir::BinaryOperator::GreaterThanEqualTo,
		}
	}
}

#[derive(Clone, Debug)]
pub enum UnaryOperator {
	BitwiseNot,
	Minus,
	Not,
}

impl UnaryOperator {
	pub(crate) fn parse_unary(operator: &str) -> Option<UnaryOperator> {
		Some(match operator {
			"-" => UnaryOperator::Minus,
			"~" => UnaryOperator::BitwiseNot,
			"!" => UnaryOperator::Not,
			_ => return None,
		})
	}
}

impl Into<eggscript_mir::UnaryOperator> for &UnaryOperator {
	fn into(self) -> eggscript_mir::UnaryOperator {
		match self {
			UnaryOperator::BitwiseNot => eggscript_mir::UnaryOperator::BitwiseNot,
			UnaryOperator::Minus => eggscript_mir::UnaryOperator::Minus,
			UnaryOperator::Not => eggscript_mir::UnaryOperator::Not,
		}
	}
}

#[derive(Clone, Copy, Eq, Debug, PartialEq)]
pub enum LogicOperator {
	And,
	Or,
}

impl LogicOperator {
	pub(crate) fn parse_logic(operator: &str) -> Option<LogicOperator> {
		Some(match operator {
			"&&" => LogicOperator::And,
			"||" => LogicOperator::Or,
			_ => return None,
		})
	}
}
