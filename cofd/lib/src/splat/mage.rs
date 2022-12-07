use serde::{Deserialize, Serialize};

use cofd_traits::VariantName;

use crate::{
	character::Skill,
	prelude::{Attribute, Character},
};

use super::{ability::Ability, Merit, NameKey, Splat, XSplat, YSplat, ZSplat};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct MageData {
	pub attr_bonus: Option<Attribute>,
	pub obsessions: Vec<String>,
	pub rotes: Vec<Rote>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, VariantName, AllVariants)]
pub enum Path {
	Acanthus,
	Mastigos,
	Moros,
	Obrimos,
	Thyrsus,
	_Custom(String, [Arcanum; 2], Arcanum),
}

impl Path {
	fn get_ruling_arcana(&self) -> &[Arcanum; 2] {
		match self {
			Path::Acanthus => &[Arcanum::Time, Arcanum::Fate],
			Path::Mastigos => &[Arcanum::Space, Arcanum::Mind],
			Path::Moros => &[Arcanum::Matter, Arcanum::Death],
			Path::Obrimos => &[Arcanum::Forces, Arcanum::Prime],
			Path::Thyrsus => &[Arcanum::Life, Arcanum::Spirit],
			Path::_Custom(_, ruling, _) => ruling,
		}
	}
	fn get_inferior_arcanum(&self) -> &Arcanum {
		match self {
			Path::Acanthus => &Arcanum::Forces,
			Path::Mastigos => &Arcanum::Matter,
			Path::Moros => &Arcanum::Spirit,
			Path::Obrimos => &Arcanum::Death,
			Path::Thyrsus => &Arcanum::Mind,
			Path::_Custom(_, _, inferior) => inferior,
		}
	}
}

impl From<Path> for XSplat {
	fn from(val: Path) -> Self {
		XSplat::Mage(val)
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Order {
	AdamantineArrow,
	GuardiansOfTheVeil,
	Mysterium,
	SilverLadder,
	FreeCouncil,
	SeersOfTheThrone(Option<Ministry>),
	_Custom(String, [Skill; 3]),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, AllVariants, VariantName)]
pub enum Ministry {
	Hegemony,
	Panopticon,
	Paternoster,
	Praetorian,
	_Custom(String, [Skill; 3]),
}

impl Order {
	pub fn get_rote_skills(&self) -> &[Skill; 3] {
		match self {
			Order::AdamantineArrow => &[Skill::Athletics, Skill::Intimidation, Skill::Medicine],
			Order::GuardiansOfTheVeil => &[Skill::Investigation, Skill::Stealth, Skill::Subterfuge],
			Order::Mysterium => &[Skill::Investigation, Skill::Occult, Skill::Survival],
			Order::SilverLadder => &[Skill::Expression, Skill::Persuasion, Skill::Subterfuge],
			Order::FreeCouncil => &[Skill::Crafts, Skill::Persuasion, Skill::Science],
			Order::SeersOfTheThrone(ministry) => match ministry {
				Some(ministry) => match ministry {
					Ministry::Hegemony => &[Skill::Politics, Skill::Persuasion, Skill::Empathy],
					Ministry::Panopticon => {
						&[Skill::Investigation, Skill::Stealth, Skill::Subterfuge]
					}
					Ministry::Paternoster => &[Skill::Academics, Skill::Occult, Skill::Expression],
					Ministry::Praetorian => {
						&[Skill::Athletics, Skill::Larceny, Skill::Intimidation]
					}
					Ministry::_Custom(_, skills) => skills,
				},
				None => &[Skill::Investigation, Skill::Occult, Skill::Persuasion],
			},
			Order::_Custom(_, skills) => skills,
		}
	}

	pub fn name(&self) -> &str {
		match self {
			Order::AdamantineArrow => "adamantine_arrow",
			Order::GuardiansOfTheVeil => "guardians_of_the_veil",
			Order::Mysterium => "mysterium",
			Order::SilverLadder => "silver_ladder",
			Order::FreeCouncil => "free_council",
			Order::SeersOfTheThrone(ministry) => match ministry {
				Some(ministry) => ministry.name(),
				None => "seers_of_the_throne",
			},
			Order::_Custom(name, _) => name,
		}
	}

	pub fn all() -> [Order; 10] {
		[
			Order::AdamantineArrow,
			Order::GuardiansOfTheVeil,
			Order::Mysterium,
			Order::SilverLadder,
			Order::FreeCouncil,
			Order::SeersOfTheThrone(None),
			Order::SeersOfTheThrone(Some(Ministry::Hegemony)),
			Order::SeersOfTheThrone(Some(Ministry::Panopticon)),
			Order::SeersOfTheThrone(Some(Ministry::Paternoster)),
			Order::SeersOfTheThrone(Some(Ministry::Praetorian)),
		]
	}
}

impl From<Order> for YSplat {
	fn from(order: Order) -> Self {
		YSplat::Mage(order)
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, VariantName)]
pub enum Legacy {
	_Custom(String, Option<Arcanum>),
}

impl From<Legacy> for ZSplat {
	fn from(legacy: Legacy) -> Self {
		ZSplat::Mage(legacy)
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, VariantName, AllVariants)]
pub enum Arcanum {
	Death,
	Fate,
	Forces,
	Life,
	Matter,
	Mind,
	Prime,
	Space,
	Spirit,
	Time,
}

impl NameKey for Arcanum {
	fn name_key(&self) -> String {
		format!("mage.{}", self.name())
	}
}

impl From<Arcanum> for Ability {
	fn from(val: Arcanum) -> Self {
		Ability::Arcanum(val)
	}
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Hash, AllVariants, VariantName)]
pub enum MageMerit {
	HighSpeech,
}

impl MageMerit {
	pub fn all() -> Vec<MageMerit> {
		vec![Self::HighSpeech]
	}

	pub fn is_available(&self, character: &Character) -> bool {
		matches!(character.splat, Splat::Mage(..))
	}
}

impl From<MageMerit> for Merit {
	fn from(merit: MageMerit) -> Self {
		Merit::Mage(merit)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Rote {
	pub arcanum: Arcanum,
	pub level: u16,
	pub spell: String,
	pub creator: String,
	pub skill: Skill,
}
