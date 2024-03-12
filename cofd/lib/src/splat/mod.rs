use serde::{Deserialize, Serialize};

use cofd_util::{AllVariants, NameKey, SplatEnum, VariantName};

use self::ability::Ability;
use crate::{
	character::modifier::{Modifier, ModifierOp, ModifierTarget, ModifierValue},
	character::traits::AttributeType,
	prelude::{Attribute, Attributes, Character, Skill, Skills, Trait},
};

pub mod ability;
pub mod merits;

pub use merits::*;

pub mod mage;
pub mod vampire;
pub mod werewolf;
// pub mod promethean;
pub mod changeling;
// pub mod hunter;
pub mod geist;
// pub mod mummy;
// pub mod demon;
// pub mod beast;
// pub mod deviant;

use mage::*;
use vampire::*;
use werewolf::*;
// use promethean::*;
use changeling::*;
// use hunter::*;
use geist::*;
// use mummy::*;
// use demon::*;
// use beast::*;
// use deviant:*;

#[derive(
	Clone,
	Default,
	Serialize,
	Deserialize,
	Debug,
	VariantName,
	SplatEnum,
	AllVariants,
	PartialEq,
	Eq,
)]
pub enum Splat {
	#[default]
	Mortal,
	#[splat(
		virtue_anchor = "mask",
		vice_anchor = "dirge",
		ability = "disciplines",
		st = "blood_potency",
		alt_beats = "blood",
		fuel = "vitae",
		integrity = "humanity",
		abilities_finite = false
	)]
	Vampire(Clan, Option<Covenant>, Option<Bloodline>, Box<VampireData>),
	#[splat(
		virtue_anchor = "blood",
		vice_anchor = "bone",
		ability = "renown",
		st = "primal_urge",
		fuel = "essence",
		integrity = "harmony"
	)]
	Werewolf(
		Option<Auspice>,
		Option<Tribe>,
		Option<Lodge>,
		Box<WerewolfData>,
	),
	#[splat(
		ability = "arcana",
		st = "gnosis",
		alt_beats = "arcane",
		fuel = "mana",
		integrity = "wisdom"
	)]
	Mage(Path, Option<Order>, Option<Legacy>, Box<MageData>), // TODO: Order = free order status, high speech merit
	// Promethean(Lineage),
	#[splat(
		virtue_anchor = "thread",
		vice_anchor = "needle",
		ability = "disciplines",
		st = "wyrd",
		fuel = "glamour",
		integrity = "clarity",
		abilities_finite = false
	)]
	Changeling(Seeming, Option<Court>, Option<Kith>, Box<ChangelingData>),
	// Hunter(Tier),
	#[splat(
		virtue_anchor = "root",
		vice_anchor = "bloom",
		ability = "haunts",
		st = "synergy",
		fuel = "plasm",
		integrity = "synergy",
		abilities_finite = false
	)]
	Bound(Burden, Archetype, #[skip] BoundData),
	// Mummy(Decree, Guild),
	// Demon(Incarnation, Vec<Agenda>),
	// Beast(Hunger),
	// Deviant(Origin, Clade, Vec<Form>),
}

impl Splat {
	pub fn custom_xsplat(&self, name: String) -> Option<XSplat> {
		match self {
			Self::Mortal => None,
			Self::Vampire(..) => Some(
				Clan::_Custom(
					name,
					Box::new([
						Discipline::Animalism,
						Discipline::Auspex,
						Discipline::Celerity,
					]),
					[Attribute::Composure, Attribute::Dexterity],
				)
				.into(),
			),
			Self::Werewolf(..) => Some(
				Auspice::_Custom(
					name,
					[Skill::Academics, Skill::AnimalKen, Skill::Athletics],
					Renown::Cunning,
					MoonGift::_Custom(String::from("Custom")),
					Box::new([ShadowGift::Death, ShadowGift::Dominance]),
					HuntersAspect::Monstrous,
				)
				.into(),
			),
			Self::Mage(..) => {
				Some(Path::_Custom(name, [Arcanum::Death, Arcanum::Fate], Arcanum::Forces).into())
			}
			Self::Changeling(..) => {
				Some(Seeming::_Custom(name, Regalia::Crown, AttributeType::Power).into())
			}
			Self::Bound(..) => {
				Some(Burden::_Custom(name, [Haunt::Boneyard, Haunt::Caul, Haunt::Curse]).into())
			}
		}
	}

	pub fn custom_ysplat(&self, name: String) -> Option<YSplat> {
		match self {
			Self::Mortal => None,
			Self::Vampire(..) => Some(Covenant::_Custom(name).into()),
			Self::Werewolf(..) => Some(
				Tribe::_Custom(
					name,
					Renown::Cunning,
					Box::new([
						ShadowGift::Death,
						ShadowGift::Dominance,
						ShadowGift::Elementals,
					]),
				)
				.into(),
			),
			Self::Mage(..) => Some(
				Order::_Custom(name, [Skill::Academics, Skill::AnimalKen, Skill::Athletics]).into(),
			),
			Self::Changeling(..) => Some(Court::_Custom(name).into()),
			Self::Bound(..) => Some(Archetype::_Custom(name).into()),
		}
	}

	pub fn custom_zsplat(&self, name: String) -> Option<ZSplat> {
		match self {
			Splat::Vampire(..) => Some(Bloodline::_Custom(name, None).into()),
			Splat::Werewolf(..) => Some(Lodge::_Custom(name).into()),
			Splat::Mage(..) => Some(Legacy::_Custom(name, None).into()),
			Splat::Changeling(..) => Some(Kith::_Custom(name).into()),
			_ => None,
		}
	}

	pub fn all_abilities(&self) -> Option<Vec<Ability>> {
		match self {
			Splat::Vampire(..) => Some(Discipline::all().into_iter().map(Into::into).collect()),
			Splat::Werewolf(..) => Some(Renown::all().into_iter().map(Into::into).collect()),
			Splat::Mage(..) => Some(Arcanum::all().into_iter().map(Into::into).collect()),
			Splat::Bound(..) => Some(Haunt::all().into_iter().map(Into::into).collect()),
			_ => None,
		}
	}

	pub fn custom_ability(&self, name: String) -> Option<Ability> {
		match self {
			Splat::Vampire(..) => Some(Discipline::_Custom(name).into()),
			Splat::Werewolf(..) => Some(MoonGift::_Custom(name).into()),
			Splat::Bound(..) => Some(Haunt::_Custom(name).into()),
			_ => None,
		}
	}

	pub fn alternate_beats_optional(&self) -> bool {
		match self {
			Self::Mage(..) => false,
			// Promethean
			// Demon
			_ => true,
		}
	}

	pub fn merits(&self) -> Vec<Merit> {
		match self {
			Self::Mortal => Merit::all(),
			Self::Vampire(..) => VampireMerit::all().map(Into::into).to_vec(),
			Self::Werewolf(..) => WerewolfMerit::all().map(Into::into).to_vec(),
			Self::Mage(..) => MageMerit::all().map(Into::into).to_vec(),
			Self::Changeling(..) => ChangelingMerit::all().map(Into::into).to_vec(),
			Self::Bound(..) => vec![],
		}
	}
}

impl XSplat {
	pub fn name_mut(&mut self) -> Option<&mut String> {
		match self {
			Self::Vampire(Clan::_Custom(name, ..))
			| Self::Werewolf(Auspice::_Custom(name, ..))
			| Self::Mage(Path::_Custom(name, ..))
			| Self::Changeling(Seeming::_Custom(name, ..))
			| Self::Bound(Burden::_Custom(name, ..)) => Some(name),
			_ => None,
		}
	}

	pub fn is_custom(&self) -> bool {
		matches!(
			self,
			Self::Vampire(Clan::_Custom(..))
				| Self::Werewolf(Auspice::_Custom(..))
				| Self::Mage(Path::_Custom(..))
				| Self::Changeling(Seeming::_Custom(..))
				| Self::Bound(Burden::_Custom(..))
		)
	}
}

impl YSplat {
	pub fn name_mut(&mut self) -> Option<&mut String> {
		match self {
			Self::Vampire(Covenant::_Custom(name))
			| Self::Werewolf(Tribe::_Custom(name, ..))
			| Self::Mage(
				Order::_Custom(name, ..)
				| Order::SeersOfTheThrone(Some(Ministry::_Custom(name, ..))),
			)
			| Self::Changeling(Court::_Custom(name))
			| Self::Bound(Archetype::_Custom(name, ..)) => Some(name),
			_ => None,
		}
	}

	pub fn is_custom(&self) -> bool {
		matches!(
			self,
			YSplat::Vampire(Covenant::_Custom(..))
				| YSplat::Werewolf(Tribe::_Custom(..))
				| YSplat::Mage(
					Order::_Custom(..) | Order::SeersOfTheThrone(Some(Ministry::_Custom(..))),
				) | YSplat::Changeling(Court::_Custom(..))
				| Self::Bound(Archetype::_Custom(..))
		)
	}
}

impl ZSplat {
	pub fn name_mut(&mut self) -> Option<&mut String> {
		match self {
			ZSplat::Vampire(Bloodline::_Custom(name, ..))
			| ZSplat::Werewolf(Lodge::_Custom(name))
			| ZSplat::Mage(Legacy::_Custom(name, ..))
			| ZSplat::Changeling(Kith::_Custom(name)) => Some(name),
			_ => None,
		}
	}

	pub fn is_custom(&self) -> bool {
		matches!(
			self,
			ZSplat::Vampire(Bloodline::_Custom(..))
				| ZSplat::Werewolf(Lodge::_Custom(..))
				| ZSplat::Mage(Legacy::_Custom(..))
				| ZSplat::Changeling(Kith::_Custom(..))
		)
	}
}
