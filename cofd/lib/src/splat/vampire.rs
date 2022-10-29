use serde::{Deserialize, Serialize};

use crate::character::{Attribute, Modifier, ModifierOp, ModifierTarget, ModifierValue, Trait};

use super::{ability::Ability, Merit, XSplat, YSplat, ZSplat};

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct VampireData {
	pub banes: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Clan {
	Daeva,
	Gangrel,
	Mekhet,
	Nosferatu,
	Ventrue,
	_Custom(String, [Discipline; 3], [Attribute; 2]),
}

impl Clan {
	pub fn name(&self) -> &str {
		match self {
			Clan::Daeva => "daeva",
			Clan::Gangrel => "gangrel",
			Clan::Mekhet => "mekhet",
			Clan::Nosferatu => "nosferatu",
			Clan::Ventrue => "ventrue",
			Clan::_Custom(name, _, _) => name,
		}
	}

	pub fn all() -> [Clan; 5] {
		[
			Clan::Daeva,
			Clan::Gangrel,
			Clan::Mekhet,
			Clan::Nosferatu,
			Clan::Ventrue,
		]
	}

	pub fn get_disciplines(&self) -> &[Discipline; 3] {
		match self {
			Clan::Daeva => &[Discipline::Celerity, Discipline::Majesty, Discipline::Vigor],
			Clan::Gangrel => &[
				Discipline::Animalism,
				Discipline::Protean,
				Discipline::Resilience,
			],
			Clan::Mekhet => &[
				Discipline::Auspex,
				Discipline::Celerity,
				Discipline::Obfuscate,
			],
			Clan::Nosferatu => &[
				Discipline::Nightmare,
				Discipline::Obfuscate,
				Discipline::Vigor,
			],
			Clan::Ventrue => &[
				Discipline::Animalism,
				Discipline::Dominate,
				Discipline::Resilience,
			],
			Clan::_Custom(_, disciplines, _) => disciplines,
		}
	}
	pub fn get_favored_attributes(&self) -> &[Attribute; 2] {
		match self {
			Clan::Daeva => &[Attribute::Dexterity, Attribute::Manipulation],
			Clan::Gangrel => &[Attribute::Composure, Attribute::Stamina],
			Clan::Mekhet => &[Attribute::Intelligence, Attribute::Wits],
			Clan::Nosferatu => &[Attribute::Composure, Attribute::Strength],
			Clan::Ventrue => &[Attribute::Presence, Attribute::Resolve],
			Clan::_Custom(_, _, attributes) => attributes,
		}
	}
}

impl From<Clan> for XSplat {
	fn from(val: Clan) -> Self {
		XSplat::Vampire(val)
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Covenant {
	CarthianMovement,
	CircleOfTheCrone,
	Invictus,
	LanceaEtSanctum,
	OrdoDracul,
	_Custom(String),
}

impl Covenant {
	pub fn name(&self) -> &str {
		match self {
			Covenant::CarthianMovement => "carthian_movement",
			Covenant::CircleOfTheCrone => "circle_of_the_crone",
			Covenant::Invictus => "invictus",
			Covenant::LanceaEtSanctum => "lancea_et_sanctum",
			Covenant::OrdoDracul => "ordo_dracul",
			Covenant::_Custom(name) => name,
		}
	}

	pub fn all() -> [Covenant; 5] {
		[
			Covenant::CarthianMovement,
			Covenant::CircleOfTheCrone,
			Covenant::Invictus,
			Covenant::LanceaEtSanctum,
			Covenant::OrdoDracul,
		]
	}
}

impl From<Covenant> for YSplat {
	fn from(covenant: Covenant) -> Self {
		YSplat::Vampire(covenant)
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Bloodline {
	_Custom(String, Option<[Discipline; 4]>),
}

impl Bloodline {
	pub fn name(&self) -> &str {
		match self {
			Bloodline::_Custom(name, _) => name,
		}
	}
}
impl From<Bloodline> for ZSplat {
	fn from(bloodline: Bloodline) -> Self {
		ZSplat::Vampire(bloodline)
	}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, Hash)]
pub enum Discipline {
	Animalism,
	Auspex,
	Celerity,
	Dominate,
	Majesty,
	Nightmare,
	Obfuscate,
	Protean,
	Resilience,
	Vigor,
	_Custom(String),
}

impl Discipline {
	pub fn all() -> [Discipline; 10] {
		[
			Discipline::Animalism,
			Discipline::Auspex,
			Discipline::Celerity,
			Discipline::Dominate,
			Discipline::Majesty,
			Discipline::Nightmare,
			Discipline::Obfuscate,
			Discipline::Protean,
			Discipline::Resilience,
			Discipline::Vigor,
		]
	}

	pub fn name(&self) -> &str {
		match self {
			Discipline::Animalism => "animalism",
			Discipline::Auspex => "auspex",
			Discipline::Celerity => "celerity",
			Discipline::Dominate => "dominate",
			Discipline::Majesty => "majesty",
			Discipline::Nightmare => "nightmare",
			Discipline::Obfuscate => "obfuscate",
			Discipline::Protean => "protean",
			Discipline::Resilience => "resilience",
			Discipline::Vigor => "vigor",
			Discipline::_Custom(name) => name,
		}
	}

	#[warn(clippy::cast_possible_wrap)]
	pub fn get_modifiers(&self, value: &u16) -> Vec<crate::character::Modifier> {
		match self {
			Discipline::Celerity => {
				vec![Modifier::new(
					ModifierTarget::Trait(Trait::Defense),
					ModifierValue::Num(*value as i16),
					ModifierOp::Add,
				)]
			}
			Discipline::Resilience => vec![Modifier::new(
				ModifierTarget::Attribute(Attribute::Stamina),
				ModifierValue::Num(*value as i16),
				ModifierOp::Add,
			)],
			Discipline::Vigor => vec![Modifier::new(
				ModifierTarget::Attribute(Attribute::Strength),
				ModifierValue::Num(*value as i16),
				ModifierOp::Add,
			)],
			_ => vec![],
		}
	}
}

impl From<Discipline> for Ability {
	fn from(val: Discipline) -> Self {
		Ability::Discipline(val)
	}
}

pub enum MaskDirge {
	Authoritarian,
	Child,
	Competitor,
	Conformist,
	Conspirator,
	Courtesan,
	CultLeader,
	Deviant,
	Follower,
	Guru,
	Idealist,
	Jester,
	Junkie,
	Martyr,
	Masochist,
	Monster,
	Nomad,
	Nurturer,
	Perfectionist,
	Penitent,
	Questioner,
	Rebel,
	Scholar,
	SocialChameleon,
	Spy,
	Survivor,
	Visionary,
	_Custom(String),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, Hash)]
pub enum VampireMerit {
	AcuteSenses,
	Atrocious,
	// Beloved, // TY
	Bloodhound,
	// CallTheBeast // TY
	ClawsOfTheUnholy,
	CloseFamily,
	Cutthroat,
	DistinguishedPalate,
	DreamVisions, // Mekhet
	Enticing,
	FeedingGrounds(String),
	// HeartOfStone, // TY
	Herd,
	HoneyTrap,
	KindredStatus(String), // Status?
	KissOfTheSuccubus,     // Daeva
	Lineage(String),
	LingeringDreams, // DE2
	// MajorDomo, // TY
	// MarriedByBlood, // TY
	PackAlpha, // Gangrel
	// ReceptiveMind, // TY
	// RevenantImpostor, // HD
	// SaviorOfTheLost, // TY
	// SpecialTreat, // TY
	SwarmForm,
	Touchstone,
	UndeadMenses,
	UnnaturalAffinity,
	UnsettlingGaze,

	CacophonySavvy,
	Courtoisie,
	Crusade,
	DynastyMembership,
	KindredDueling,
	MobilizeOutrage, // SotC, Carthian
	RidingTheWave,
	RitesOfTheImpaled, // SotC, Ordo, Sworn
	TempleGuardian,    // SotC, Crone

	// Elder Merits,

	// Revenant Merits

	// Covenant Merits

	// Ordo Dracul
	IndependentStudy, // SotC
	SecretSocietyJunkie,
	Sworn(String),
	TwilightJudge, // SotC

	NestGuardian,
}

impl VampireMerit {
	pub fn all() -> Vec<VampireMerit> {
		vec![
			Self::AcuteSenses,
			Self::Atrocious,
			Self::Bloodhound,
			Self::ClawsOfTheUnholy,
			Self::CloseFamily,
			Self::Cutthroat,
			Self::DistinguishedPalate,
			Self::DreamVisions, // Mekhet
			Self::Enticing,
			Self::FeedingGrounds(String::new()),
			Self::Herd,
			Self::HoneyTrap,
			Self::KindredStatus(String::new()), // Status?
			Self::KissOfTheSuccubus,            // Daeva
			Self::Lineage(String::new()),
			Self::LingeringDreams, // DE2
			Self::PackAlpha,       // Gangrel
			// RevenantImpostor, // HD
			Self::SwarmForm,
			Self::Touchstone,
			Self::UndeadMenses,
			Self::UnnaturalAffinity,
			Self::UnsettlingGaze,
			// Style Merits
			Self::CacophonySavvy,
			Self::Courtoisie,
			Self::Crusade,
			Self::DynastyMembership,
			Self::KindredDueling,
			Self::MobilizeOutrage, // SotC, Carthian
			Self::RidingTheWave,
			Self::RitesOfTheImpaled, // SotC, Ordo, Sworn
			Self::TempleGuardian,    // SotC, Crone
			// Elder Merits,
			// Beloved, // TY
			// CallTheBeast // TY
			// HeartOfStone, // TY
			// MajorDomo, // TY
			// MarriedByBlood, // TY
			// ReceptiveMind, // TY

			// SaviorOfTheLost, // TY
			// SpecialTreat, // TY

			// Revenant Merits

			// Covenant Merits

			// Ordo Dracul
			Self::IndependentStudy, // SotC
			Self::SecretSocietyJunkie,
			Self::Sworn(String::new()),
			Self::TwilightJudge, // SotC
			Self::NestGuardian,
		]
	}
}

impl From<VampireMerit> for Merit {
	fn from(merit: VampireMerit) -> Self {
		Merit::Vampire(merit)
	}
}
