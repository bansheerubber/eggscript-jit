use anyhow::Result;
use eggscript_mir::{
	EggscriptLowerContext, LlvmLowerContext, MIRInfo, Transition, Unit, UnitHandle, UnitStore,
	Value, ValueStore, MIR,
};
use eggscript_types::P;
use inkwell::{builder::Builder, context, module::Module};

use crate::{
	expressions::{Block, Expression, ExpressionInfo},
	Function, Program,
};

pub struct AstLowerContext {
	pub program: P<Program>,
	pub unit_store: UnitStore,
	pub value_store: ValueStore,
}

impl Into<EggscriptLowerContext> for AstLowerContext {
	fn into(self) -> EggscriptLowerContext {
		EggscriptLowerContext::new(
			self.program.type_store.clone(),
			self.value_store,
			&self.program.file_name,
		)
	}
}

impl AstLowerContext {
	pub fn new(program: P<Program>) -> AstLowerContext {
		AstLowerContext {
			program,
			unit_store: UnitStore::new(),
			value_store: ValueStore::new(),
		}
	}

	pub fn into_llvm_lower_context<'a, 'ctx>(
		self,
		context: &'ctx context::Context,
		builder: &'a Builder<'ctx>,
		module: &'a Module<'ctx>,
	) -> LlvmLowerContext<'a, 'ctx> {
		LlvmLowerContext::new(
			context,
			builder,
			module,
			self.program.type_store.clone(),
			self.value_store,
			&self.program.file_name,
		)
	}

	pub fn lower_expression(
		&mut self,
		expression: &P<Expression>,
	) -> Result<(Vec<UnitHandle>, Option<P<Value>>)> {
		match expression.info {
			ExpressionInfo::Assign(_, _, _) => self.lower_variable_assignment(expression),
			ExpressionInfo::BinaryOperation(_, _, _) => self.lower_binary_operation(expression),
			ExpressionInfo::Else(_) => unreachable!(),
			ExpressionInfo::FieldAccess(_) => self.lower_field_access(expression),
			ExpressionInfo::For(_, _, _, _) => self.lower_for_block(expression),
			ExpressionInfo::FunctionCall(_, _) => self.lower_function_call(expression),
			ExpressionInfo::If(_, _, _) => self.lower_if_block(expression),
			ExpressionInfo::Primitive(_, _) => self.lower_primitive(expression),
			ExpressionInfo::Return(_) => self.lower_return_statement(expression),
			ExpressionInfo::Scope(_) => self.lower_scope(expression),
			ExpressionInfo::While(_, _) => self.lower_while_block(expression),
			_ => unimplemented!("{:?}", expression.info),
		}
	}

	pub fn lower_block(&mut self, block: &P<Block>) -> Result<(Vec<UnitHandle>, Option<P<Value>>)> {
		let mut units: Vec<UnitHandle> = vec![];
		for expression in block.expressions.iter() {
			let (mut more_units, _) = self.lower_expression(expression)?;
			units.append(&mut more_units);
		}

		Ok((units, None))
	}
}

pub fn compile_function(
	function: P<Function>,
	program: P<Program>,
	expression: P<Expression>,
) -> Result<(AstLowerContext, Vec<Unit>)> {
	let mut lower_context = AstLowerContext::new(program);

	let mut mir = vec![];
	let mut index = 0;
	for argument in function.arguments.iter() {
		let (value, _) = lower_context
			.value_store
			.new_location(&argument.name, argument.ty.unwrap());

		mir.push(MIR::new(
			MIRInfo::Allocate(value, Some(index)),
			argument.span,
		));
		index += 1;
	}

	let argument_unit = lower_context
		.unit_store
		.new_unit(mir, eggscript_mir::Transition::Next);

	let (mut units, _) = lower_context.lower_expression(&expression)?;
	units.insert(0, argument_unit);

	let mut return_count = 0;
	for unit in units.iter() {
		let Some(unit) = lower_context.unit_store.get_unit(unit) else {
			continue;
		};

		if let Transition::Return(_) = unit.transition {
			return_count += 1;
		}
	}

	if return_count == 0 {
		units.push(
			lower_context
				.unit_store
				.new_unit(vec![], eggscript_mir::Transition::Return(None)),
		);
	}

	let units = lower_context.unit_store.take_units(units);
	Ok((lower_context, units))
}

pub fn compile_expression(
	program: P<Program>,
	expression: P<Expression>,
) -> Result<(AstLowerContext, Vec<Unit>)> {
	let mut lower_context = AstLowerContext::new(program);
	let (units, _) = lower_context.lower_expression(&expression)?;
	let units = lower_context.unit_store.take_units(units);
	Ok((lower_context, units))
}
