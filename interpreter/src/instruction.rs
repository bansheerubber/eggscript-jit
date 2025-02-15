use crate::function::FunctionHandle;

pub type AbsoluteStackAddress = usize;
pub type RelativeStackAddress = isize;

#[derive(Clone, Debug)]
pub enum Value {
	Boolean(bool),
	Double(f64),
	Integer(i64),
	Null,
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
	IntegerMath(
		IntegerMathOperation,
		RelativeStackAddress,
		RelativeStackAddress,
	),
	DoubleMath(
		DoubleMathOperation,
		RelativeStackAddress,
		RelativeStackAddress,
	),
	CallFunction(FunctionHandle),
	Return(bool),
}

#[derive(Clone, Debug)]
pub enum IntegerMathOperation {
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
pub enum DoubleMathOperation {
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
