use serde::{Deserialize, Serialize};

use cofd_util::VariantName;

use super::{
	geist::Haunt,
	mage::Arcanum,
	vampire::Discipline,
	werewolf::{MoonGift, Renown},
	Merit, NameKey,
};
use crate::character::modifier::Modifier;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, VariantName)]
pub enum Ability {
	#[expand]
	Merit(Merit),
	#[expand]
	Discipline(Discipline),
	#[expand]
	Renown(Renown),
	#[expand]
	MoonGift(MoonGift),
	#[expand]
	Arcanum(Arcanum),
	#[expand]
	Haunt(Haunt),
}

impl Ability {
	pub fn name_mut(&mut self) -> Option<&mut String> {
		match self {
			Ability::Merit(Merit::_Custom(name))
			| Ability::Discipline(Discipline::_Custom(name))
			| Ability::MoonGift(MoonGift::_Custom(name)) => Some(name),
			_ => None,
		}
	}

	pub fn get_modifiers(&self, value: u16) -> Vec<Modifier> {
		match self {
			Ability::Merit(merit) => merit.get_modifiers(value),
			Ability::Discipline(discipline) => discipline.get_modifiers(value),
			Ability::MoonGift(moon_gift) => moon_gift.get_modifiers(value),
			_ => vec![],
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

impl NameKey for Ability {
	fn name_key(&self) -> String {
		match self {
			Ability::Merit(merit) => format!("merit.{}", merit.name()),
			Ability::Discipline(discipline) => format!("vampire.{}", discipline.name()),
			Ability::Renown(renown) => format!("werewolf.{}", renown.name()),
			Ability::MoonGift(moon_gift) => moon_gift.name_key(),
			Ability::Arcanum(arcanum) => arcanum.name_key(),
			Ability::Haunt(haunt) => format!("geist.{}", haunt.name()),
		}
	}
}
// #[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
// pub struct AbilityVal(pub Ability, pub u16);

// impl AbilityVal {
// 	pub fn get_modifiers(&self) -> Vec<Modifier> {
// 		self.0.get_modifiers(self.1)
// 	}
// }
