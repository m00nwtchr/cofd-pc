use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;
use std::fmt::Display;

use self::ability::Ability;
use crate::{
	character::{AttributeType, Modifier, ModifierOp, ModifierTarget, ModifierValue, Skill, Trait},
	prelude::Attribute,
};

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

#[derive(Debug, Clone, Copy, Serialize)]
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

impl SplatType {
	pub fn name(&self) -> &str {
		to_variant_name(self).unwrap()
	}
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Splat {
	#[default]
	Mortal,
	// #[splat(
	// 	name = "vampire",
	// 	xsplat = "clan",
	// 	ysplat = "covenant",
	// 	zsplat = "bloodline",
	// 	virtue_anchor = "mask",
	// 	vice_anchor = "dirge",
	// 	ability = "disciplines",
	// 	st = "blood_potency"
	// )]
	Vampire(Clan, Option<Covenant>, Option<Bloodline>, VampireData),
	Werewolf(Option<Auspice>, Option<Tribe>, Option<Lodge>, WerewolfData),
	Mage(Path, Option<Order>, Option<Legacy>, MageData), // TODO: Order = free order status, high speech merit
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
			Splat::Vampire(_, _, _, _) => "vampire",
			Splat::Werewolf(_, _, _, _) => "werewolf",
			Splat::Mage(_, _, _, _) => "mage",
			Splat::Changeling(_, _, _, _) => "changeling",
		}
	}

	pub fn _type(&self) -> SplatType {
		match self {
			Splat::Mortal => SplatType::Mortal,
			Splat::Vampire(_, _, _, _) => SplatType::Vampire,
			Splat::Werewolf(_, _, _, _) => SplatType::Werewolf,
			Splat::Mage(_, _, _, _) => SplatType::Mage,
			Splat::Changeling(_, _, _, _) => SplatType::Changeling,
		}
	}

	pub fn virtue_anchor(&self) -> &str {
		match self {
			Splat::Vampire(_, _, _, _) => "mask",
			Splat::Werewolf(_, _, _, _) => "blood",
			Splat::Changeling(_, _, _, _) => "thread",
			_ => "virtue",
		}
	}

	pub fn vice_anchor(&self) -> &str {
		match self {
			Splat::Vampire(_, _, _, _) => "dirge",
			Splat::Werewolf(_, _, _, _) => "bone",
			Splat::Changeling(_, _, _, _) => "needle",
			_ => "vice",
		}
	}

	pub fn xsplat_name(&self) -> &str {
		match self {
			Splat::Mortal => "",
			Splat::Vampire(_, _, _, _) => "clan",
			Splat::Werewolf(_, _, _, _) => "auspice",
			Splat::Mage(_, _, _, _) => "path",
			Splat::Changeling(_, _, _, _) => "seeming",
		}
	}

	pub fn xsplat(&self) -> Option<XSplat> {
		match self {
			Splat::Mortal => None,
			Splat::Vampire(clan, _, _, _) => Some(clan.clone().into()),
			Splat::Werewolf(auspice, _, _, _) => auspice.clone().map(Into::into),
			Splat::Mage(path, _, _, _) => Some(path.clone().into()),
			Splat::Changeling(seeming, _, _, _) => Some(seeming.clone().into()),
		}
	}

	pub fn set_xsplat(&mut self, xsplat: Option<XSplat>) {
		match xsplat {
			Some(xsplat) => match xsplat {
				XSplat::Vampire(clan) => {
					if let Splat::Vampire(_clan, _, _, _) = self {
						*_clan = clan;
					}
				}
				XSplat::Werewolf(ausipce) => {
					if let Splat::Werewolf(_auspice, _, _, _) = self {
						*_auspice = Some(ausipce);
					}
				}
				XSplat::Mage(path) => {
					if let Splat::Mage(_path, _, _, _) = self {
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

	pub fn custom_xsplat(&self, name: String) -> Option<XSplat> {
		match self {
			Self::Mortal => None,
			Self::Vampire(..) => Some(
				Clan::_Custom(
					name,
					[
						Discipline::Animalism,
						Discipline::Auspex,
						Discipline::Celerity,
					],
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
					[ShadowGift::Death, ShadowGift::Dominance],
				)
				.into(),
			),
			Self::Mage(..) => {
				Some(Path::_Custom(name, [Arcanum::Death, Arcanum::Fate], Arcanum::Forces).into())
			}
			Self::Changeling(..) => {
				Some(Seeming::_Custom(name, Regalia::Crown, AttributeType::Power).into())
			}
		}
	}

	pub fn ysplat_name(&self) -> &str {
		match self {
			Splat::Mortal => "faction",
			Splat::Vampire(_, _, _, _) => "covenant",
			Splat::Werewolf(_, _, _, _) => "tribe",
			Splat::Mage(_, _, _, _) => "order",
			Splat::Changeling(_, _, _, _) => "court",
		}
	}

	pub fn ysplat(&self) -> Option<YSplat> {
		match self {
			Splat::Mortal => None,
			Splat::Vampire(_, covenant, _, _) => covenant.clone().map(Into::into),
			Splat::Werewolf(_, tribe, _, _) => tribe.clone().map(Into::into),
			Splat::Mage(_, order, _, _) => order.clone().map(Into::into),
			Splat::Changeling(_, court, _, _) => court.clone().map(Into::into),
		}
	}

	pub fn set_ysplat(&mut self, xsplat: Option<YSplat>) {
		match xsplat {
			Some(xsplat) => match xsplat {
				YSplat::Vampire(covenant) => {
					if let Splat::Vampire(_, _covenant, _, _) = self {
						*_covenant = Some(covenant);
					}
				}
				YSplat::Werewolf(tribe) => {
					if let Splat::Werewolf(_, _tribe, _, _) = self {
						*_tribe = Some(tribe);
					}
				}
				YSplat::Mage(order) => {
					if let Splat::Mage(_, _order, _, _) = self {
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
				Splat::Vampire(_, covenant, _, _) => *covenant = None,
				Splat::Werewolf(_, tribe, _, _) => *tribe = None,
				Splat::Mage(_, order, _, _) => *order = None,
				Splat::Changeling(_, court, _, _) => *court = None,
			},
		}
	}

	pub fn custom_ysplat(&self, name: String) -> Option<YSplat> {
		match self {
			Self::Mortal => None,
			Self::Vampire(_, _, _, _) => Some(Covenant::_Custom(name).into()),
			Self::Werewolf(_, _, _, _) => Some(
				Tribe::_Custom(
					name,
					Renown::Cunning,
					[
						ShadowGift::Death,
						ShadowGift::Dominance,
						ShadowGift::Elemental,
					],
				)
				.into(),
			),
			Self::Mage(_, _, _, _) => Some(
				Order::_Custom(name, [Skill::Academics, Skill::AnimalKen, Skill::Athletics]).into(),
			),
			Self::Changeling(_, _, _, _) => Some(Court::_Custom(name).into()),
		}
	}

	pub fn zsplat_name(&self) -> &str {
		match self {
			Splat::Mortal => "",
			Splat::Vampire(_, _, _, _) => "bloodline",
			Splat::Werewolf(_, _, _, _) => "lodge",
			Splat::Mage(_, _, _, _) => "legacy",
			Splat::Changeling(_, _, _, _) => "kith",
		}
	}

	pub fn zsplat(&self) -> Option<ZSplat> {
		match self {
			Splat::Mortal => None,
			Splat::Vampire(_, _, bloodline, _) => bloodline.clone().map(Into::into),
			Splat::Werewolf(_, _, lodge, _) => lodge.clone().map(Into::into),
			Splat::Mage(_, _, legacy, _) => legacy.clone().map(Into::into),
			Splat::Changeling(_, _, kith, _) => kith.clone().map(Into::into),
		}
	}

	pub fn set_zsplat(&mut self, zsplat: Option<ZSplat>) {
		match zsplat {
			Some(zsplat) => match zsplat {
				ZSplat::Vampire(bloodline) => {
					if let Splat::Vampire(_, _, _bloodline, _) = self {
						*_bloodline = Some(bloodline);
					}
				}
				ZSplat::Werewolf(lodge) => {
					if let Splat::Werewolf(_, _, _lodge, _) = self {
						*_lodge = Some(lodge);
					}
				}
				ZSplat::Mage(legacy) => {
					if let Splat::Mage(_, _, _legacy, _) = self {
						*_legacy = Some(legacy);
					}
				}
				ZSplat::Changeling(kith) => {
					if let Splat::Changeling(_, _, _kith, _) = self {
						*_kith = Some(kith);
					}
				}
			},
			None => match self {
				Splat::Mortal => {}
				Splat::Vampire(_, _, bloodline, _) => *bloodline = None,
				Splat::Werewolf(_, _, lodge, _) => *lodge = None,
				Splat::Mage(_, _, legacy, _) => *legacy = None,
				Splat::Changeling(_, _, kith, _) => *kith = None,
			},
		}
	}

	pub fn custom_zsplat(&self, name: String) -> Option<ZSplat> {
		match self {
			Splat::Mortal => None,
			Splat::Vampire(_, _, _, _) => Some(Bloodline::_Custom(name, None).into()),
			Splat::Werewolf(_, _, _, _) => Some(Lodge::_Custom(name).into()),
			Splat::Mage(_, _, _, _) => Some(Legacy::_Custom(name, None).into()),
			Splat::Changeling(_, _, _, _) => Some(Kith::_Custom(name).into()),
		}
	}

	pub fn ability_name(&self) -> Option<&str> {
		match self {
			Splat::Mortal => None,
			Splat::Vampire(_, _, _, _) => Some("disciplines"),
			Splat::Werewolf(_, _, _, _) => Some("renown"),
			Splat::Mage(_, _, _, _) => Some("arcana"),
			Splat::Changeling(_, _, _, _) => None,
		}
	}

	pub fn are_abilities_finite(&self) -> bool {
		match self {
			Splat::Mortal => true,
			Splat::Vampire(_, _, _, _) => false,
			Splat::Werewolf(_, _, _, _) => true,
			Splat::Mage(_, _, _, _) => true,
			Splat::Changeling(_, _, _, _) => false,
		}
	}

	pub fn all_abilities(&self) -> Option<Vec<Ability>> {
		match self {
			Splat::Mortal => None,
			Splat::Vampire(_, _, _, _) => Some(Vec::from(Discipline::all().map(Into::into))),
			Splat::Werewolf(_, _, _, _) => Some(Vec::from(Renown::all().map(Into::into))),
			Splat::Mage(_, _, _, _) => Some(Vec::from(Arcanum::all().map(Into::into))),
			Splat::Changeling(_, _, _, _) => None,
		}
	}

	pub fn custom_ability(&self, name: String) -> Option<Ability> {
		match self {
			Splat::Mortal => None,
			Splat::Vampire(_, _, _, _) => Some(Ability::Discipline(Discipline::_Custom(name))),
			Splat::Werewolf(_, _, _, _) => Some(Ability::MoonGift(MoonGift::_Custom(name))),
			Splat::Mage(_, _, _, _) => None,
			Splat::Changeling(_, _, _, _) => None,
		}
	}

	pub fn supernatural_tolerance(&self) -> Option<&str> {
		match self {
			Splat::Mortal => None,
			Splat::Vampire(_, _, _, _) => Some("blood_potency"),
			Splat::Werewolf(_, _, _, _) => Some("primal_urge"),
			Splat::Mage(_, _, _, _) => Some("gnosis"),
			Splat::Changeling(_, _, _, _) => Some("wyrd"),
		}
	}

	pub fn fuel(&self) -> Option<&str> {
		match self {
			Splat::Mortal => None,
			Splat::Vampire(_, _, _, _) => Some("vitae"),
			Splat::Werewolf(_, _, _, _) => Some("essence"),
			Splat::Mage(_, _, _, _) => Some("mana"),
			Splat::Changeling(_, _, _, _) => Some("glamour"),
		}
	}

	pub fn integrity(&self) -> &str {
		match self {
			Splat::Mortal => "integrity",
			Splat::Vampire(_, _, _, _) => "humanity",
			Splat::Werewolf(_, _, _, _) => "harmony",
			Splat::Mage(_, _, _, _) => "wisdom",
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

	pub fn name_mut(&mut self) -> Option<&mut String> {
		match self {
			Self::Vampire(Clan::_Custom(name, ..))
			| Self::Werewolf(Auspice::_Custom(name, ..))
			| Self::Mage(Path::_Custom(name, ..))
			| Self::Changeling(Seeming::_Custom(name, ..)) => Some(name),
			_ => None,
		}
	}

	pub fn all(_type: SplatType) -> Vec<XSplat> {
		match _type {
			SplatType::Vampire => Clan::all().map(Into::into).to_vec(),
			SplatType::Werewolf => Auspice::all().map(Into::into).to_vec(),
			SplatType::Mage => Path::all().map(Into::into).to_vec(),
			SplatType::Changeling => Seeming::all().map(Into::into).to_vec(),
			_ => vec![],
		}
	}

	pub fn is_custom(&self) -> bool {
		matches!(
			self,
			Self::Vampire(Clan::_Custom(..))
				| Self::Werewolf(Auspice::_Custom(..))
				| Self::Mage(Path::_Custom(..))
				| Self::Changeling(Seeming::_Custom(..))
		)
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

	pub fn name_mut(&mut self) -> Option<&mut String> {
		match self {
			Self::Vampire(Covenant::_Custom(name))
			| Self::Werewolf(Tribe::_Custom(name, _, _))
			| Self::Mage(
				Order::_Custom(name, _) | Order::SeersOfTheThrone(Some(Ministry::_Custom(name, _))),
			)
			| Self::Changeling(Court::_Custom(name)) => Some(name),
			_ => None,
		}
	}

	pub fn all(_type: SplatType) -> Vec<YSplat> {
		match _type {
			SplatType::Vampire => Covenant::all().map(Into::into).to_vec(),
			SplatType::Werewolf => Tribe::all().map(Into::into).to_vec(),
			SplatType::Mage => Order::all().map(Into::into).to_vec(),
			SplatType::Changeling => Court::all().map(Into::into).to_vec(),
			_ => vec![],
		}
	}

	pub fn is_custom(&self) -> bool {
		matches!(
			self,
			YSplat::Vampire(Covenant::_Custom(_))
				| YSplat::Werewolf(Tribe::_Custom(_, _, _))
				| YSplat::Mage(
					Order::_Custom(_, _) | Order::SeersOfTheThrone(Some(Ministry::_Custom(_, _))),
				) | YSplat::Changeling(Court::_Custom(_))
		)
	}
}

impl Display for YSplat {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.name())
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZSplat {
	Vampire(Bloodline),
	Werewolf(Lodge),
	Mage(Legacy),
	Changeling(Kith),
}

impl ZSplat {
	pub fn name(&self) -> &str {
		match self {
			Self::Vampire(bloodline) => bloodline.name(),
			Self::Werewolf(lodge) => lodge.name(),
			Self::Mage(legacy) => legacy.name(),
			Self::Changeling(kith) => kith.name(),
		}
	}

	pub fn name_mut(&mut self) -> Option<&mut String> {
		match self {
			ZSplat::Vampire(Bloodline::_Custom(name, _))
			| ZSplat::Werewolf(Lodge::_Custom(name))
			| ZSplat::Mage(Legacy::_Custom(name, _))
			| ZSplat::Changeling(Kith::_Custom(name)) => Some(name),
			_ => None,
		}
	}

	pub fn all(_type: SplatType) -> Vec<ZSplat> {
		match _type {
			SplatType::Changeling => Kith::all().map(Into::into).to_vec(),
			_ => vec![],
		}
	}

	pub fn is_custom(&self) -> bool {
		matches!(
			self,
			ZSplat::Vampire(Bloodline::_Custom(_, _))
				| ZSplat::Werewolf(Lodge::_Custom(_))
				| ZSplat::Mage(Legacy::_Custom(_, _))
				| ZSplat::Changeling(Kith::_Custom(_))
		)
	}
}

impl Display for ZSplat {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.name())
	}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, Hash)]

pub enum Merit {
	// Mental Merits
	AreaOfExpertise(String),
	CommonSense,
	DangerSense,
	DirectionSense,
	EideticMemory,
	EncyclopedicKnowledge(String),
	EyeForTheStrange,
	FastReflexes,
	GoodTimeManagement,
	HolisticAwareness,
	// HumanPrey,      // DTR
	// Hypervigilance, // DTR
	Indomitable,
	InterdisciplinarySpecialty(String, Option<Skill>),
	InvestigativeAide(Option<Skill>),
	InvestigativeProdigy,
	Language(String),
	Library(Option<Skill>),
	LibraryAdvanced(Vec<String>),
	// LucidDreamer, // CTL
	MeditativeMind,
	Multilingual(String, String),
	ObjectFetishism(String),
	Patient,
	// RenownedArtisan(String) // MTC
	Scarred(String), // TODO: Condition
	ToleranceForBiology,
	TrainedObserver,
	ViceRidden(String),
	Virtuous(String),

	// Physical Merits
	Ambidextrous,
	AutomotiveGenius,
	CovertOperative,
	CrackDriver,
	Demolisher,
	DoubleJointed,
	FleetOfFoot,
	Freediving,
	Giant,
	Hardy,
	Greyhound,
	// IronSkin, // BTP
	IronStamina,
	QuickDraw(String),
	PunchDrunk,
	Relentless,
	Roadkill,
	SeizingTheEdge,
	SleightOfHand,
	SmallFramed,
	Survivalist,

	// Social Merits
	AirOfMenace,
	Allies(String),
	AlternateIdentity(String),
	Anonymity,
	Barfly,
	ClosedBook,
	CohesiveUnit,
	Contacts(Vec<String>),
	Defender,
	Empath,
	Fame,
	Fixer,
	HobbyistClique(String, Option<Skill>),
	Inspiring,
	IronWill,
	Mentor(String, Option<[Skill; 3]>), // TODO: Add Resources to list
	Peacemaker,
	Pusher,
	Resources,
	Retainer(String),
	SafePlace(String),
	SmallUnitTactics,
	SpinDoctor,
	Staff,
	Status(String),
	StrikingLooks(String),
	SupportNetwork(String, Option<Box<Merit>>), // TODO: Restrict to social merits
	Sympathetic,
	TableTurner,
	TakesOneToKnowOne,
	Taste(String, Option<Skill>), // TODO: Restrict to Crafts/Expression
	TrueFriend(String),
	Untouchable,

	// Style Merits
	// Mental Styles
	ProfessionalTraining(String, Option<[Skill; 2]>, Option<Skill>),
	// Physical Styles
	AggresiveDriving,
	DroneControl,
	Falconry,
	K9,
	Parkour,
	StuntDriver,
	// Social Styles
	Etiquette,
	FastTalking,
	// MysteryCultInitation(String, _, Merit, _, Merit, _)
	// ScorpionCultInitation, // MTC

	// Fighting Merits
	DefensiveCombat(bool, Option<Skill>), // Brawl / Weaponry

	// Fighting Styles
	// ArmedDefense,
	// Avoidance,
	// Berserker,
	// Bowmanship,
	// Boxing,
	// BruteForce,
	// ChainWeapons,
	// CloseQuartersCombat,
	// CombatArchery,
	// DisablingTactics,
	// Firefight,
	// Grappling,
	// HeavyWeapons,
	// ImprovisedWeapons,
	// KinoMutai,
	// LightWeapons
	// Marksmanship
	// MaritalArts
	// MountedCombat
	// PoliceTactics
	// PoweredProjectile
	RelentlessAssault,
	// SpearAndBayonet
	// StaffFighting,
	// StreetFighting,
	// StrengthPerformance, // TODO: Give Giant?
	// Systema,
	// ThrownWepons,
	// TwoWeaponFighting,
	// UnarmedDefense,
	// WeaponAndShield
	Mage(MageMerit),
	Vampire(VampireMerit),
	Werewolf(WerewolfMerit),
	Changeling(ChangelingMerit),

	_Custom(String),
}

impl Merit {
	pub fn all() -> Vec<Merit> {
		let v = vec![
			// Mental Merits

			// Physical Merits

			// Social Merits

			// Style Merits
			// Mental Styles
			Self::ProfessionalTraining(String::new(), None, None),
			// Physical Styles
			Self::AggresiveDriving,
			Self::DroneControl,
			Self::Falconry,
			Self::K9,
			Self::Parkour,
			Self::StuntDriver,
			// Social Styles
			Self::Etiquette,
			Self::FastTalking,
			// MysteryCultInitation(String, _, Merit, _, Merit, _)
			// ScorpionCultInitation, // MTC

			// Fighting Merits
			Self::DefensiveCombat(false, None),
			// Fighting Styles
			// ArmedDefense,
			// Avoidance,
			// Berserker,
			// Bowmanship,
			// Boxing,
			// BruteForce,
			// ChainWeapons,
			// CloseQuartersCombat,
			// CombatArchery,
			// DisablingTactics,
			// Firefight,
			// Grappling,
			// HeavyWeapons,
			// ImprovisedWeapons,
			// KinoMutai,
			// LightWeapons
			// Marksmanship
			// MaritalArts
			// MountedCombat
			// PoliceTactics
			// PoweredProjectile
			Self::RelentlessAssault,
			// SpearAndBayonet
			// StaffFighting,
			// StreetFighting,
			// StrengthPerformance,
			// Systema,
			// ThrownWepons,
			// TwoWeaponFighting,
			// UnarmedDefense,
			// WeaponAndShield
		];

		let mut vec = Vec::new();

		vec.extend(Merit::mental());
		vec.extend(Merit::physical());
		vec.extend(Merit::social());

		vec.extend(v);

		vec
	}

	pub fn mental() -> Vec<Merit> {
		vec![
			Self::AreaOfExpertise(String::new()),
			Self::CommonSense,
			Self::DangerSense,
			Self::DirectionSense,
			Self::EideticMemory,
			Self::EncyclopedicKnowledge(String::new()),
			Self::EyeForTheStrange,
			Self::FastReflexes,
			Self::GoodTimeManagement,
			Self::HolisticAwareness,
			// Self::HumanPrey,      // DTR
			// Self::Hypervigilance, // DTR
			Self::Indomitable,
			Self::InterdisciplinarySpecialty(String::new(), None),
			Self::InvestigativeAide(None),
			Self::InvestigativeProdigy,
			Self::Language(String::new()),
			Self::Library(None),
			Self::LibraryAdvanced(vec![]),
			// LucidDreamer, // CTL
			Self::MeditativeMind,
			Self::Multilingual(String::new(), String::new()),
			Self::ObjectFetishism(String::new()),
			Self::Patient,
			// RenownedArtisan(String) // MTC
			Self::Scarred(String::new()),
			Self::ToleranceForBiology,
			Self::TrainedObserver,
			Self::ViceRidden(String::new()),
			Self::Virtuous(String::new()),
		]
	}

	pub fn physical() -> Vec<Merit> {
		vec![
			Self::Ambidextrous,
			Self::AutomotiveGenius,
			Self::CovertOperative,
			Self::CrackDriver,
			Self::Demolisher,
			Self::DoubleJointed,
			Self::FleetOfFoot,
			Self::Freediving,
			Self::Giant,
			Self::Hardy,
			Self::Greyhound,
			// IronSkin, // BTP
			Self::IronStamina,
			Self::QuickDraw(String::new()),
			Self::PunchDrunk,
			Self::Relentless,
			Self::Roadkill,
			Self::SeizingTheEdge,
			Self::SleightOfHand,
			Self::SmallFramed,
			Self::Survivalist,
		]
	}

	pub fn social() -> Vec<Merit> {
		vec![
			Self::AirOfMenace,
			Self::Allies(String::new()),
			Self::AlternateIdentity(String::new()),
			Self::Anonymity,
			Self::Barfly,
			Self::ClosedBook,
			Self::CohesiveUnit,
			Self::Contacts(vec![]),
			Self::Defender,
			Self::Empath,
			Self::Fame,
			Self::Fixer,
			Self::HobbyistClique(String::new(), None),
			Self::Inspiring,
			Self::IronWill,
			Self::Mentor(String::new(), None),
			Self::Peacemaker,
			Self::Pusher,
			Self::Resources,
			Self::Retainer(String::new()),
			Self::SafePlace(String::new()),
			Self::SmallUnitTactics,
			Self::SpinDoctor,
			Self::Staff,
			Self::Status(String::new()),
			Self::StrikingLooks(String::new()),
			Self::SupportNetwork(String::new(), None),
			Self::Sympathetic,
			Self::TableTurner,
			Self::TakesOneToKnowOne,
			Self::Taste(String::new(), None),
			Self::TrueFriend(String::new()),
			Self::Untouchable,
		]
	}

	pub fn get(splat: SplatType) -> Vec<Merit> {
		match splat {
			SplatType::Mortal => Merit::all(),
			SplatType::Vampire => VampireMerit::all().into_iter().map(Into::into).collect(),
			SplatType::Werewolf => WerewolfMerit::all().into_iter().map(Into::into).collect(),
			SplatType::Mage => MageMerit::all().into_iter().map(Into::into).collect(),
			SplatType::Changeling => ChangelingMerit::all().into_iter().map(Into::into).collect(),
		}
	}

	fn name(&self) -> &str {
		match self {
			Merit::Mage(merit) => to_variant_name(merit).unwrap(),
			Merit::Vampire(merit) => to_variant_name(merit).unwrap(),
			Merit::Werewolf(merit) => to_variant_name(merit).unwrap(),
			Merit::_Custom(name) => name,
			_ => to_variant_name(self).unwrap(),
		}
	}

	pub fn get_modifiers(&self, value: u16) -> Vec<Modifier> {
		match &self {
			Merit::DefensiveCombat(true, Some(skill)) => {
				vec![Modifier::new(
					ModifierTarget::Trait(Trait::DefenseSkill),
					ModifierValue::Skill(*skill),
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

impl Display for Merit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.name())
	}
}

impl From<Merit> for Ability {
	fn from(merit: Merit) -> Self {
		Ability::Merit(merit)
	}
}
