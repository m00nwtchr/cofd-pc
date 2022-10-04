use std::fmt::Display;

use serde::{Deserialize, Serialize};

use self::ability::Ability;
use crate::character::{Modifier, ModifierOp, ModifierTarget, ModifierValue, Skill, Trait};

pub mod ability;

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
// use hunter::*;
// use geist::*;
// use mummy::*;
// use demon::*;
// use beast::*;
// use deviant:*;

#[derive(Debug, Clone)]
pub enum SplatType {
	Mortal,
	Vampire,
	Werewolf,
	Mage,
	// Promethean,
	Changeling,
	// Hunter,
	// Geist,
	// Mummy,
	// Demon,
	// Beast,
	// Deviant,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Splat {
	#[default]
	Mortal,
	Vampire(Clan, Option<Covenant>, Option<Bloodline>),
	Werewolf(Option<Auspice>, Option<Tribe>, Option<String>, WerewolfData),
	Mage(Path, Option<Order>, Option<Legacy>),
	// Promethean,
	Changeling(Seeming, Option<Court>, Option<Kith>, ChangelingData),
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
			Splat::Changeling(_, _, _, _) => "changeling",
		}
	}

	pub fn _type(&self) -> SplatType {
		match self {
			Splat::Mortal => SplatType::Mortal,
			Splat::Vampire(_, _, _) => SplatType::Vampire,
			Splat::Werewolf(_, _, _, _) => SplatType::Werewolf,
			Splat::Mage(_, _, _) => SplatType::Mage,
			Splat::Changeling(_, _, _, _) => SplatType::Changeling,
		}
	}

	pub fn virtue_anchor(&self) -> &str {
		match self {
			Splat::Vampire(_, _, _) => "mask",
			Splat::Werewolf(_, _, _, _) => "blood",
			Splat::Changeling(_, _, _, _) => "thread",
			_ => "virtue",
		}
	}

	pub fn vice_anchor(&self) -> &str {
		match self {
			Splat::Vampire(_, _, _) => "dirge",
			Splat::Werewolf(_, _, _, _) => "bone",
			Splat::Changeling(_, _, _, _) => "needle",
			_ => "vice",
		}
	}

	pub fn xsplat_name(&self) -> &str {
		match self {
			Splat::Mortal => "",
			Splat::Vampire(_, _, _) => "clan",
			Splat::Werewolf(_, _, _, _) => "auspice",
			Splat::Mage(_, _, _) => "path",
			Splat::Changeling(_, _, _, _) => "seeming",
		}
	}

	pub fn xsplat(&self) -> Option<XSplat> {
		match self.clone() {
			Splat::Mortal => None,
			Splat::Vampire(clan, _, _) => Some(clan.into()),
			Splat::Werewolf(auspice, _, _, _) => auspice.map(Into::into),
			Splat::Mage(path, _, _) => Some(path.into()),
			Splat::Changeling(seeming, _, _, _) => Some(seeming.into()),
		}
	}

	pub fn set_xsplat(&mut self, xsplat: Option<XSplat>) {
		match xsplat {
			Some(xsplat) => match xsplat {
				XSplat::Vampire(clan) => {
					if let Splat::Vampire(_clan, _, _) = self {
						*_clan = clan;
					}
				}
				XSplat::Werewolf(ausipce) => {
					if let Splat::Werewolf(_auspice, _, _, _) = self {
						*_auspice = Some(ausipce);
					}
				}
				XSplat::Mage(path) => {
					if let Splat::Mage(_path, _, _) = self {
						*_path = path;
					}
				}
				XSplat::Changeling(seeming) => {
					if let Splat::Changeling(_seeming, _, _, _) = self {
						*_seeming = seeming;
					}
				}
			},
			None => {
				if let Splat::Werewolf(auspice, _, _, _) = self {
					*auspice = None;
				}
			}
		}
	}

	pub fn ysplat_name(&self) -> &str {
		match self {
			Splat::Mortal => "faction",
			Splat::Vampire(_, _, _) => "covenant",
			Splat::Werewolf(_, _, _, _) => "tribe",
			Splat::Mage(_, _, _) => "order",
			Splat::Changeling(_, _, _, _) => "court",
		}
	}

	pub fn ysplat(&self) -> Option<YSplat> {
		match self.clone() {
			Splat::Mortal => None,
			Splat::Vampire(_, covenant, _) => covenant.map(Into::into),
			Splat::Werewolf(_, tribe, _, _) => tribe.map(Into::into),
			Splat::Mage(_, order, _) => order.map(Into::into),
			Splat::Changeling(_, court, _, _) => court.map(Into::into),
		}
	}

	pub fn set_ysplat(&mut self, xsplat: Option<YSplat>) {
		match xsplat {
			Some(xsplat) => match xsplat {
				YSplat::Vampire(covenant) => {
					if let Splat::Vampire(_, _covenant, _) = self {
						*_covenant = Some(covenant);
					}
				}
				YSplat::Werewolf(tribe) => {
					if let Splat::Werewolf(_, _tribe, _, _) = self {
						*_tribe = Some(tribe);
					}
				}
				YSplat::Mage(order) => {
					if let Splat::Mage(_, _order, _) = self {
						*_order = Some(order);
					}
				}
				YSplat::Changeling(court) => {
					if let Splat::Changeling(_, _court, _, _) = self {
						*_court = Some(court);
					}
				}
			},
			None => match self {
				Splat::Mortal => {}
				Splat::Vampire(_, covenant, _) => *covenant = None,
				Splat::Werewolf(_, tribe, _, _) => *tribe = None,
				Splat::Mage(_, order, _) => *order = None,
				Splat::Changeling(_, court, _, _) => *court = None,
			},
		}
	}

	pub fn zsplat_name(&self) -> &str {
		match self {
			Splat::Mortal => "",
			Splat::Vampire(_, _, _) => "bloodline",
			Splat::Werewolf(_, _, _, _) => "lodge",
			Splat::Mage(_, _, _) => "legacy",
			Splat::Changeling(_, _, _, _) => "kith",
		}
	}

	pub fn ability_name(&self) -> Option<&str> {
		match self {
			Splat::Mortal => None,
			Splat::Vampire(_, _, _) => Some("disciplines"),
			Splat::Werewolf(_, _, _, _) => Some("renown"),
			Splat::Mage(_, _, _) => Some("arcana"),
			Splat::Changeling(_, _, _, _) => None,
		}
	}

	pub fn are_abilities_finite(&self) -> bool {
		match self {
			Splat::Mortal => true,
			Splat::Vampire(_, _, _) => false,
			Splat::Werewolf(_, _, _, _) => true,
			Splat::Mage(_, _, _) => true,
			Splat::Changeling(_, _, _, _) => false,
		}
	}

	pub fn all_abilities(&self) -> Option<Vec<Ability>> {
		match self {
			Splat::Mortal => None,
			Splat::Vampire(_, _, _) => Some(Vec::from(Discipline::all().map(Into::into))),
			Splat::Werewolf(_, _, _, _) => Some(Vec::from(Renown::all().map(Into::into))),
			Splat::Mage(_, _, _) => Some(Vec::from(Arcanum::all().map(Into::into))),
			Splat::Changeling(_, _, _, _) => None,
		}
	}

	pub fn custom_ability(&self, str: String) -> Option<Ability> {
		match self {
			Splat::Mortal => None,
			Splat::Vampire(_, _, _) => Some(Ability::Discipline(Discipline::custom(str))),
			Splat::Werewolf(_, _, _, _) => Some(Ability::MoonGift(MoonGift::custom(str))),
			Splat::Mage(_, _, _) => None,
			Splat::Changeling(_, _, _, _) => None,
		}
	}

	pub fn supernatural_tolerance(&self) -> Option<&str> {
		match self {
			Splat::Mortal => None,
			Splat::Vampire(_, _, _) => Some("blood_potency"),
			Splat::Werewolf(_, _, _, _) => Some("primal_urge"),
			Splat::Mage(_, _, _) => Some("gnosis"),
			Splat::Changeling(_, _, _, _) => Some("wyrd"),
		}
	}

	pub fn fuel(&self) -> Option<&str> {
		match self {
			Splat::Mortal => None,
			Splat::Vampire(_, _, _) => Some("vitae"),
			Splat::Werewolf(_, _, _, _) => Some("essence"),
			Splat::Mage(_, _, _) => Some("mana"),
			Splat::Changeling(_, _, _, _) => Some("glamour"),
		}
	}

	pub fn integrity(&self) -> &str {
		match self {
			Splat::Mortal => "integrity",
			Splat::Vampire(_, _, _) => "humanity",
			Splat::Werewolf(_, _, _, _) => "harmony",
			Splat::Mage(_, _, _) => "wisdom",
			Splat::Changeling(_, _, _, _) => "clarity",
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XSplat {
	Vampire(Clan),
	Werewolf(Auspice),
	Mage(Path),
	Changeling(Seeming),
}

impl XSplat {
	pub fn name(&self) -> &str {
		match self {
			XSplat::Vampire(clan) => clan.name(),
			XSplat::Werewolf(auspice) => auspice.name(),
			XSplat::Mage(path) => path.name(),
			XSplat::Changeling(seeming) => seeming.name(),
		}
	}

	pub fn all(_type: &SplatType) -> Vec<XSplat> {
		match _type {
			SplatType::Vampire => Clan::all().map(Into::into).to_vec(),
			SplatType::Werewolf => Auspice::all().map(Into::into).to_vec(),
			SplatType::Mage => Path::all().map(Into::into).to_vec(),
			SplatType::Changeling => Seeming::all().map(Into::into).to_vec(),
			_ => vec![],
		}
	}
}

impl Display for XSplat {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.name())
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum YSplat {
	Vampire(Covenant),
	Werewolf(Tribe),
	Mage(Order),
	Changeling(Court),
}

impl YSplat {
	pub fn name(&self) -> &str {
		match self {
			YSplat::Vampire(covenant) => covenant.name(),
			YSplat::Werewolf(tribe) => tribe.name(),
			YSplat::Mage(order) => order.name(),
			YSplat::Changeling(court) => court.name(),
		}
	}

	pub fn all(_type: &SplatType) -> Vec<YSplat> {
		match _type {
			SplatType::Vampire => Covenant::all().map(Into::into).to_vec(),
			SplatType::Werewolf => Tribe::all().map(Into::into).to_vec(),
			SplatType::Mage => Order::all().map(Into::into).to_vec(),
			SplatType::Changeling => Court::all().map(Into::into).to_vec(),
			_ => vec![],
		}
	}
}

impl Display for YSplat {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.name())
	}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]

pub enum Merit {
	Status(String),
	FastTalking,
	ProfessionalTraining(String, [Skill; 2], Option<Skill>),
	Contacts(String),
	SafePlace(Option<String>),
	Resources,
	SleightOfHand,
	StrikingLooks(String),
	TrainedObserver,
	Language(String),

	Giant,

	// Fighting Merits
	DefensiveCombat(bool, Skill),

	// Vampire
	FeedingGrounds,
	CacophonySavvy,
	HoneyTrap,
	//
	NestGuardian,

	// Werewolf
	FavoredForm(Form),
	RelentlessAssault,
	EfficientKiller,
	Totem,

	_Custom(String),
}

impl Merit {
	pub fn custom(str: String) -> Merit {
		Merit::_Custom(str)
	}

	fn name(&self) -> &str {
		""
	}

	pub fn get_modifiers(&self, value: u8) -> Vec<Modifier> {
		match &self {
			Merit::DefensiveCombat(true, skill) => {
				vec![Modifier::new(
					ModifierTarget::Trait(Trait::DefenseSkill),
					ModifierValue::Skill(skill.clone()),
					ModifierOp::Set,
				)]
			}
			Merit::Giant => {
				if value == 3 {
					vec![Modifier::new(
						ModifierTarget::Trait(Trait::Size),
						ModifierValue::Num(1),
						ModifierOp::Add,
					)]
				} else {
					vec![]
				}
			}
			_ => vec![],
		}
	}
}
