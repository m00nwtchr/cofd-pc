use serde::{Deserialize, Serialize};

pub mod mage;
pub mod vampire;
pub mod werewolf;
// pub mod promethean;
pub mod changeling;
// pub mod hunter;
// pub mod geist;
// pub mod mummy;
// pub mod demon;
// pub mod beast;
// pub mod deviant;

use mage::*;
use vampire::*;
use werewolf::*;
// use promethean::*;
use changeling::*;

use crate::character::{
	ability::Ability, Modifier, ModifierOp, ModifierTarget, ModifierValue, Skill, Trait,
};
// use hunter::*;
// use geist::*;
// use mummy::*;
// use demon::*;
// use beast::*;
// use deviant:*;

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
pub enum Splat {
	#[default]
	Mortal,
	Vampire(Clan, Option<Covenant>, Option<Bloodline>),
	Werewolf(Option<Auspice>, Option<Tribe>, Option<String>, WerewolfData),
	Mage(Path, Option<Order>, Option<Legacy>),
	// Promethean,
	Changeling(Seeming, Option<Court>, Option<Kith>),
	// Hunter,
	// Geist,
	// Mummy,
	// Demon,
	// Beast,
	// Deviant,
}

impl Splat {
	pub fn name(&self) -> &str {
		match self {
			Splat::Mortal => "mortal",
			Splat::Vampire(_, _, _) => "vampire",
			Splat::Werewolf(_, _, _, _) => "werewolf",
			Splat::Mage(_, _, _) => "mage",
			Splat::Changeling(_, _, _) => "changeling",
		}
	}

	pub fn virtue_anchor(&self) -> &str {
		match self {
			Splat::Vampire(_, _, _) => "mask",
			Splat::Werewolf(_, _, _, _) => "blood",
			Splat::Changeling(_, _, _) => "thread",
			_ => "virtue",
		}
	}

	pub fn vice_anchor(&self) -> &str {
		match self {
			Splat::Vampire(_, _, _) => "dirge",
			Splat::Werewolf(_, _, _, _) => "bone",
			Splat::Changeling(_, _, _) => "needle",
			_ => "vice",
		}
	}

	pub fn xsplat_name(&self) -> &str {
		match self {
			Splat::Mortal => "",
			Splat::Vampire(_, _, _) => "clan",
			Splat::Werewolf(_, _, _, _) => "auspice",
			Splat::Mage(_, _, _) => "path",
			Splat::Changeling(_, _, _) => "seeming",
		}
	}

	pub fn ysplat_name(&self) -> &str {
		match self {
			Splat::Mortal => "faction",
			Splat::Vampire(_, _, _) => "covenant",
			Splat::Werewolf(_, _, _, _) => "tribe",
			Splat::Mage(_, _, _) => "order",
			Splat::Changeling(_, _, _) => "court",
		}
	}

	pub fn zsplat_name(&self) -> &str {
		match self {
			Splat::Mortal => "",
			Splat::Vampire(_, _, _) => "bloodline",
			Splat::Werewolf(_, _, _, _) => "lodge",
			Splat::Mage(_, _, _) => "legacy",
			Splat::Changeling(_, _, _) => "kith",
		}
	}
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum AbilityKey {
	Merit(Merit),
	Discipline(Discipline),
	Renown(Renown),
	Arcanum(Arcanum),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]

pub enum Merit {
	Status(String),
	FastTalking,
	ProfessionalTraining(String, [Skill; 2], Option<Skill>),
	Contacts(String),
	SafePlace(String),
	Resources,
	SleightOfHand,
	StrikingLooks(String),

	// Fighting Merits
	DefensiveCombat(Skill),

	//
	FeedingGrounds,
	CacophonySavvy,
	HoneyTrap,
	//
	NestGuardian,
	_Custom(String),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MeritAbility(pub u8, pub Merit);

impl Ability for MeritAbility {
	fn value(&self) -> &u8 {
		&self.0
	}

	fn value_mut(&mut self) -> &mut u8 {
		&mut self.0
	}

	fn get_modifiers(&self) -> Vec<Modifier> {
		match &self.1 {
			Merit::DefensiveCombat(skill) => {
				vec![Modifier::new(
					ModifierTarget::Trait(Trait::DefenseSkill),
					ModifierValue::Skill(skill.clone()),
					ModifierOp::Set,
				)]
			}
			_ => vec![],
		}
	}
}
