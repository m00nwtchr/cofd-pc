use serde::{Deserialize, Serialize};

use cofd_util::VariantName;

use super::Armor;
use crate::splat::NameKey;

#[derive(VariantName)]
pub enum TraitCategory {
	Mental,
	Physical,
	Social,
}

impl TraitCategory {
	pub fn unskilled(&self) -> u16 {
		match self {
			TraitCategory::Mental => 3,
			TraitCategory::Physical => 1,
			TraitCategory::Social => 1,
		}
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum AttributeType {
	Power,
	Finesse,
	Resistance,
}

pub enum AttributeCategory {
	Type(AttributeType),
	Trait(TraitCategory),
}

#[derive(
	PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize, AllVariants, VariantName,
)]
pub enum Attribute {
	Intelligence,
	Wits,
	Resolve,

	Strength,
	Dexterity,
	Stamina,

	Presence,
	Manipulation,
	Composure,
}

impl Attribute {
	pub fn mental() -> [Attribute; 3] {
		[Self::Intelligence, Self::Wits, Self::Resolve]
	}

	pub fn physical() -> [Attribute; 3] {
		[Self::Strength, Self::Dexterity, Self::Stamina]
	}

	pub fn social() -> [Attribute; 3] {
		[Self::Presence, Self::Manipulation, Self::Composure]
	}

	pub fn power() -> [Attribute; 3] {
		[Self::Intelligence, Self::Strength, Self::Presence]
	}

	pub fn finesse() -> [Attribute; 3] {
		[Self::Wits, Self::Dexterity, Self::Manipulation]
	}

	pub fn resistance() -> [Attribute; 3] {
		[Self::Resolve, Self::Stamina, Self::Composure]
	}

	pub fn get(cat: AttributeCategory) -> [Attribute; 3] {
		match cat {
			AttributeCategory::Type(_type) => match _type {
				AttributeType::Power => Self::power(),
				AttributeType::Finesse => Self::finesse(),
				AttributeType::Resistance => Self::resistance(),
			},
			AttributeCategory::Trait(_trait) => match _trait {
				TraitCategory::Mental => Self::mental(),
				TraitCategory::Physical => Self::physical(),
				TraitCategory::Social => Self::social(),
			},
		}
	}

	pub fn get_attr(_trait: &TraitCategory, _type: &AttributeType) -> Attribute {
		match _trait {
			TraitCategory::Mental => match _type {
				AttributeType::Power => Self::Intelligence,
				AttributeType::Finesse => Self::Wits,
				AttributeType::Resistance => Self::Resolve,
			},
			TraitCategory::Physical => match _type {
				AttributeType::Power => Self::Strength,
				AttributeType::Finesse => Self::Dexterity,
				AttributeType::Resistance => Self::Stamina,
			},
			TraitCategory::Social => match _type {
				AttributeType::Power => Self::Presence,
				AttributeType::Finesse => Self::Manipulation,
				AttributeType::Resistance => Self::Composure,
			},
		}
	}

	#[allow(clippy::trivially_copy_pass_by_ref)]
	pub fn get_type(&self) -> AttributeType {
		match self {
			Attribute::Intelligence => AttributeType::Power,
			Attribute::Wits => AttributeType::Finesse,
			Attribute::Resolve => AttributeType::Resistance,
			Attribute::Strength => AttributeType::Power,
			Attribute::Dexterity => AttributeType::Finesse,
			Attribute::Stamina => AttributeType::Resistance,
			Attribute::Presence => AttributeType::Power,
			Attribute::Manipulation => AttributeType::Finesse,
			Attribute::Composure => AttributeType::Resistance,
		}
	}
}

#[derive(
	Clone,
	Copy,
	Debug,
	Hash,
	PartialEq,
	PartialOrd,
	Eq,
	Ord,
	Serialize,
	Deserialize,
	AllVariants,
	VariantName,
)]
pub enum Skill {
	Academics,
	Computer,
	Crafts,
	Investigation,
	Medicine,
	Occult,
	Politics,
	Science,

	Athletics,
	Brawl,
	Drive,
	Firearms,
	Larceny,
	Stealth,
	Survival,
	Weaponry,

	AnimalKen,
	Empathy,
	Expression,
	Intimidation,
	Persuasion,
	Socialize,
	Streetwise,
	Subterfuge,
}

impl Skill {
	fn mental() -> [Skill; 8] {
		[
			Self::Academics,
			Self::Computer,
			Self::Crafts,
			Self::Investigation,
			Self::Medicine,
			Self::Occult,
			Self::Politics,
			Self::Science,
		]
	}

	fn physical() -> [Skill; 8] {
		[
			Self::Athletics,
			Self::Brawl,
			Self::Drive,
			Self::Firearms,
			Self::Larceny,
			Self::Stealth,
			Self::Survival,
			Self::Weaponry,
		]
	}

	fn social() -> [Skill; 8] {
		[
			Self::AnimalKen,
			Self::Empathy,
			Self::Expression,
			Self::Intimidation,
			Self::Persuasion,
			Self::Socialize,
			Self::Streetwise,
			Self::Subterfuge,
		]
	}

	pub fn get(cat: &TraitCategory) -> [Skill; 8] {
		match cat {
			TraitCategory::Mental => Self::mental(),
			TraitCategory::Physical => Self::physical(),
			TraitCategory::Social => Self::social(),
		}
	}
}

impl NameKey for Skill {
	fn name_key(&self) -> String {
		format!("skill.{}", self.name())
	}
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Trait {
	Speed,
	Defense,
	Initative,
	Perception,
	Health,
	Size,

	Beats,
	AlternateBeats,

	Armor(Option<Armor>),

	Willpower,
	Power,
	Fuel,
	Integrity,
}

impl Trait {
	pub fn name(&self) -> Option<&str> {
		match self {
			Trait::Speed => Some("speed"),
			Trait::Defense => Some("defense"),
			Trait::Initative => Some("initative"),
			Trait::Perception => Some("perception"),
			Trait::Health => Some("health"),
			Trait::Size => Some("size"),
			Trait::Beats => Some("beats"),
			Trait::Armor(_) => Some("armor"),
			Trait::Willpower => Some("willpower"),
			Trait::Power => None,
			Trait::Fuel => None,
			Trait::Integrity => None,
			Trait::AlternateBeats => None,
		}
	}
}
