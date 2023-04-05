use cofd_util::VariantName;
use serde::{Deserialize, Serialize};

use super::{ability::Ability, NameKey};

#[derive(
	Debug, Clone, PartialEq, Eq, Serialize, Deserialize, AllVariants, VariantName, Default,
)]
pub enum Burden {
	#[default]
	Abiding,
	Bereaved,
	Hungry,
	Kindly,
	Vengeful,
	_Custom(String, [Haunt; 3]),
}

impl Burden {
	pub fn get_favoured_haunts(&self) -> &[Haunt; 3] {
		match self {
			Self::Abiding => &[Haunt::Caul, Haunt::Memoria, Haunt::Tomb],
			Self::Bereaved => &[Haunt::Curse, Haunt::Oracle, Haunt::Shroud],
			Self::Hungry => &[Haunt::Boneyard, Haunt::Marionette, Haunt::Caul],
			Self::Kindly => &[Haunt::Dirge, Haunt::Marionette, Haunt::Shroud],
			Self::Vengeful => &[Haunt::Curse, Haunt::Memoria, Haunt::Rage],
			Self::_Custom(_, haunts) => haunts,
		}
	}
}

#[derive(
	Debug, Clone, PartialEq, Eq, Serialize, Deserialize, AllVariants, VariantName, Default,
)]
pub enum Archetype {
	#[default]
	Furies,
	Mourners,
	Necropolitans,
	Pilgrims,
	Undertakers,
	_Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, AllVariants, VariantName)]
pub enum Haunt {
	Boneyard,
	Caul,
	Curse,
	Dirge,
	Marionette,
	Memoria,
	Oracle,
	Rage,
	Shroud,
	Tomb,
	_Custom(String),
}

impl From<Haunt> for Ability {
	fn from(val: Haunt) -> Self {
		Ability::Haunt(val)
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, AllVariants, VariantName)]
pub enum Key {
	Beasts,
	Blood,
	Chance,
	ColdWind,
	Disease,
	GraveDirt,
	PyreFlame,
	Stillness,
}

impl NameKey for Key {
	fn name_key(&self) -> String {
		format!("geist.{}", self.name())
	}
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BoundData {
	pub keys: Vec<Key>,
}
