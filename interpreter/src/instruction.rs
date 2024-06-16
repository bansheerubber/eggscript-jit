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
	Pop,
	Store(AbsoluteStackAddress, RelativeStackAddress),
	Reserve(usize),
	Jump(usize),
}
