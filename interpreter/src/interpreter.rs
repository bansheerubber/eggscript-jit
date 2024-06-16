use crate::instruction::{Instruction, Value};

// extract values off of the stack based on isize stack index (negative means pop, positive means index into stack)
macro_rules! stack_extract {
	($self:ident, $i:expr) => {
		if ($i) < 0 {
			pop_stack(&$self.stack, &mut $self.stack_pointer)
		} else {
			&$self.stack[($i) as usize]
		}
	};
}

// make it so we always inline stack resizes, otherwise things can get slow
macro_rules! stack_resize {
	($self:ident) => {
		if $self.stack_pointer >= $self.stack.len() {
			let mut new_size = $self.stack_pointer * 2;
			while $self.stack_pointer >= new_size {
				new_size *= 2;
			}

			$self.stack.resize(new_size, Value::Null);
		}
	};
}

pub struct Interpreter {
	instructions: Vec<Instruction>,
	instruction_index: usize,
	stack: Vec<Value>,
	stack_pointer: usize,
}

impl Interpreter {
	pub fn new(instructions: Vec<Instruction>) -> Interpreter {
		Interpreter {
			instructions,
			instruction_index: 0,
			stack: vec![],
			stack_pointer: 0,
		}
	}

	pub fn run(&mut self) {
		while self.instruction_index < self.instructions.len() {
			self.interpret();
		}
	}

	pub fn print_stack(&mut self) {
		self.print_stack_region(0, self.stack_pointer);
	}

	pub fn print_stack_region(&mut self, start: usize, end: usize) {
		for i in start..(end.min(self.stack.len())) {
			println!("{}: {:?}", i, self.stack[i]);
		}
	}

	fn interpret(&mut self) {
		let instruction = &self.instructions[self.instruction_index];

		match instruction {
			Instruction::Invalid => panic!("Invalid instruction"),
			Instruction::Noop => {}
			Instruction::Push(value) => {
				self.push_stack(value.clone());
			}
			Instruction::Pop => {
				pop_stack(&self.stack, &mut self.stack_pointer);
			}
			Instruction::Store(index, position) => {
				let value = stack_extract!(self, *position);
				self.stack[*index] = value.clone();
			}
			Instruction::Reserve(amount) => {
				self.stack_pointer += amount;
				stack_resize!(self);
			}
			Instruction::Jump(position) => {
				self.instruction_index = *position;
				return;
			}
		}

		self.instruction_index += 1;
	}

	fn push_stack(&mut self, value: Value) {
		stack_resize!(self);
		self.stack[self.stack_pointer] = value;
		self.stack_pointer += 1;
	}
}

fn pop_stack<'a, 'b>(stack: &'a Vec<Value>, stack_pointer: &'b mut usize) -> &'a Value {
	let value = &stack[*stack_pointer - 1];
	*stack_pointer -= 1;
	return value;
}
