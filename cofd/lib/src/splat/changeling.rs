use serde::{Deserialize, Serialize};

use super::{Merit, NameKey, Splat};
use crate::{
	character::{
		traits::{AttributeCategory, AttributeType},
		Character, Damage,
	},
	prelude::*,
};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct ChangelingData {
	pub attr_bonus: Option<Attribute>,
	pub regalia: Option<Regalia>,
	pub frailties: Vec<String>,
	pub clarity: Damage,
	pub contracts: Vec<Contract>,
}

impl ChangelingData {
	pub fn max_clarity(&self, character: &Character) -> u16 {
		let attributes = character.attributes();
		attributes.wits + attributes.composure
	}
}

#[derive(
	Clone, Serialize, Deserialize, Debug, PartialEq, Eq, VariantName, AllVariants, Default,
)]
pub enum Seeming {
	#[default]
	Beast,
	Darkling,
	Elemental,
	Fairest,
	Ogre,
	Wizened,
	_Custom(String, Regalia, AttributeType),
}

impl Seeming {
	pub fn get_favored_regalia(&self) -> &Regalia {
		match self {
			Seeming::Beast => &Regalia::Steed,
			Seeming::Darkling => &Regalia::Mirror,
			Seeming::Elemental => &Regalia::Sword,
			Seeming::Fairest => &Regalia::Crown,
			Seeming::Ogre => &Regalia::Shield,
			Seeming::Wizened => &Regalia::Jewels,
			Seeming::_Custom(_, regalia, ..) => regalia,
		}
	}

	pub fn get_favored_attributes(&self) -> [Attribute; 3] {
		Attribute::get(AttributeCategory::Type(match self {
			Seeming::Beast => AttributeType::Resistance,
			Seeming::Darkling => AttributeType::Finesse,
			Seeming::Elemental => AttributeType::Resistance,
			Seeming::Fairest => AttributeType::Power,
			Seeming::Ogre => AttributeType::Power,
			Seeming::Wizened => AttributeType::Finesse,
			Seeming::_Custom(.., _type) => _type.clone(),
		}))
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, VariantName, AllVariants)]
pub enum Court {
	Spring,
	Summer,
	Autumn,
	Winter,
	_Custom(String),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, VariantName, AllVariants)]
pub enum Kith {
	Artist,
	BrightOne,
	Chatelane,
	Gristlegrinder,
	Helldiver,
	Hunterheart,
	Leechfinger,
	Mirrorskin,
	Nightsinger,
	Notary,
	Playmate,
	Snowskin,
	_Custom(String),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, VariantName, AllVariants)]
pub enum Regalia {
	Crown,
	Jewels,
	Mirror,
	Shield,
	Steed,
	Sword,
	_Custom(String),
}

impl NameKey for Regalia {
	fn name_key(&self) -> String {
		format!("changeling.{}", self.name())
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, AllVariants, VariantName)]
pub enum ChangelingMerit {
	Mantle,
}

impl ChangelingMerit {
	pub fn is_available(&self, character: &Character) -> bool {
		matches!(character.splat, Splat::Changeling(..))
	}
}

impl From<ChangelingMerit> for Merit {
	fn from(merit: ChangelingMerit) -> Self {
		Merit::Changeling(merit)
	}
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Contract {
	pub name: String,
	pub goblin: bool,
	pub cost: String,
	pub dice: String,
	pub action: String,
	pub duration: String,
	pub loophole: String,
	pub seeming_benefit: String,
}
