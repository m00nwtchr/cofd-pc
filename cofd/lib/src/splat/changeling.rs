use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::character::{Character, Damage};

use super::{Merit, XSplat, YSplat};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChangelingData {
	pub regalia: Option<Regalia>,
	pub clarity: Damage,
}

impl ChangelingData {
	pub fn max_clarity(&self, character: &Character) -> u16 {
		let attributes = character.attributes();

		attributes.wits + attributes.composure
	}
}

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

impl Court {
	pub fn name(&self) -> &str {
		match self {
			Court::Spring => "spring",
			Court::Summer => "summer",
			Court::Autumn => "autumn",
			Court::Winter => "winter",
			Court::_Custom(name) => name,
		}
	}

	pub fn all() -> [Court; 4] {
		[Court::Spring, Court::Summer, Court::Autumn, Court::Winter]
	}
}

impl From<Court> for YSplat {
	fn from(court: Court) -> Self {
		YSplat::Changeling(court)
	}
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

impl Regalia {
	pub fn name(&self) -> &str {
		match self {
			Regalia::Crown => "crown",
			Regalia::Jewels => "jewels",
			Regalia::Mirror => "mirror",
			Regalia::Shield => "shield",
			Regalia::Steed => "steed",
			Regalia::Sword => "sword",
			Regalia::_Custom(name) => name,
		}
	}

	pub fn all() -> [Regalia; 6] {
		[
			Regalia::Crown,
			Regalia::Jewels,
			Regalia::Mirror,
			Regalia::Shield,
			Regalia::Steed,
			Regalia::Sword,
		]
	}
}

impl Display for Regalia {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.name())
	}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub enum ChangelingMerit {}

impl ChangelingMerit {
	pub fn all() -> Vec<ChangelingMerit> {
		vec![]
	}
}

impl From<ChangelingMerit> for Merit {
	fn from(merit: ChangelingMerit) -> Self {
		Merit::Changeling(merit)
	}
}
