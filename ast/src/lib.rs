#![feature(let_chains)]

mod context;
mod expressions;
mod lower;
mod operators;
mod parser;
mod symbol;

pub(crate) use context::AstContext;
pub use expressions::Function;
pub use expressions::FunctionArgument;
pub use lower::compile_expression;
pub use lower::compile_function;
pub use operators::BinaryOperator;
pub use operators::UnaryOperator;
pub use parser::parse_file;
pub use parser::parse_string;
pub use parser::Program;
pub use symbol::Ident;
pub use symbol::Span;
