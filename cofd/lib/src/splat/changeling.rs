

use serde::{Deserialize, Serialize};

use crate::{
	character::{AttributeCategory, AttributeType, Character, Damage},
	prelude::Attribute,
};

use super::{Merit, NameKey, Splat, XSplat, YSplat, ZSplat};

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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Seeming {
	Beast,
	Darkling,
	Elemental,
	Fairest,
	Ogre,
	Wizened,
	_Custom(String, Regalia, AttributeType),
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
			Seeming::_Custom(name, ..) => name,
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
			Seeming::_Custom(_, _, _type) => _type.clone(),
		}))
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

impl Kith {
	pub fn name(&self) -> &str {
		match self {
			Self::Artist => "artist",
			Self::BrightOne => "bright_one",
			Self::Chatelane => "chatelane",
			Self::Gristlegrinder => "gristlegrinder",
			Self::Helldiver => "helldiver",
			Self::Hunterheart => "hunterheart",
			Self::Leechfinger => "leechfinger",
			Self::Mirrorskin => "mirrorskin",
			Self::Nightsinger => "nightsinger",
			Self::Notary => "notary",
			Self::Playmate => "playmate",
			Self::Snowskin => "snowskin",
			Self::_Custom(name) => name,
		}
	}

	pub fn all() -> [Kith; 12] {
		[
			Self::Artist,
			Self::BrightOne,
			Self::Chatelane,
			Self::Gristlegrinder,
			Self::Helldiver,
			Self::Hunterheart,
			Self::Leechfinger,
			Self::Mirrorskin,
			Self::Nightsinger,
			Self::Notary,
			Self::Playmate,
			Self::Snowskin,
		]
	}
}
impl From<Kith> for ZSplat {
	fn from(kith: Kith) -> Self {
		ZSplat::Changeling(kith)
	}
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
}

impl NameKey for Regalia {
	fn name_key(&self) -> String {
		format!("changeling.{}", self.name())
	}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, Hash)]
pub enum ChangelingMerit {}

impl ChangelingMerit {
	pub fn all() -> Vec<ChangelingMerit> {
		vec![]
	}

	pub fn is_available(&self, character: &Character) -> bool {
		if let Splat::Changeling(_, _, _, _) = character.splat {
			true
		} else {
			false
		}
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
