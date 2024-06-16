use std::{collections::HashMap, fmt::Write};

use eggscript_types::P;

use crate::mir::{Transition, MIR};

pub type UnitHandle = usize;

pub struct Unit {
	pub(crate) id: UnitHandle,
	pub(crate) mir: Vec<MIR>,
	pub(crate) transition: Transition,
}

impl Unit {
	pub fn append(&mut self, mir: Vec<MIR>) {
		let mut mir = mir;
		self.mir.append(&mut mir);
	}

	pub(crate) fn take(&mut self) -> Vec<MIR> {
		std::mem::replace(&mut self.mir, vec![])
	}

	pub fn combine(&mut self, unit: Unit) {
		self.append(unit.mir);
		self.transition = unit.transition;
	}
}

impl std::fmt::Display for Unit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("block {} {{\n", self.id))?;

		for mir in self.mir.iter() {
			f.write_fmt(format_args!("{}", mir))?;
		}

		f.write_fmt(format_args!("{}\n", self.transition))?;

		f.write_str("}\n")
	}
}

pub struct UnitStore {
	next_unit_id: usize,
	unit_id_to_unit: HashMap<UnitHandle, Unit>,
	units: Vec<UnitHandle>,
}

impl UnitStore {
	pub fn new() -> UnitStore {
		UnitStore {
			next_unit_id: 0,
			unit_id_to_unit: HashMap::new(),
			units: vec![],
		}
	}

	pub fn new_unit(&mut self, mir: Vec<MIR>, transition: Transition) -> UnitHandle {
		let id = self.next_unit_id;
		self.next_unit_id += 1;

		self.unit_id_to_unit.insert(
			id,
			Unit {
				id,
				mir,
				transition,
			},
		);

		self.units.push(id);

		id
	}

	pub fn combine_units(&mut self) {
		let mut spans_to_combine = vec![];
		let mut span = (0, 0);
		for i in 0..self.units.len() {
			let unit = &self.unit_id_to_unit[&self.units[i]];
			match unit.transition {
				Transition::Next => {}
				_ => {
					if span.0 != span.1 {
						spans_to_combine.push(span);
						span = (0, 0);
					}
					continue;
				}
			}

			if span.0 == span.1 {
				span.0 = i;
				span.1 = i + 1;
			} else {
				span.1 = i + 1;
			}
		}

		if span.0 != span.1 {
			span.1 -= 1;
			spans_to_combine.push(span);
		}

		for span in spans_to_combine.iter() {
			for i in span.0 + 1..=span.1 {	
				let other = self.unit_id_to_unit.remove(&self.units[i]).unwrap();
				let main = self.unit_id_to_unit.get_mut(&self.units[span.0]).unwrap();
				main.combine(other);
			}
		}

		for span in spans_to_combine.iter().rev() {
			for i in (span.0 + 1..=span.1).rev() {
				self.units.remove(i);
			}
		}
	}

	pub fn take_units(&mut self) -> Vec<Unit> {
		self.combine_units();

		self.units
			.iter()
			.map(|unit_id| self.unit_id_to_unit.remove(unit_id).unwrap())
			.collect::<Vec<Unit>>()
	}
}
