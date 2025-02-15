mod function;
mod instruction;
mod interpreter;
pub mod runtime;

pub use function::Function;
pub use instruction::AbsoluteStackAddress;
pub use instruction::DoubleMathOperation;
pub use instruction::Instruction;
pub use instruction::IntegerMathOperation;
pub use instruction::RelativeStackAddress;
pub use instruction::Value;
pub use interpreter::Interpreter;
