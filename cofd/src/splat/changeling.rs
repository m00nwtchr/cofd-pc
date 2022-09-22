use serde::{Deserialize, Serialize};

use crate::character::{
	ability::Ability, Attribute, Modifier, ModifierTarget, ModifierValue, Trait,
};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Seeming {
	Beast,
	Darkling,
	Elemental,
	Fairest,
	Ogre,
	Wizened,
	_Custom(String, Regalia),
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
			Seeming::_Custom(_, regalia) => regalia,
		}
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Court {
	Spring,
	Summer,
	Autumn,
	Winter,
	_Custom(String),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Regalia {
	Crown,
	Jewels,
	Mirror,
	Shield,
	Steed,
	Sword,
	_Custom(String),
}
