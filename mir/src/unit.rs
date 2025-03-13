use anyhow::{Context, Result};
use indexmap::IndexMap;
use std::collections::HashMap;

use crate::{MIRInfo, Transition, MIR};

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

	pub fn starts_with_phi(&self) -> bool {
		let Some(mir) = self.mir.first() else {
			return false;
		};

		if let MIRInfo::LogicPhi(_, _, _, _, _, _) = mir.info {
			return true;
		} else {
			return false;
		}
	}

	pub fn goto_target(&self) -> Option<UnitHandle> {
		match self.transition {
			Transition::Goto(target) => Some(target),
			Transition::GotoIfFalse(target, _) => Some(target),
			Transition::GotoIfTrue(target, _) => Some(target),
			Transition::Invalid => None,
			Transition::Next => None,
			Transition::Return(_) => None,
		}
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

#[derive(Default)]
pub struct UnitStore {
	next_unit_id: usize,
	unit_id_to_unit: HashMap<usize, Unit>,
	units: Vec<usize>,

	/// Key is the target, value is the unit that jumps to that target
	jump_targets: HashMap<UnitHandle, UnitHandle>,
	combined_units: HashMap<UnitHandle, UnitHandle>,
}

impl UnitStore {
	pub fn new() -> UnitStore {
		UnitStore {
			next_unit_id: 0,
			unit_id_to_unit: HashMap::new(),
			units: vec![],

			jump_targets: HashMap::new(),
			combined_units: HashMap::new(),
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

	pub fn get_unit_mut(&mut self, unit: &UnitHandle) -> Option<&mut Unit> {
		self.unit_id_to_unit.get_mut(unit)
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
			let parent_unit = span
				.iter()
				.nth(0)
				.expect("Could not get first unit in span");

			for unit_handle in span.iter().skip(1) {
				let other = self
					.unit_id_to_unit
					.remove(&unit_handle)
					.expect("Could not remove unit");

				let main = self
					.unit_id_to_unit
					.get_mut(&parent_unit)
					.expect("Could not mutate parent unit");

				self.combined_units.insert(other.id, main.id);

				main.combine(other);
			}
		}

		for span in spans_to_combine.iter().rev() {
			for unit_handle in span.iter().skip(1) {
				units.remove(
					units
						.iter()
						.position(|unit| unit == unit_handle)
						.expect("Could not find unit in list"),
				);
			}
		}

		// re-write PHI instructions
		for unit in units.iter() {
			let unit = self
				.unit_id_to_unit
				.get_mut(unit)
				.expect("Could not find unit");

			for mir in unit.mir.iter_mut() {
				match &mir.info {
					MIRInfo::LogicPhi(
						result,
						default,
						test_value,
						operator,
						use_default_units,
						use_value_unit,
					) => {
						let new_units = use_default_units
							.iter()
							.map(|unit| *self.combined_units.get(unit).unwrap_or(unit))
							.collect::<Vec<UnitHandle>>();

						mir.info = MIRInfo::LogicPhi(
							result.clone(),
							default.clone(),
							test_value.clone(),
							operator.clone(),
							new_units,
							*self
								.combined_units
								.get(use_value_unit)
								.unwrap_or(use_value_unit),
						);
					}
					_ => {}
				}
			}
		}

		return units;
	}

	pub fn take_units(&mut self, units: Vec<UnitHandle>) -> IndexMap<UnitHandle, Unit> {
		for unit in self.unit_id_to_unit.values() {
			if let Some(target) = unit.transition.jump_target() {
				self.jump_targets.insert(target, unit.id);
			}
		}

		let units = self.combine_units(units);

		let mut result = IndexMap::new();
		for unit in units {
			result.insert(
				unit,
				self.unit_id_to_unit
					.remove(&unit)
					.expect("Could not find unit"),
			);
		}

		return result;
	}
}
