use serde::{Deserialize, Serialize};

use crate::character::{
	ability::Ability, Attribute, Modifier, ModifierTarget, ModifierValue, Trait, ModifierOp,
};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Clan {
	Daeva,
	Gangrel,
	Mekhet,
	Nosferatu,
	Ventrue,
	_Custom(String, [Discipline; 3], [Attribute; 2]),
}

impl Clan {
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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Covenant {
	CarthianMovement,
	CircleOfTheCrone,
	Invictus,
	LanceaEtSanctum,
	OrdoDracul,
	_Custom(String),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Bloodline {
	_Custom(String, [Discipline; 4])
}

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone)]
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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct DisciplineAbility(pub u8, pub Discipline);

impl Ability for DisciplineAbility {
	fn value(&self) -> &u8 {
		&self.0
	}

	fn value_mut(&mut self) -> &mut u8 {
		&mut self.0
	}

	fn get_modifiers(&self) -> Vec<crate::character::Modifier> {
		match self.1 {
			Discipline::Celerity => {
				vec![Modifier::new(
					ModifierTarget::Trait(Trait::Defense),
					ModifierValue::Num(self.0 as i8),
					ModifierOp::Add,
				)]
			}
			Discipline::Resilience => vec![Modifier::new(
				ModifierTarget::Attribute(Attribute::Stamina),
				ModifierValue::Num(self.0 as i8),
				ModifierOp::Add,
			)],
			Discipline::Vigor => vec![Modifier::new(
				ModifierTarget::Attribute(Attribute::Strength),
				ModifierValue::Num(self.0 as i8),
				ModifierOp::Add
			)],
			_ => vec![],
		}
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
