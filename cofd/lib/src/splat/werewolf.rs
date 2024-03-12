use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use cofd_util::VariantName;

use super::{ability::Ability, Merit, NameKey, Splat};
use crate::{
	character::modifier::{Modifier, ModifierOp, ModifierTarget},
	dice_pool::DicePool,
	prelude::*,
};

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct KuruthTriggerSet {
	pub passive: String,
	pub common: String,
	pub specific: String,
}

#[derive(Clone, VariantName)]
pub enum KuruthTrigger {
	Passive,
	Common,
	Specific,
}

impl KuruthTrigger {
	pub fn all(&self) -> [KuruthTrigger; 3] {
		[Self::Passive, Self::Common, Self::Specific]
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, AllVariants)]
pub enum KuruthTriggers {
	Blood,
	Moon,
	TheOther,
	Pack,
	Territory,
	Wound,
	_Custom(KuruthTriggerSet),
}

impl NameKey for KuruthTriggers {
	fn name_key(&self) -> String {
		if let Some(name) = self.name() {
			format!("kuruth-triggers.{name}")
		} else {
			"custom".to_string()
		}
	}
}

impl Default for KuruthTriggers {
	fn default() -> Self {
		Self::_Custom(Default::default())
	}
}

impl KuruthTriggers {
	pub fn name(&self) -> Option<&str> {
		match self {
			KuruthTriggers::Blood => Some("blood"),
			KuruthTriggers::Moon => Some("moon"),
			KuruthTriggers::TheOther => Some("the_other"),
			KuruthTriggers::Pack => Some("pack"),
			KuruthTriggers::Territory => Some("territory"),
			KuruthTriggers::Wound => Some("wound"),
			KuruthTriggers::_Custom(_) => None,
		}
	}

	pub fn get_triggers(&self) -> KuruthTriggerSet {
		match self {
			KuruthTriggers::Blood => KuruthTriggerSet {
				passive: "Smelling human blood.".to_owned(),
				common: "Tasting human blood.".to_owned(),
				specific: "Swallowing human blood.".to_owned(),
			},
			KuruthTriggers::Moon => KuruthTriggerSet {
				passive: "Your auspice moon is in the sky.".to_owned(),
				common: "You witness your auspice moon in the sky.".to_owned(),
				specific: "Hear a wolf or werewolf howl when your auspice moon is in the sky."
					.to_owned(),
			},
			KuruthTriggers::TheOther => KuruthTriggerSet {
				passive: "You come within 10 yards of a supernatural creature.".to_owned(),
				common: "You witness a supernatural creature doing something obviously inhuman."
					.to_owned(),
				specific: "You are the target of a supernatural power.".to_owned(),
			},
			KuruthTriggers::Pack => KuruthTriggerSet {
				passive: "A pack member takes lethal damage.".to_owned(),
				common: "Seeing someone attack a pack member.".to_owned(),
				specific: "You cause lethal damage to a pack member.".to_owned(),
			},
			KuruthTriggers::Territory => KuruthTriggerSet {
				passive: "A werewolf you don't know enters your territory without permission."
					.to_owned(),
				common: "You see a werewolf you don't know in your territory.".to_owned(),
				specific:
					"A werewolf you don't know challenges your pack's ability to do its duty."
						.to_owned(),
			},
			KuruthTriggers::Wound => KuruthTriggerSet {
				passive: "Being in the area of a Wound.".to_owned(),
				common: "Interacting with a Wound-born spirit.".to_owned(),
				specific: "Being attacked by a Wound-born spirit.".to_owned(),
			},
			KuruthTriggers::_Custom(set) => set.clone(),
		}
	}
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct WerewolfData {
	pub skill_bonus: Option<Skill>,
	pub form: Form,
	// pub moon_gifts: BTreeMap<MoonGift, AbilityVal>,
	pub triggers: KuruthTriggers,
	pub hunters_aspect: Option<HuntersAspect>,
	pub moon_gifts: HashMap<MoonGift, u16>,
	pub shadow_gifts: Vec<ShadowGift>,
	pub wolf_gifts: Vec<WolfGift>,
	pub rites: Vec<Rite>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, VariantName)]
pub enum HuntersAspect {
	Monstrous,
	Isolating,
	Blissful,
	Mystic,
	Dominant,

	Fanatical,
	Frenzied,
	Agnoized,
	Insidious,
	Implacable,
	Primal,

	_Custom(String),
}

impl NameKey for HuntersAspect {
	fn name_key(&self) -> String {
		format!("werewolf.{}", self.name())
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, VariantName, AllVariants)]
pub enum Auspice {
	Cahalith,
	Elodoth,
	Irraka,
	Ithaeur,
	Rahu,
	_Custom(
		String,
		[Skill; 3],
		Renown,
		MoonGift,
		Box<[ShadowGift; 2]>,
		HuntersAspect,
	),
}

impl Auspice {
	pub fn get_skills(&self) -> &[Skill; 3] {
		match self {
			Auspice::Cahalith => &[Skill::Crafts, Skill::Expression, Skill::Persuasion],
			Auspice::Elodoth => &[Skill::Empathy, Skill::Investigation, Skill::Politics],
			Auspice::Irraka => &[Skill::Larceny, Skill::Stealth, Skill::Subterfuge],
			Auspice::Ithaeur => &[Skill::AnimalKen, Skill::Medicine, Skill::Occult],
			Auspice::Rahu => &[Skill::Brawl, Skill::Intimidation, Skill::Survival],
			Auspice::_Custom(_, skills, ..) => skills,
		}
	}

	pub fn get_renown(&self) -> &Renown {
		match self {
			Auspice::Cahalith => &Renown::Glory,
			Auspice::Elodoth => &Renown::Honor,
			Auspice::Irraka => &Renown::Cunning,
			Auspice::Ithaeur => &Renown::Wisdom,
			Auspice::Rahu => &Renown::Purity,
			Auspice::_Custom(_, _, renown, ..) => renown,
		}
	}

	pub fn get_gifts(&self) -> &[ShadowGift; 2] {
		match self {
			Auspice::Cahalith => &[ShadowGift::Inspiration, ShadowGift::Knowledge],
			Auspice::Elodoth => &[ShadowGift::Insight, ShadowGift::Warding],
			Auspice::Irraka => &[ShadowGift::Evasion, ShadowGift::Stealth],
			Auspice::Ithaeur => &[ShadowGift::Elementals, ShadowGift::Shaping],
			Auspice::Rahu => &[ShadowGift::Dominance, ShadowGift::Strength],
			Auspice::_Custom(.., gifts, _) => gifts,
		}
	}

	pub fn get_moon_gift(&self) -> &MoonGift {
		match self {
			Auspice::Cahalith => &MoonGift::Gibbous,
			Auspice::Elodoth => &MoonGift::Half,
			Auspice::Irraka => &MoonGift::New,
			Auspice::Ithaeur => &MoonGift::Crescent,
			Auspice::Rahu => &MoonGift::Full,
			Auspice::_Custom(.., moon_gift, _, _) => moon_gift,
		}
	}

	pub fn get_hunters_aspect(&self) -> &HuntersAspect {
		match self {
			Auspice::Cahalith => &HuntersAspect::Monstrous,
			Auspice::Elodoth => &HuntersAspect::Isolating,
			Auspice::Irraka => &HuntersAspect::Blissful,
			Auspice::Ithaeur => &HuntersAspect::Mystic,
			Auspice::Rahu => &HuntersAspect::Dominant,
			Auspice::_Custom(.., aspect) => aspect,
		}
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, VariantName, AllVariants)]
pub enum PureTribe {
	FireTouched,
	IvoryClaws,
	PredatorKings,
	_Custom(
		String,
		Renown,
		[Renown; 2],
		[Skill; 3],
		[HuntersAspect; 2],
		Box<[ShadowGift; 4]>,
	),
}

impl PureTribe {
	pub fn get_secondary_renown(&self) -> &[Renown; 2] {
		match self {
			Self::FireTouched => &[Renown::Cunning, Renown::Glory],
			Self::IvoryClaws => &[Renown::Glory, Renown::Honor],
			Self::PredatorKings => &[Renown::Purity, Renown::Wisdom],
			Self::_Custom(_, _, renown, ..) => renown,
		}
	}

	pub fn get_skills(&self) -> &[Skill; 3] {
		match self {
			Self::FireTouched => &[Skill::Expression, Skill::Occult, Skill::Subterfuge],
			Self::IvoryClaws => &[Skill::Intimidation, Skill::Persuasion, Skill::Politics],
			Self::PredatorKings => &[Skill::AnimalKen, Skill::Brawl, Skill::Crafts],
			Self::_Custom(.., skills, _, _) => skills,
		}
	}

	pub fn get_hunters_aspects(&self) -> &[HuntersAspect; 2] {
		match self {
			Self::FireTouched => &[HuntersAspect::Fanatical, HuntersAspect::Frenzied],
			Self::IvoryClaws => &[HuntersAspect::Agnoized, HuntersAspect::Insidious],
			Self::PredatorKings => &[HuntersAspect::Implacable, HuntersAspect::Primal],
			Self::_Custom(.., aspects, _) => aspects,
		}
	}
}

impl From<PureTribe> for Tribe {
	fn from(pure: PureTribe) -> Self {
		Tribe::Pure(pure)
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, AllVariants, VariantName)]
pub enum Tribe {
	BloodTalons,
	BoneShadows,
	HuntersInDarkness,
	IronMasters,
	StormLords,
	#[expand]
	Pure(PureTribe),
	_Custom(String, Renown, Box<[ShadowGift; 3]>),
}

impl Tribe {
	pub fn get_renown(&self) -> &Renown {
		match self {
			Self::BloodTalons => &Renown::Glory,
			Self::BoneShadows => &Renown::Wisdom,
			Self::HuntersInDarkness => &Renown::Purity,
			Self::IronMasters => &Renown::Cunning,
			Self::StormLords => &Renown::Honor,
			// Tribe::GhostWolves => &None,
			Self::Pure(tribe) => match tribe {
				PureTribe::FireTouched => &Renown::Wisdom,
				PureTribe::IvoryClaws => &Renown::Purity,
				PureTribe::PredatorKings => &Renown::Glory,
				PureTribe::_Custom(_, renown, ..) => renown,
			},
			Self::_Custom(_, renown, _) => renown,
		}
	}

	pub fn get_gifts(&self) -> Vec<ShadowGift> {
		match self {
			Tribe::BloodTalons => vec![
				ShadowGift::Inspiration,
				ShadowGift::Rage,
				ShadowGift::Strength,
			],
			Tribe::BoneShadows => vec![
				ShadowGift::Death,
				ShadowGift::Elementals,
				ShadowGift::Insight,
			],
			Tribe::HuntersInDarkness => {
				vec![ShadowGift::Nature, ShadowGift::Stealth, ShadowGift::Warding]
			}
			Tribe::IronMasters => vec![
				ShadowGift::Knowledge,
				ShadowGift::Shaping,
				ShadowGift::Technology,
			],
			Tribe::StormLords => vec![
				ShadowGift::Evasion,
				ShadowGift::Dominance,
				ShadowGift::Weather,
			],
			Tribe::Pure(tribe) => match tribe {
				PureTribe::FireTouched => vec![
					ShadowGift::Disease,
					ShadowGift::Fervor,
					ShadowGift::Insight,
					ShadowGift::Inspiration,
				],
				PureTribe::IvoryClaws => vec![
					ShadowGift::Agony,
					ShadowGift::Blood,
					ShadowGift::Dominance,
					ShadowGift::Warding,
				],
				PureTribe::PredatorKings => vec![
					ShadowGift::Hunger,
					ShadowGift::Nature,
					ShadowGift::Rage,
					ShadowGift::Strength,
				],
				PureTribe::_Custom(.., gifts) => gifts.to_vec(),
			},
			// Tribe::GhostWolves => &None,
			Tribe::_Custom(.., gifts) => gifts.to_vec(),
		}
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, AllVariants, VariantName)]
pub enum Lodge {
	_Custom(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, VariantName, AllVariants)]
pub enum Renown {
	Purity,
	Glory,
	Honor,
	Wisdom,
	Cunning,
}

impl From<Renown> for Ability {
	fn from(val: Renown) -> Self {
		Ability::Renown(val)
	}
}

cofd_macros::gifts!();

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub enum Gift {
	Moon(MoonGift),
	Shadow(ShadowGift),
	Wolf(WolfGift),
}

impl MoonGift {
	pub fn get_modifiers(&self, value: u16) -> Vec<crate::character::modifier::Modifier> {
		match self {
			// MoonGift::Crescent => vec![],
			MoonGift::Full => {
				if value > 2 {
					vec![Modifier::new(
						Trait::Health,
						Ability::Renown(Renown::Purity),
						ModifierOp::Add,
					)]
				} else {
					vec![]
				}
			}
			// MoonGift::Gibbous => vec![],
			// MoonGift::Half => vec![],
			// MoonGift::New => vec![],
			// MoonGift::_Custom(_) => todo!(),
			_ => vec![],
		}
	}
}

impl NameKey for MoonGift {
	fn name_key(&self) -> String {
		format!("moon-gifts.{}", self.name())
	}
}

impl From<MoonGift> for Ability {
	fn from(gift: MoonGift) -> Self {
		Ability::MoonGift(gift)
	}
}

impl NameKey for ShadowGift {
	fn name_key(&self) -> String {
		format!("shadow-gifts.{}", self.name())
	}
}

impl NameKey for WolfGift {
	fn name_key(&self) -> String {
		format!("wolf-gifts.{}", self.name())
	}
}

#[derive(
	Clone,
	Debug,
	Serialize,
	Deserialize,
	Default,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Hash,
	VariantName,
	AllVariants,
)]
pub enum Form {
	#[default]
	Hishu,
	Dalu,
	Gauru,
	Urshul,
	Urhan,
}

impl Form {
	#[allow(clippy::too_many_lines)]
	pub fn get_modifiers(&self) -> Vec<Modifier> {
		match self {
			Form::Hishu => vec![Modifier::new(Trait::Perception, 1, ModifierOp::Add)],
			Form::Dalu => vec![
				Modifier::new(Attribute::Strength, 1, ModifierOp::Add),
				Modifier::new(Attribute::Stamina, 1, ModifierOp::Add),
				Modifier::new(Attribute::Manipulation, -1, ModifierOp::Add),
				Modifier::new(Trait::Size, 1, ModifierOp::Add),
				Modifier::new(Trait::Perception, 2, ModifierOp::Add),
			],
			Form::Gauru => vec![
				Modifier::new(Attribute::Strength, 3, ModifierOp::Add),
				Modifier::new(Attribute::Dexterity, 1, ModifierOp::Add),
				Modifier::new(Attribute::Stamina, 2, ModifierOp::Add),
				Modifier::new(Trait::Size, 2, ModifierOp::Add),
				Modifier::new(Trait::Perception, 3, ModifierOp::Add),
			],
			Form::Urshul => vec![
				Modifier::new(Attribute::Strength, 2, ModifierOp::Add),
				Modifier::new(Attribute::Dexterity, 2, ModifierOp::Add),
				Modifier::new(Attribute::Stamina, 2, ModifierOp::Add),
				Modifier::new(Attribute::Manipulation, -1, ModifierOp::Add),
				Modifier::new(Trait::Size, 1, ModifierOp::Add),
				Modifier::new(Trait::Speed, 3, ModifierOp::Add),
				Modifier::new(Trait::Perception, 3, ModifierOp::Add),
			],
			Form::Urhan => vec![
				Modifier::new(Attribute::Dexterity, 2, ModifierOp::Add),
				Modifier::new(Attribute::Stamina, 1, ModifierOp::Add),
				Modifier::new(Attribute::Manipulation, -1, ModifierOp::Add),
				Modifier::new(Trait::Size, -1, ModifierOp::Add),
				Modifier::new(Trait::Speed, 3, ModifierOp::Add),
				Modifier::new(Trait::Perception, 4, ModifierOp::Add),
			],
		}
	}

	pub fn modifiers() -> Vec<Modifier> {
		// match self {
		vec![
			//
			Modifier::conditional(Trait::Perception, 1, ModifierOp::Add, Form::Hishu),
			//
			Modifier::conditional(Attribute::Strength, 1, ModifierOp::Add, Form::Dalu),
			Modifier::conditional(Attribute::Stamina, 1, ModifierOp::Add, Form::Dalu),
			Modifier::conditional(Attribute::Manipulation, -1, ModifierOp::Add, Form::Dalu),
			Modifier::conditional(Trait::Size, 1, ModifierOp::Add, Form::Dalu),
			Modifier::conditional(Trait::Perception, 2, ModifierOp::Add, Form::Dalu),
			//
			Modifier::conditional(Attribute::Strength, 3, ModifierOp::Add, Form::Gauru),
			Modifier::conditional(Attribute::Dexterity, 1, ModifierOp::Add, Form::Gauru),
			Modifier::conditional(Attribute::Stamina, 2, ModifierOp::Add, Form::Gauru),
			Modifier::conditional(Trait::Size, 2, ModifierOp::Add, Form::Gauru),
			Modifier::conditional(Trait::Perception, 3, ModifierOp::Add, Form::Gauru),
			//
			Modifier::conditional(Attribute::Strength, 2, ModifierOp::Add, Form::Urshul),
			Modifier::conditional(Attribute::Dexterity, 2, ModifierOp::Add, Form::Urshul),
			Modifier::conditional(Attribute::Stamina, 2, ModifierOp::Add, Form::Urshul),
			Modifier::conditional(Attribute::Manipulation, -1, ModifierOp::Add, Form::Urshul),
			Modifier::conditional(Trait::Size, 1, ModifierOp::Add, Form::Urshul),
			Modifier::conditional(Trait::Speed, 3, ModifierOp::Add, Form::Urshul),
			Modifier::conditional(Trait::Perception, 3, ModifierOp::Add, Form::Urshul),
			//
			Modifier::conditional(Attribute::Dexterity, 2, ModifierOp::Add, Form::Urhan),
			Modifier::conditional(Attribute::Stamina, 1, ModifierOp::Add, Form::Urhan),
			Modifier::conditional(Attribute::Manipulation, -1, ModifierOp::Add, Form::Urhan),
			Modifier::conditional(Trait::Size, -1, ModifierOp::Add, Form::Urhan),
			Modifier::conditional(Trait::Speed, 3, ModifierOp::Add, Form::Urhan),
			Modifier::conditional(Trait::Perception, 4, ModifierOp::Add, Form::Urhan),
		]
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, VariantName)]
pub enum Rite {
	SacredHunt,
	_Custom(String),
}

impl NameKey for Rite {
	fn name_key(&self) -> String {
		format!("werewolf.{}", self.name())
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, AllVariants, VariantName)]
pub enum WerewolfMerit {
	FavoredForm {
		form: Form,
		//
	},
	EfficientKiller,
	Totem,

	InstinctiveDefense,
}

impl WerewolfMerit {
	pub fn is_available(&self, character: &crate::prelude::Character) -> bool {
		if matches!(character.splat, Splat::Werewolf(..)) {
			match self {
				Self::InstinctiveDefense => {
					character.power >= 2 && character.skills().athletics >= 2
				}
				_ => true,
			}
		} else {
			false
		}
	}

	pub fn get_modifiers(&self, value: u16) -> Vec<Modifier> {
		match self {
			Self::InstinctiveDefense => {
				if value == 2 {
					vec![
						Modifier::conditional(
							Trait::Defense,
							DicePool::max(Attribute::Wits, Attribute::Dexterity),
							ModifierOp::Set,
							Form::Urhan,
						),
						Modifier::conditional(
							Trait::Defense,
							DicePool::max(Attribute::Wits, Attribute::Dexterity),
							ModifierOp::Set,
							Form::Urshul,
						),
					]
				} else {
					vec![]
				}
			}
			_ => vec![],
		}
	}
}

impl From<WerewolfMerit> for Merit {
	fn from(merit: WerewolfMerit) -> Self {
		Merit::Werewolf(merit)
	}
}

pub fn get_form_trait(character: &Character, form: &Form, target: &ModifierTarget) -> i16 {
	let Splat::Werewolf(.., data) = &character.splat else {
		unreachable!()
	};
	let active_form = &data.form;

	let value = match target {
		ModifierTarget::BaseAttribute(_)
		| ModifierTarget::BaseSkill(_)
		| ModifierTarget::Skill(_) => unreachable!(),
		ModifierTarget::Attribute(attr) => *character.attributes().get(&attr) as i16,
		ModifierTarget::Trait(trait_) => character.get_trait(trait_) as i16,
	};

	if form.eq(active_form) {
		value
	} else {
		let modifiers = match target {
			ModifierTarget::Trait(trait_) => match trait_ {
				Trait::Speed => {
					form_modifier(character, form, &Attribute::Dexterity)
						+ form_modifier(character, form, &Attribute::Strength)
						- form_modifier(character, active_form, &Attribute::Dexterity)
						- form_modifier(character, active_form, &Attribute::Strength)
				}
				Trait::Initative => {
					form_modifier(character, form, &Attribute::Dexterity)
						+ form_modifier(character, form, &Attribute::Composure)
						- form_modifier(character, active_form, &Attribute::Dexterity)
						- form_modifier(character, active_form, &Attribute::Composure)
				}
				Trait::Defense => {
					// let active_form_pool = form_pool(character, active_form, target);
					// let form_pool = form_pool(character, form, target);

					// let attributes = character.attributes();
					// println!("{active_form_pool} - {form_pool}");

					// let attributes = character.attributes();
					// let dex = attributes.dexterity as i16
					// 	+ form_modifier(character, form, &Attribute::Dexterity)
					// 	- form_modifier(character, active_form, &Attribute::Dexterity);
					// let wits = attributes.wits as i16
					// 	+ form_modifier(character, form, &Attribute::Wits)
					// 	- form_modifier(character, active_form, &Attribute::Wits);

					// TODO: uhh make defense display in forms work.
					return value; // Active Form Defense
				}
				_ => 0,
			},
			_ => 0,
		};

		let active_form_modifier = form_modifier(character, active_form, target);
		let form_mod = form_modifier(character, form, target);

		// println!("{form:?} {target:?} = {value} - {active_form_modifier} + {form_mod}");
		value - active_form_modifier + form_mod + modifiers
	}
}

fn form_modifier(
	character: &Character,
	form: &Form,
	target: &(impl Into<ModifierTarget> + Clone),
) -> i16 {
	character
		.get_conditional_modifier((*target).clone(), form.clone())
		.unwrap_or(0)
}

fn form_pool(
	character: &Character,
	form: &Form,
	target: &(impl Into<ModifierTarget> + Clone),
) -> DicePool {
	character
		.get_conditional_pool((*target).clone(), form.clone())
		.unwrap_or(DicePool::Mod(0))
}
