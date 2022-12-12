use serde::{Deserialize, Serialize};

use self::ability::Ability;
use crate::{
	character::{AttributeType, Modifier, ModifierOp, ModifierTarget, ModifierValue, Skill, Trait},
	prelude::{Attribute, Character},
};

use cofd_traits::{AllVariants, SplatEnum, VariantName};

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

#[derive(Clone, Default, Serialize, Deserialize, Debug, VariantName, SplatEnum)]
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
		integrity = "humanity"
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
		integrity = "clarity"
	)]
	Changeling(Seeming, Option<Court>, Option<Kith>, Box<ChangelingData>),
	// Hunter(Tier),
	// Bound(Burden, Archetype),
	// Mummy(Decree, Guild),
	// Demon(Incarnation, Vec<Agenda>),
	// Beast(Hunger),
	// Deviant(Origin, Clade, Vec<Form>),
}

impl Splat {
	pub fn set_xsplat(&mut self, xsplat: Option<XSplat>) {
		match xsplat {
			Some(xsplat) => match xsplat {
				XSplat::Vampire(clan) => {
					if let Splat::Vampire(_clan, ..) = self {
						*_clan = clan;
					}
				}
				XSplat::Werewolf(ausipce) => {
					if let Splat::Werewolf(_auspice, ..) = self {
						*_auspice = Some(ausipce);
					}
				}
				XSplat::Mage(path) => {
					if let Splat::Mage(_path, ..) = self {
						*_path = path;
					}
				}
				XSplat::Changeling(seeming) => {
					if let Splat::Changeling(_seeming, ..) = self {
						*_seeming = seeming;
					}
				}
			},
			None => {
				if let Splat::Werewolf(auspice, ..) = self {
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
			Self::Vampire(..) => Some(Covenant::_Custom(name).into()),
			Self::Werewolf(..) => Some(
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
			Self::Mage(..) => Some(
				Order::_Custom(name, [Skill::Academics, Skill::AnimalKen, Skill::Athletics]).into(),
			),
			Self::Changeling(..) => Some(Court::_Custom(name).into()),
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
			Splat::Vampire(..) => Some(Bloodline::_Custom(name, None).into()),
			Splat::Werewolf(..) => Some(Lodge::_Custom(name).into()),
			Splat::Mage(..) => Some(Legacy::_Custom(name, None).into()),
			Splat::Changeling(..) => Some(Kith::_Custom(name).into()),
		}
	}

	pub fn are_abilities_finite(&self) -> bool {
		match self {
			Splat::Mortal => true,
			Splat::Vampire(..) => false,
			Splat::Werewolf(..) => true,
			Splat::Mage(..) => true,
			Splat::Changeling(..) => false,
		}
	}

	pub fn all_abilities(&self) -> Option<Vec<Ability>> {
		match self {
			Splat::Mortal => None,
			Splat::Vampire(..) => Some(Vec::from(Discipline::all().map(Into::into))),
			Splat::Werewolf(..) => Some(Vec::from(Renown::all().map(Into::into))),
			Splat::Mage(..) => Some(Vec::from(Arcanum::all().map(Into::into))),
			Splat::Changeling(..) => None,
		}
	}

	pub fn custom_ability(&self, name: String) -> Option<Ability> {
		match self {
			Splat::Mortal => None,
			Splat::Vampire(..) => Some(Ability::Discipline(Discipline::_Custom(name))),
			Splat::Werewolf(..) => Some(Ability::MoonGift(MoonGift::_Custom(name))),
			Splat::Mage(..) => None,
			Splat::Changeling(..) => None,
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
		}
	}
}

impl XSplat {
	pub fn name_mut(&mut self) -> Option<&mut String> {
		match self {
			Self::Vampire(Clan::_Custom(name, ..))
			| Self::Werewolf(Auspice::_Custom(name, ..))
			| Self::Mage(Path::_Custom(name, ..))
			| Self::Changeling(Seeming::_Custom(name, ..)) => Some(name),
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
		)
	}
}

impl YSplat {
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

impl ZSplat {
	pub fn name_mut(&mut self) -> Option<&mut String> {
		match self {
			ZSplat::Vampire(Bloodline::_Custom(name, _))
			| ZSplat::Werewolf(Lodge::_Custom(name))
			| ZSplat::Mage(Legacy::_Custom(name, _))
			| ZSplat::Changeling(Kith::_Custom(name)) => Some(name),
			_ => None,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, AllVariants, VariantName)]
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
	#[expand]
	Mage(MageMerit),
	#[expand]
	Vampire(VampireMerit),
	#[expand]
	Werewolf(WerewolfMerit),
	#[expand]
	Changeling(ChangelingMerit),

	_Custom(String),
}

impl Merit {
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

	pub fn is_available(&self, character: &Character) -> bool {
		match self {
			Merit::_Custom(_) => true,

			Merit::AreaOfExpertise(_) => character.attributes().resolve > 1,
			Merit::EyeForTheStrange => {
				character.attributes().resolve > 1 && character.skills().occult > 0
			}
			Merit::FastReflexes => {
				let attr = character.attributes();
				attr.wits > 2 || attr.dexterity > 2
			}
			Merit::GoodTimeManagement => {
				let skills = character.skills();
				skills.academics > 1 || skills.science > 1
			}
			Self::Indomitable => character.attributes().resolve > 2,
			Self::InterdisciplinarySpecialty(_, Some(skill)) => *character.skills().get(*skill) > 2,
			Self::InvestigativeAide(Some(skill)) => *character.skills().get(*skill) > 2,
			Self::InvestigativeProdigy => {
				character.attributes().wits > 2 && character.skills().investigation > 2
			}
			// Self::LibraryAdvanced() // Library 3 + <= Safe Place
			Self::Scarred(_) => character.integrity <= 5,
			Self::ToleranceForBiology => character.attributes().resolve > 2,
			Self::TrainedObserver => {
				let attrs = character.attributes();
				attrs.wits > 2 || attrs.composure > 2
			}
			Self::ViceRidden(_) if character.splat.vice_anchor() != "vice" => false,
			Self::Virtuous(_) if character.splat.virtue_anchor() != "virtue" => false,

			// Self::Ambidextrous // Character creation only
			Self::AutomotiveGenius => {
				let skills = character.skills();
				skills.crafts > 2 && skills.drive > 0 && skills.science > 0
			}
			Self::CovertOperative => {
				let attr = character.attributes();
				attr.wits > 2 && attr.dexterity > 2 && character.skills().stealth > 1
			}
			Self::CrackDriver => character.skills().drive > 2,
			Self::Demolisher => {
				let attr = character.attributes();
				attr.strength > 2 || attr.intelligence > 2
			}
			Self::DoubleJointed => character.attributes().dexterity > 2,
			Self::FleetOfFoot => character.skills().athletics > 1,
			Self::Freediving => character.skills().athletics > 1,
			// Self::Giant // Character Creation OR Strength Performance
			Self::Hardy => character.attributes().stamina > 2,
			Self::Greyhound => {
				let attr = character.attributes();
				character.skills().athletics > 2 && attr.wits > 2 && attr.stamina > 2
			}
			// IronSkin
			Self::IronStamina => {
				let attr = character.attributes();
				attr.stamina > 2 || attr.resolve > 2
			}
			Self::QuickDraw(_) => character.attributes().wits > 2,
			Self::PunchDrunk => character.max_willpower() > 5,
			Self::Relentless => {
				character.skills().athletics > 1 && character.attributes().stamina > 2
			}
			// Self::Roadkill // Merit Dep Aggresive Driving 2
			Self::SeizingTheEdge => {
				let attr = character.attributes();
				attr.wits > 2 && attr.composure > 2
			}
			Self::SleightOfHand => character.skills().larceny > 2,
			// Self::SmallFramed // Character Creation
			// Self::Survivalist => character.skills().survival > 2 // Iron Stamina 3 dependency
			Self::AirOfMenace => character.skills().intimidation > 1,
			// Self::Anonymity // No Fame Merit
			Self::Barfly => character.skills().socialize > 1,
			Self::ClosedBook => {
				let attr = character.attributes();
				attr.manipulation > 2 && attr.resolve > 2
			}
			Self::CohesiveUnit => character.attributes().presence > 2,
			Self::Empath => character.skills().empathy > 1,
			// Self::Fame // No Anonimity Merit
			// Self::Fixer => character.attributes().wits > 2 // Contacts 2
			Self::HobbyistClique(_, Some(skill)) => *character.skills().get(*skill) > 1,
			Self::Inspiring => character.attributes().presence > 2,
			Self::IronWill => character.attributes().resolve > 3,
			Self::Peacemaker => character.attributes().wits > 2 && character.skills().empathy > 2,
			Self::Pusher => character.skills().persuasion > 1,
			Self::SmallUnitTactics => character.attributes().presence > 2,
			Self::SpinDoctor => {
				character.attributes().manipulation > 2 && character.skills().subterfuge > 1
			}
			Self::TableTurner => {
				let attr = character.attributes();
				attr.composure > 2 && attr.manipulation > 2 && attr.wits > 2
			}
			Self::TakesOneToKnowOne if character.splat.vice_anchor() != "vice" => false,
			Self::Taste(_, _) => character.skills().crafts > 1,
			Self::Untouchable => {
				character.attributes().manipulation > 2 && character.skills().subterfuge > 1
			}

			// Self::Mage(merit) => merit.is_available(character),
			// Self::Vampire(merit) => merit.is_available(character),
			// Self::Werewolf(merit) => merit.is_available(character),
			// Self::Changeling(merit) => merit.is_available(character),
			_ => true,
		}
	}
}

impl NameKey for Merit {
	fn name_key(&self) -> String {
		format!("merits.{}", self.name())
	}
}

impl From<Merit> for Ability {
	fn from(merit: Merit) -> Self {
		Ability::Merit(merit)
	}
}

pub trait NameKey {
	fn name_key(&self) -> String;
}
