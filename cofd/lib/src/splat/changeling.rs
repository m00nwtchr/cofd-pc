use serde::{Deserialize, Serialize};

use super::XSplat;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
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
	pub fn all() -> [Seeming; 6] {
		[
			Seeming::Beast,
			Seeming::Darkling,
			Seeming::Elemental,
			Seeming::Fairest,
			Seeming::Ogre,
			Seeming::Wizened,
		]
	}

	pub fn name(&self) -> &str {
		match self {
			Seeming::Beast => "beast",
			Seeming::Darkling => "darkling",
			Seeming::Elemental => "elemental",
			Seeming::Fairest => "fairest",
			Seeming::Ogre => "ogre",
			Seeming::Wizened => "wizened",
			Seeming::_Custom(name, _) => name,
		}
	}

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

impl From<Seeming> for XSplat {
	fn from(val: Seeming) -> Self {
		XSplat::Changeling(val)
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Court {
	Spring,
	Summer,
	Autumn,
	Winter,
	_Custom(String),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Regalia {
	Crown,
	Jewels,
	Mirror,
	Shield,
	Steed,
	Sword,
	_Custom(String),
}
