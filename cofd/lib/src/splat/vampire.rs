use serde::{Deserialize, Serialize};

use crate::character::{Attribute, Modifier, ModifierOp, ModifierTarget, ModifierValue, Trait};

use super::{ability::Ability, XSplat, YSplat};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Clan {
	Daeva,
	Gangrel,
	Mekhet,
	Nosferatu,
	Ventrue,
	_Custom(String, [Discipline; 3], [Attribute; 2]),
}

impl Clan {
	pub fn name(&self) -> &str {
		match self {
			Clan::Daeva => "daeva",
			Clan::Gangrel => "gangrel",
			Clan::Mekhet => "mekhet",
			Clan::Nosferatu => "nosferatu",
			Clan::Ventrue => "ventrue",
			Clan::_Custom(name, _, _) => name,
		}
	}

	pub fn all() -> [Clan; 5] {
		[
			Clan::Daeva,
			Clan::Gangrel,
			Clan::Mekhet,
			Clan::Nosferatu,
			Clan::Ventrue,
		]
	}

	pub fn get_disciplines(&self) -> &[Discipline; 3] {
		match self {
			Clan::Daeva => &[Discipline::Celerity, Discipline::Majesty, Discipline::Vigor],
			Clan::Gangrel => &[
				Discipline::Animalism,
				Discipline::Protean,
				Discipline::Resilience,
			],
			Clan::Mekhet => &[
				Discipline::Auspex,
				Discipline::Celerity,
				Discipline::Obfuscate,
			],
			Clan::Nosferatu => &[
				Discipline::Nightmare,
				Discipline::Obfuscate,
				Discipline::Vigor,
			],
			Clan::Ventrue => &[
				Discipline::Animalism,
				Discipline::Dominate,
				Discipline::Resilience,
			],
			Clan::_Custom(_, disciplines, _) => disciplines,
		}
	}
	pub fn get_favored_attributes(&self) -> &[Attribute; 2] {
		match self {
			Clan::Daeva => &[Attribute::Dexterity, Attribute::Manipulation],
			Clan::Gangrel => &[Attribute::Composure, Attribute::Stamina],
			Clan::Mekhet => &[Attribute::Intelligence, Attribute::Wits],
			Clan::Nosferatu => &[Attribute::Composure, Attribute::Strength],
			Clan::Ventrue => &[Attribute::Presence, Attribute::Resolve],
			Clan::_Custom(_, _, attributes) => attributes,
		}
	}
}

impl From<Clan> for XSplat {
	fn from(val: Clan) -> Self {
		XSplat::Vampire(val)
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Covenant {
	CarthianMovement,
	CircleOfTheCrone,
	Invictus,
	LanceaEtSanctum,
	OrdoDracul,
	_Custom(String),
}

impl Covenant {
	pub fn name(&self) -> &str {
		match self {
			Covenant::CarthianMovement => "carthian_movement",
			Covenant::CircleOfTheCrone => "circle_of_the_crone",
			Covenant::Invictus => "invictus",
			Covenant::LanceaEtSanctum => "lancea_et_sanctum",
			Covenant::OrdoDracul => "ordo_dracul",
			Covenant::_Custom(name) => name,
		}
	}

	pub fn all() -> [Covenant; 5] {
		[
			Covenant::CarthianMovement,
			Covenant::CircleOfTheCrone,
			Covenant::Invictus,
			Covenant::LanceaEtSanctum,
			Covenant::OrdoDracul,
		]
	}
}

impl From<Covenant> for YSplat {
	fn from(covenant: Covenant) -> Self {
		YSplat::Vampire(covenant)
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Bloodline {
	_Custom(String, [Discipline; 4]),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub enum Discipline {
	Animalism,
	Auspex,
	Celerity,
	Dominate,
	Majesty,
	Nightmare,
	Obfuscate,
	Protean,
	Resilience,
	Vigor,
	_Custom(String),
}

impl Discipline {
	pub fn all() -> [Discipline; 10] {
		[
			Discipline::Animalism,
			Discipline::Auspex,
			Discipline::Celerity,
			Discipline::Dominate,
			Discipline::Majesty,
			Discipline::Nightmare,
			Discipline::Obfuscate,
			Discipline::Protean,
			Discipline::Resilience,
			Discipline::Vigor,
		]
	}

	pub fn name(&self) -> &str {
		match self {
			Discipline::Animalism => "animalism",
			Discipline::Auspex => "auspex",
			Discipline::Celerity => "celerity",
			Discipline::Dominate => "dominate",
			Discipline::Majesty => "majesty",
			Discipline::Nightmare => "nightmare",
			Discipline::Obfuscate => "obfuscate",
			Discipline::Protean => "protean",
			Discipline::Resilience => "resilience",
			Discipline::Vigor => "vigor",
			Discipline::_Custom(name) => name,
		}
	}

	pub fn custom(str: String) -> Discipline {
		Discipline::_Custom(str)
	}

	#[warn(clippy::cast_possible_wrap)]
	pub fn get_modifiers(&self, value: u8) -> Vec<crate::character::Modifier> {
		match self {
			Discipline::Celerity => {
				vec![Modifier::new(
					ModifierTarget::Trait(Trait::Defense),
					ModifierValue::Num(value as i8),
					ModifierOp::Add,
				)]
			}
			Discipline::Resilience => vec![Modifier::new(
				ModifierTarget::Attribute(Attribute::Stamina),
				ModifierValue::Num(value as i8),
				ModifierOp::Add,
			)],
			Discipline::Vigor => vec![Modifier::new(
				ModifierTarget::Attribute(Attribute::Strength),
				ModifierValue::Num(value as i8),
				ModifierOp::Add,
			)],
			_ => vec![],
		}
	}
}

impl From<Discipline> for Ability {
	fn from(val: Discipline) -> Self {
		Ability::Discipline(val)
	}
}

pub enum MaskDirge {
	Authoritarian,
	Child,
	Competitor,
	Conformist,
	Conspirator,
	Courtesan,
	CultLeader,
	Deviant,
	Follower,
	Guru,
	Idealist,
	Jester,
	Junkie,
	Martyr,
	Masochist,
	Monster,
	Nomad,
	Nurturer,
	Perfectionist,
	Penitent,
	Questioner,
	Rebel,
	Scholar,
	SocialChameleon,
	Spy,
	Survivor,
	Visionary,
	_Custom(String),
}
