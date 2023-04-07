use serde::{Deserialize, Serialize};

use cofd_util::VariantName;

use super::{ability::Ability, Merit, Splat};
use crate::{
	character::modifier::{Modifier, ModifierOp},
	dice_pool::DicePool,
	prelude::{Attribute, Trait},
};

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct VampireData {
	pub attr_bonus: Option<Attribute>,
	pub banes: Vec<String>,
}

#[derive(
	Clone, Serialize, Deserialize, Debug, PartialEq, Eq, VariantName, AllVariants, Default,
)]
pub enum Clan {
	#[default]
	Daeva,
	Gangrel,
	Mekhet,
	Nosferatu,
	Ventrue,
	_Custom(String, Box<[Discipline; 3]>, [Attribute; 2]),
}

impl Clan {
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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, VariantName, AllVariants)]
pub enum Covenant {
	CarthianMovement,
	CircleOfTheCrone,
	Invictus,
	LanceaEtSanctum,
	OrdoDracul,
	_Custom(String),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, AllVariants, VariantName)]
pub enum Bloodline {
	_Custom(String, Option<[Discipline; 4]>),
}

#[derive(
	Clone,
	Debug,
	PartialEq,
	PartialOrd,
	Eq,
	Ord,
	Serialize,
	Deserialize,
	Hash,
	VariantName,
	AllVariants,
)]
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
	#[warn(clippy::cast_possible_wrap)]
	pub fn get_modifiers(&self, value: u16) -> Vec<crate::character::modifier::Modifier> {
		match self {
			Discipline::Celerity => {
				vec![Modifier::new(Trait::Defense, value, ModifierOp::Add)]
			}
			Discipline::Resilience => {
				vec![Modifier::new(Attribute::Stamina, value, ModifierOp::Add)]
			}
			Discipline::Vigor => vec![Modifier::new(Attribute::Strength, value, ModifierOp::Add)],
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, AllVariants, VariantName)]
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
	pub fn is_available(&self, character: &crate::prelude::Character) -> bool {
		matches!(character.splat, Splat::Vampire(..))
	}
}

impl From<VampireMerit> for Merit {
	fn from(merit: VampireMerit) -> Self {
		Merit::Vampire(merit)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Devotion {
	name: String,
	cost: String,
	disciplines: Vec<(Discipline, u16)>,
	dice_pool: DicePool,
	book: String,
	page: u16,
}
