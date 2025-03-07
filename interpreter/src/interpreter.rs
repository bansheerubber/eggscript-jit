use std::rc::Rc;
use std::time::Instant;

use crate::instruction::{DoubleMathOperation, Instruction, IntegerUnaryOperation, Value};
use crate::{DoubleUnaryOperation, Function, IntegerMathOperation};

// extract values off of the stack based on isize stack index (negative means pop, positive means index into stack)
macro_rules! stack_extract {
	($self:ident, $i:expr) => {
		if ($i) < 0 {
			pop_stack(&$self.stack, &mut $self.stack_pointer)
		} else {
			&$self.stack[$self.stack_base + ($i) as usize]
		}
	};
}

// make it so we always inline stack resizes, otherwise things can get slow
macro_rules! stack_resize {
	($self:ident) => {
		if $self.stack_pointer >= $self.stack.len() && $self.stack.len() != 0 {
			let mut new_size = $self.stack_pointer * 2;
			while $self.stack_pointer >= new_size {
				new_size *= 2;
			}

			$self.stack.resize(new_size, Value::Null);
		}
	};
}

struct InterpreterFrame {
	argument_count: usize,
	instruction_index: usize,
	instructions: Rc<Vec<Instruction>>,
	stack_base: usize,
	stack_pointer: usize,
}

pub struct Interpreter {
	frames: Vec<InterpreterFrame>,
	functions: Vec<Function>,
	instruction_index: usize,
	instructions: Rc<Vec<Instruction>>,
	stack: Vec<Value>,
	stack_base: usize,
	stack_pointer: usize,
}

impl Interpreter {
	pub fn new(global_instructions: Vec<Instruction>) -> Interpreter {
		let global_instructions = Rc::new(global_instructions);

		Interpreter {
			frames: vec![],
			functions: vec![],
			instructions: global_instructions,
			instruction_index: 0,
			stack: vec![Value::Null],
			stack_base: 0,
			stack_pointer: 0,
		}
	}

	pub fn run(&mut self) {
		while self.instruction_index < self.instructions.len() {
			self.interpret();
		}
	}

	pub fn run_with_timeout(&mut self, ms: u128) {
		let start = Instant::now();
		while self.instruction_index < self.instructions.len() {
			self.interpret();

			if start.elapsed().as_millis() > ms {
				return;
			}
		}
	}

	pub fn print_stack(&self) {
		self.print_stack_region(0, self.stack_pointer);
	}

	pub fn print_stack_region(&self, start: usize, end: usize) {
		println!("stack {}..{}:", start, end);
		for i in start..(end.min(self.stack.len())) {
			println!("{}: {:?}", i, self.stack[i]);
		}
	}

	pub fn add_function(&mut self, function: Function) {
		self.functions.push(function);
	}

	fn interpret(&mut self) {
		let instruction = &self.instructions[self.instruction_index];

		self.print_stack();

		match instruction {
			Instruction::DoubleMath(operator, lvalue, rvalue) => {
				let rvalue = stack_extract!(self, *rvalue);
				let lvalue = stack_extract!(self, *lvalue);

				let Value::Double(lvalue) = lvalue else {
					unreachable!();
				};

				let Value::Double(rvalue) = rvalue else {
					unreachable!();
				};

				match operator {
					DoubleMathOperation::Plus => self.push_stack(Value::Double(lvalue + rvalue)),
					DoubleMathOperation::Minus => self.push_stack(Value::Double(lvalue - rvalue)),
					DoubleMathOperation::Multiply => {
						self.push_stack(Value::Double(lvalue * rvalue))
					}
					DoubleMathOperation::Divide => self.push_stack(Value::Double(lvalue / rvalue)),
					DoubleMathOperation::Modulus => self.push_stack(Value::Double(lvalue % rvalue)),
					DoubleMathOperation::BitwiseAnd => todo!(),
					DoubleMathOperation::BitwiseOr => todo!(),
					DoubleMathOperation::BitwiseXor => todo!(),
					DoubleMathOperation::ShiftLeft => todo!(),
					DoubleMathOperation::ShiftRight => todo!(),
					DoubleMathOperation::Equal => self.push_stack(Value::Boolean(lvalue == rvalue)),
					DoubleMathOperation::NotEqual => {
						self.push_stack(Value::Boolean(lvalue != rvalue))
					}
					DoubleMathOperation::LessThan => {
						self.push_stack(Value::Boolean(lvalue < rvalue))
					}
					DoubleMathOperation::GreaterThan => {
						self.push_stack(Value::Boolean(lvalue > rvalue))
					}
					DoubleMathOperation::LessThanEqualTo => {
						self.push_stack(Value::Boolean(lvalue <= rvalue))
					}
					DoubleMathOperation::GreaterThanEqualTo => {
						self.push_stack(Value::Boolean(lvalue >= rvalue))
					}
				}
			}
			Instruction::IntegerMath(operator, lvalue, rvalue) => {
				let rvalue = stack_extract!(self, *rvalue);
				let lvalue = stack_extract!(self, *lvalue);

				let Value::Integer(lvalue) = lvalue else {
					unreachable!();
				};

				let Value::Integer(rvalue) = rvalue else {
					unreachable!();
				};

				match operator {
					IntegerMathOperation::Plus => self.push_stack(Value::Integer(lvalue + rvalue)),
					IntegerMathOperation::Minus => self.push_stack(Value::Integer(lvalue - rvalue)),
					IntegerMathOperation::Multiply => {
						self.push_stack(Value::Integer(lvalue * rvalue))
					}
					IntegerMathOperation::Divide => {
						self.push_stack(Value::Integer(lvalue / rvalue))
					}
					IntegerMathOperation::Modulus => {
						self.push_stack(Value::Integer(lvalue % rvalue))
					}
					IntegerMathOperation::BitwiseAnd => {
						self.push_stack(Value::Integer(lvalue & rvalue))
					}
					IntegerMathOperation::BitwiseOr => {
						self.push_stack(Value::Integer(lvalue | rvalue))
					}
					IntegerMathOperation::BitwiseXor => {
						self.push_stack(Value::Integer(lvalue ^ rvalue))
					}
					IntegerMathOperation::ShiftLeft => {
						self.push_stack(Value::Integer(lvalue << rvalue))
					}
					IntegerMathOperation::ShiftRight => {
						self.push_stack(Value::Integer(lvalue >> rvalue))
					}
					IntegerMathOperation::Equal => {
						self.push_stack(Value::Boolean(lvalue == rvalue))
					}
					IntegerMathOperation::NotEqual => {
						self.push_stack(Value::Boolean(lvalue != rvalue))
					}
					IntegerMathOperation::LessThan => {
						self.push_stack(Value::Boolean(lvalue < rvalue))
					}
					IntegerMathOperation::GreaterThan => {
						self.push_stack(Value::Boolean(lvalue > rvalue))
					}
					IntegerMathOperation::LessThanEqualTo => {
						self.push_stack(Value::Boolean(lvalue <= rvalue))
					}
					IntegerMathOperation::GreaterThanEqualTo => {
						self.push_stack(Value::Boolean(lvalue >= rvalue))
					}
				}
			}
			Instruction::Invalid => panic!("Invalid instruction"),
			Instruction::Noop => {}
			Instruction::Push(value) => {
				self.push_stack(value.clone());
			}
			Instruction::CopyPush(position) => {
				self.push_stack(self.stack[self.stack_base + *position].clone());
			}
			Instruction::Pop => {
				pop_stack(&self.stack, &mut self.stack_pointer);
			}
			Instruction::Store(index, position) => {
				let value = stack_extract!(self, *position);
				self.stack[self.stack_base + *index] = value.clone();
			}
			Instruction::Reserve(amount) => {
				self.stack_pointer += amount;
				stack_resize!(self);
			}
			Instruction::Jump(position) => {
				self.instruction_index = self
					.instruction_index
					.checked_add_signed(*position)
					.expect("Failed relative jump");
				return;
			}
			Instruction::JumpIfFalse(position, value_position) => {
				let value = stack_extract!(self, *value_position);
				if let Value::Boolean(true) = value {
				} else {
					self.instruction_index = self
						.instruction_index
						.checked_add_signed(*position)
						.expect("Failed relative jump");

					return;
				}
			}
			Instruction::JumpIfTrue(position, value_position) => {
				let value = stack_extract!(self, *value_position);
				if let Value::Boolean(true) = value {
					self.instruction_index = self
						.instruction_index
						.checked_add_signed(*position)
						.expect("Failed relative jump");

					return;
				}
			}
			Instruction::CallFunction(function_handle) => {
				let function = &self.functions[*function_handle];
				match function {
					Function::Eggscript {
						argument_count,
						instructions,
						..
					} => {
						self.frames.push(InterpreterFrame {
							argument_count: *argument_count,
							instruction_index: self.instruction_index,
							instructions: self.instructions.clone(),
							stack_base: self.stack_base,
							stack_pointer: self.stack_pointer,
						});

						self.instruction_index = 0;
						self.instructions = instructions.clone();
						self.stack_base = self.stack_pointer - *argument_count;

						return;
					}
					Function::Native {
						argument_count,
						function,
						..
					} => {
						let mut arguments = vec![];
						for _ in 0..*argument_count {
							arguments.push(pop_stack(&self.stack, &mut self.stack_pointer).clone());
						}

						self.push_stack(function(arguments));
					}
				}
			}
			Instruction::Return(has_value) => {
				let value = if *has_value {
					Some(pop_stack(&self.stack, &mut self.stack_pointer))
				} else {
					None
				};

				let old_frame = self.frames.pop().unwrap();
				self.instruction_index = old_frame.instruction_index;
				self.instructions = old_frame.instructions;
				self.stack_pointer = old_frame.stack_pointer - old_frame.argument_count;
				self.stack_base = old_frame.stack_base;

				if let Some(value) = value {
					self.push_stack(value.clone());
				}
			}
			Instruction::IntegerUnary(operator, value_position) => {
				let value = stack_extract!(self, *value_position);

				let Value::Integer(value) = value else {
					unreachable!();
				};

				match operator {
					IntegerUnaryOperation::BitwiseNot => {
						self.push_stack(Value::Integer(!value));
					}
					IntegerUnaryOperation::Minus => {
						self.push_stack(Value::Integer(-value));
					}
					IntegerUnaryOperation::Not => {
						if value == &0 {
							self.push_stack(Value::Integer(1));
						} else {
							self.push_stack(Value::Integer(0));
						}
					}
				}
			}
			Instruction::DoubleUnary(operator, value_position) => {
				let value = stack_extract!(self, *value_position);

				let Value::Double(value) = value else {
					unreachable!();
				};

				match operator {
					DoubleUnaryOperation::BitwiseNot => {
						unreachable!()
					}
					DoubleUnaryOperation::Minus => {
						self.push_stack(Value::Double(-value));
					}
					DoubleUnaryOperation::Not => {
						if value == &0.0 {
							self.push_stack(Value::Integer(1));
						} else {
							self.push_stack(Value::Integer(0));
						}
					}
				}
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
