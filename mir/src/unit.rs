use anyhow::{Context, Result};
use std::collections::HashMap;

use crate::{Transition, MIR};

pub type UnitHandle = usize;

pub struct Unit {
	pub id: usize,
	pub mir: Vec<MIR>,
	pub transition: Transition,
}

impl Unit {
	pub fn new() -> Self {
		Unit {
			id: 0,
			mir: Vec::new(),
			transition: Transition::Invalid,
		}
	}

	pub fn add_mir(&mut self, mir: Vec<MIR>) {
		let mut mir = mir;
		self.mir.append(&mut mir);
	}

	pub fn combine(&mut self, unit: Unit) {
		self.add_mir(unit.mir);
		self.transition = unit.transition.clone();
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
	unit_id_to_unit: HashMap<usize, Unit>,
	units: Vec<usize>,

	/// Key is the target, value is the unit that jumps to that target
	jump_targets: HashMap<UnitHandle, UnitHandle>,
}

impl UnitStore {
	pub fn new() -> UnitStore {
		UnitStore {
			next_unit_id: 0,
			unit_id_to_unit: HashMap::new(),
			units: vec![],

			jump_targets: HashMap::new(),
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

		return id;
	}

	pub fn set_transition(&mut self, unit: UnitHandle, transition: Transition) -> Result<()> {
		self.unit_id_to_unit
			.get_mut(&unit)
			.context("Could not find unit")?
			.transition = transition;

		Ok(())
	}

	pub fn get_unit(&self, unit: &UnitHandle) -> Option<&Unit> {
		self.unit_id_to_unit.get(unit)
	}

	pub fn combine_units(&mut self, units: Vec<UnitHandle>) -> Vec<usize> {
		let mut units = units;

		let mut spans_to_combine = vec![];
		let mut span = vec![];
		for i in 0..units.len() {
			// do not combine jump targets
			if span.len() > 0 && self.jump_targets.contains_key(&units[i]) {
				spans_to_combine.push(span);
				span = vec![];
			}

			let unit = &self.unit_id_to_unit[&units[i]];
			match unit.transition {
				Transition::Next => {}
				_ => {
					if span.len() > 0 {
						span.push(units[i]);
						spans_to_combine.push(span);
						span = vec![];
					}

					continue;
				}
			}

			span.push(units[i]);
		}

		if span.len() != 0 {
			spans_to_combine.push(span);
		}

		for span in spans_to_combine.iter() {
			let parent_unit = span.iter().nth(0).unwrap();
			for unit_handle in span.iter().skip(1) {
				let other = self.unit_id_to_unit.remove(&unit_handle).unwrap();
				let main = self.unit_id_to_unit.get_mut(&parent_unit).unwrap();
				main.combine(other);
			}
		}

		for span in spans_to_combine.iter().rev() {
			for unit_handle in span.iter().skip(1) {
				units.remove(units.iter().position(|unit| unit == unit_handle).unwrap());
			}
		}

		return units;
	}

	pub fn take_units(&mut self, units: Vec<UnitHandle>) -> Vec<Unit> {
		for unit in self.unit_id_to_unit.values() {
			if let Some(target) = unit.transition.jump_target() {
				self.jump_targets.insert(target, unit.id);
			}
		}

		let units = self.combine_units(units);

		units
			.iter()
			.map(|unit_id| self.unit_id_to_unit.remove(unit_id).unwrap())
			.collect::<Vec<Unit>>()
	}
}
