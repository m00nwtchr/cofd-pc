use serde::{Deserialize, Serialize};

use cofd_util::{AllVariants, VariantName};

use super::{
	ability::Ability, changeling::ChangelingMerit, mage::MageMerit, vampire::VampireMerit,
	werewolf::WerewolfMerit,
};
use crate::{
	character::modifier::{Modifier, ModifierOp, ModifierTarget, ModifierValue},
	prelude::{Attributes, Character, Skill, Skills, Trait},
};

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
	ProfessionalTraining {
		profession: String,
		skills: [Skill; 2],
		skill: Option<Skill>,
	},
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
					ModifierTarget::Trait(Trait::Defense),
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
			Merit::Werewolf(merit) => merit.get_modifiers(value),
			_ => vec![],
		}
	}

	pub fn is_available(
		&self,
		character: &crate::prelude::Character,
		attributes: &Attributes,
		skills: &Skills,
	) -> bool {
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
			Self::InterdisciplinarySpecialty(_, Some(skill)) => character.skills().get(*skill) > 2,
			Self::InvestigativeAide(Some(skill)) => character.skills().get(*skill) > 2,
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
			Self::HobbyistClique(_, Some(skill)) => character.skills().get(*skill) > 1,
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

			Self::Mage(merit) => merit.is_available(character),
			Self::Vampire(merit) => merit.is_available(character, attributes, skills),
			Self::Werewolf(merit) => merit.is_available(character),
			Self::Changeling(merit) => merit.is_available(character),
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
