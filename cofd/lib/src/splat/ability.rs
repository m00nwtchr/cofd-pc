use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::character::Modifier;

use super::{
	mage::Arcanum,
	vampire::Discipline,
	werewolf::{MoonGift, Renown},
	Merit,
};

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub enum Ability {
	Merit(Merit),
	Discipline(Discipline),
	Renown(Renown),
	MoonGift(MoonGift),
	Arcanum(Arcanum),
}

impl Ability {
	pub fn name(&self) -> &str {
		match self {
			Ability::Merit(merit) => merit.name(),
			Ability::Discipline(discipline) => discipline.name(),
			Ability::Renown(renown) => renown.name(),
			Ability::MoonGift(moon_gift) => moon_gift.name(),
			Ability::Arcanum(arcanum) => arcanum.name(),
		}
	}

	pub fn name_mut(&mut self) -> Option<&mut String> {
		match self {
			Ability::Merit(Merit::_Custom(name))
			| Ability::Discipline(Discipline::_Custom(name))
			| Ability::MoonGift(MoonGift::_Custom(name)) => Some(name),
			_ => None,
		}
	}

	pub fn get_modifiers(&self, value: u8) -> Vec<Modifier> {
		match self {
			Ability::Merit(merit) => merit.get_modifiers(value),
			Ability::Discipline(discipline) => discipline.get_modifiers(value),
			Ability::Renown(_renown) => vec![],
			Ability::MoonGift(moon_gift) => moon_gift.get_modifiers(value),
			Ability::Arcanum(_arcanum) => vec![],
		}
	}

	pub fn is_custom(&self) -> bool {
		matches!(
			self,
			Ability::Merit(Merit::_Custom(_))
				| Ability::Discipline(Discipline::_Custom(_))
				| Ability::MoonGift(MoonGift::_Custom(_))
		)
	}
}

impl Display for Ability {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.name())
	}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct AbilityVal(pub Ability, pub u8);

impl AbilityVal {
	pub fn get_modifiers(&self) -> Vec<Modifier> {
		self.0.get_modifiers(self.1)
	}
}
