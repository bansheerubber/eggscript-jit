use crate::function::FunctionHandle;

pub type AbsoluteStackAddress = usize;
pub type RelativeStackAddress = isize;

#[derive(Clone, Debug)]
pub enum Value {
	Boolean(bool),
	Number(f64),
	Null,
}

impl Value {
	pub fn as_boolean(&self) -> bool {
		if let Value::Boolean(value) = self {
			return *value;
		} else {
			unreachable!();
		}
	}

	pub fn as_number(&self) -> f64 {
		if let Value::Number(value) = self {
			return *value;
		} else {
			unreachable!();
		}
	}
}

#[derive(Clone, Debug)]
pub enum Instruction {
	Invalid,
	Noop,
	Push(Value),
	CopyPush(AbsoluteStackAddress),
	Pop,
	Store(AbsoluteStackAddress, RelativeStackAddress),
	Reserve(usize),
	Jump(isize),
	JumpIfFalse(isize, RelativeStackAddress),
	JumpIfTrue(isize, RelativeStackAddress),
	NumberMath(
		NumberMathOperation,
		RelativeStackAddress,
		RelativeStackAddress,
	),
	ImmediateNumberMath(
		NumberMathOperation,
		Value,
		RelativeStackAddress,
	),
	CallFunction(FunctionHandle),
	Return(bool),
	NumberUnary(NumberUnaryOperation, RelativeStackAddress),
}

#[derive(Clone, Copy, Debug)]
pub enum NumberMathOperation {
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

#[derive(Clone, Debug)]
pub enum NumberUnaryOperation {
	BitwiseNot,
	Minus,
	Not,
}
