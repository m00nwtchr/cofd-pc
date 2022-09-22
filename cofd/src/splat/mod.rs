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

impl Splat {}

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
