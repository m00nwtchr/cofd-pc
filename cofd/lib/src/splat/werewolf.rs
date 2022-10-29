use serde::{Deserialize, Serialize};

use crate::character::{
	Attribute, Modifier, ModifierOp, ModifierTarget, ModifierValue, Skill, Trait,
};

use super::{ability::Ability, Merit, XSplat, YSplat, ZSplat};

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct WerewolfData {
	pub form: Form,
	// pub moon_gifts: BTreeMap<MoonGift, AbilityVal>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Auspice {
	Cahalith,
	Elodoth,
	Irraka,
	Ithaeur,
	Rahu,
	_Custom(String, [Skill; 3], Renown, MoonGift, [ShadowGift; 2]),
}

impl Auspice {
	pub fn all() -> [Auspice; 5] {
		[
			Auspice::Cahalith,
			Auspice::Elodoth,
			Auspice::Irraka,
			Auspice::Ithaeur,
			Auspice::Rahu,
		]
	}

	pub fn name(&self) -> &str {
		match self {
			Auspice::Cahalith => "cahalith",
			Auspice::Elodoth => "elodoth",
			Auspice::Irraka => "irraka",
			Auspice::Ithaeur => "ithaeur",
			Auspice::Rahu => "rahu",
			Auspice::_Custom(name, _, _, _, _) => name,
		}
	}

	pub fn get_skills(&self) -> &[Skill; 3] {
		match self {
			Auspice::Cahalith => &[Skill::Crafts, Skill::Expression, Skill::Persuasion],
			Auspice::Elodoth => &[Skill::Empathy, Skill::Investigation, Skill::Politics],
			Auspice::Irraka => &[Skill::Larceny, Skill::Stealth, Skill::Subterfuge],
			Auspice::Ithaeur => &[Skill::AnimalKen, Skill::Medicine, Skill::Occult],
			Auspice::Rahu => &[Skill::Brawl, Skill::Intimidation, Skill::Survival],
			Auspice::_Custom(_, skills, _, _, _) => skills,
		}
	}

	pub fn get_renown(&self) -> &Renown {
		match self {
			Auspice::Cahalith => &Renown::Glory,
			Auspice::Elodoth => &Renown::Honor,
			Auspice::Irraka => &Renown::Cunning,
			Auspice::Ithaeur => &Renown::Wisdom,
			Auspice::Rahu => &Renown::Purity,
			Auspice::_Custom(_, _, renown, _, _) => renown,
		}
	}

	pub fn get_gifts(&self) -> &[ShadowGift; 2] {
		match self {
			Auspice::Cahalith => &[ShadowGift::Inspiration, ShadowGift::Knowledge],
			Auspice::Elodoth => &[ShadowGift::Insight, ShadowGift::Warding],
			Auspice::Irraka => &[ShadowGift::Evasion, ShadowGift::Stealth],
			Auspice::Ithaeur => &[ShadowGift::Elemental, ShadowGift::Shaping],
			Auspice::Rahu => &[ShadowGift::Dominance, ShadowGift::Strength],
			Auspice::_Custom(_, _, _, _, gifts) => gifts,
		}
	}

	pub fn get_moon_gift(&self) -> &MoonGift {
		match self {
			Auspice::Cahalith => &MoonGift::Gibbous,
			Auspice::Elodoth => &MoonGift::Half,
			Auspice::Irraka => &MoonGift::New,
			Auspice::Ithaeur => &MoonGift::Crescent,
			Auspice::Rahu => &MoonGift::Full,
			Auspice::_Custom(_, _, _, moon_gift, _) => moon_gift,
		}
	}
}

impl From<Auspice> for XSplat {
	fn from(val: Auspice) -> Self {
		XSplat::Werewolf(val)
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Tribe {
	BloodTalons,
	BoneShadows,
	HuntersInDarkness,
	IronMasters,
	StormLords,
	_Custom(String, Renown, [ShadowGift; 3]),
}

impl Tribe {
	pub fn get_renown(&self) -> &Renown {
		match self {
			Tribe::BloodTalons => &Renown::Glory,
			Tribe::BoneShadows => &Renown::Wisdom,
			Tribe::HuntersInDarkness => &Renown::Purity,
			Tribe::IronMasters => &Renown::Cunning,
			Tribe::StormLords => &Renown::Honor,
			// Tribe::GhostWolves => &None,
			Tribe::_Custom(_, renown, _) => renown,
		}
	}

	pub fn get_gifts(&self) -> &[ShadowGift; 3] {
		match self {
			Tribe::BloodTalons => &[
				ShadowGift::Inspiration,
				ShadowGift::Rage,
				ShadowGift::Strength,
			],
			Tribe::BoneShadows => &[
				ShadowGift::Death,
				ShadowGift::Elemental,
				ShadowGift::Insight,
			],
			Tribe::HuntersInDarkness => {
				&[ShadowGift::Nature, ShadowGift::Stealth, ShadowGift::Warding]
			}
			Tribe::IronMasters => &[
				ShadowGift::Knowledge,
				ShadowGift::Shaping,
				ShadowGift::Technology,
			],
			Tribe::StormLords => &[
				ShadowGift::Evasion,
				ShadowGift::Dominance,
				ShadowGift::Weather,
			],
			// Tribe::GhostWolves => &None,
			Tribe::_Custom(_, _, gifts) => gifts,
		}
	}

	pub fn name(&self) -> &str {
		match self {
			Tribe::BloodTalons => "blood_talons",
			Tribe::BoneShadows => "bone_shadows",
			Tribe::HuntersInDarkness => "hunters_in_darkness",
			Tribe::IronMasters => "iron_masters",
			Tribe::StormLords => "storm_lords",
			Tribe::_Custom(name, _, _) => name,
		}
	}

	pub fn all() -> [Tribe; 5] {
		[
			Tribe::BloodTalons,
			Tribe::BoneShadows,
			Tribe::HuntersInDarkness,
			Tribe::IronMasters,
			Tribe::StormLords,
		]
	}
}

impl From<Tribe> for YSplat {
	fn from(tribe: Tribe) -> Self {
		YSplat::Werewolf(tribe)
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Lodge {
	_Custom(String),
}

impl Lodge {
	pub fn name(&self) -> &str {
		match self {
			Self::_Custom(name) => name,
		}
	}
}

impl From<Lodge> for ZSplat {
	fn from(lodge: Lodge) -> Self {
		ZSplat::Werewolf(lodge)
	}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, Hash)]
pub enum Renown {
	Purity,
	Glory,
	Honor,
	Wisdom,
	Cunning,
}

impl Renown {
	pub fn all() -> [Renown; 5] {
		[
			Renown::Purity,
			Renown::Glory,
			Renown::Honor,
			Renown::Wisdom,
			Renown::Cunning,
		]
	}

	pub fn name(&self) -> &str {
		match self {
			Renown::Purity => "purity",
			Renown::Glory => "glory",
			Renown::Honor => "honor",
			Renown::Wisdom => "wisdom",
			Renown::Cunning => "cunning",
		}
	}
}

impl From<Renown> for Ability {
	fn from(val: Renown) -> Self {
		Ability::Renown(val)
	}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub enum Gift {
	Moon(MoonGift),
	Shadow(ShadowGift),
	Wolf(WolfGift),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, Hash)]
pub enum MoonGift {
	Crescent,
	Full,
	Gibbous,
	Half,
	New,
	_Custom(String),
}

impl MoonGift {
	pub fn get_modifiers(&self, value: &u16) -> Vec<crate::character::Modifier> {
		match self {
			// MoonGift::Crescent => vec![],
			MoonGift::Full => {
				if value > &2 {
					vec![Modifier::new(
						ModifierTarget::Trait(Trait::Health),
						ModifierValue::Ability(Ability::Renown(Renown::Purity)),
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

	pub fn name(&self) -> &str {
		match self {
			MoonGift::Crescent => "crescent",
			MoonGift::Full => "full",
			MoonGift::Gibbous => "gibbous",
			MoonGift::Half => "half",
			MoonGift::New => "new",
			MoonGift::_Custom(name) => name,
		}
	}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub enum ShadowGift {
	Death,
	Dominance,
	Elemental,
	Evasion,
	Insight,
	Inspiration,
	Knowledge,
	Nature,
	Rage,
	Shaping,
	Stealth,
	Strength,
	Technology,
	Warding,
	Weather,
	_Custom(String),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub enum WolfGift {
	Change,
	Hunting,
	Pack,
	_Custom(String),
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Form {
	#[default]
	Hishu,
	Dalu,
	Gauru,
	Urshul,
	Urhan,
}

impl Form {
	pub fn all() -> [Form; 5] {
		[
			Self::Hishu,
			Self::Dalu,
			Self::Gauru,
			Self::Urshul,
			Self::Urhan,
		]
	}

	pub fn name(&self) -> &str {
		match self {
			Form::Hishu => "hishu",
			Form::Dalu => "dalu",
			Form::Gauru => "gauru",
			Form::Urshul => "urshul",
			Form::Urhan => "urhan",
		}
	}

	#[allow(clippy::too_many_lines)]
	pub fn get_modifiers(&self) -> Vec<Modifier> {
		match self {
			Form::Hishu => vec![Modifier::new(
				ModifierTarget::Trait(Trait::Preception),
				ModifierValue::Num(1),
				ModifierOp::Add,
			)],
			Form::Dalu => vec![
				Modifier::new(
					ModifierTarget::Attribute(Attribute::Strength),
					ModifierValue::Num(1),
					ModifierOp::Add,
				),
				Modifier::new(
					ModifierTarget::Attribute(Attribute::Stamina),
					ModifierValue::Num(1),
					ModifierOp::Add,
				),
				Modifier::new(
					ModifierTarget::Attribute(Attribute::Manipulation),
					ModifierValue::Num(-1),
					ModifierOp::Add,
				),
				Modifier::new(
					ModifierTarget::Trait(Trait::Size),
					ModifierValue::Num(1),
					ModifierOp::Add,
				),
				Modifier::new(
					ModifierTarget::Trait(Trait::Preception),
					ModifierValue::Num(2),
					ModifierOp::Add,
				),
			],
			Form::Gauru => vec![
				Modifier::new(
					ModifierTarget::Attribute(Attribute::Strength),
					ModifierValue::Num(3),
					ModifierOp::Add,
				),
				Modifier::new(
					ModifierTarget::Attribute(Attribute::Dexterity),
					ModifierValue::Num(1),
					ModifierOp::Add,
				),
				Modifier::new(
					ModifierTarget::Attribute(Attribute::Stamina),
					ModifierValue::Num(2),
					ModifierOp::Add,
				),
				Modifier::new(
					ModifierTarget::Trait(Trait::Size),
					ModifierValue::Num(2),
					ModifierOp::Add,
				),
				Modifier::new(
					ModifierTarget::Trait(Trait::Preception),
					ModifierValue::Num(3),
					ModifierOp::Add,
				),
			],
			Form::Urshul => vec![
				Modifier::new(
					ModifierTarget::Attribute(Attribute::Strength),
					ModifierValue::Num(2),
					ModifierOp::Add,
				),
				Modifier::new(
					ModifierTarget::Attribute(Attribute::Dexterity),
					ModifierValue::Num(2),
					ModifierOp::Add,
				),
				Modifier::new(
					ModifierTarget::Attribute(Attribute::Stamina),
					ModifierValue::Num(2),
					ModifierOp::Add,
				),
				Modifier::new(
					ModifierTarget::Attribute(Attribute::Manipulation),
					ModifierValue::Num(-1),
					ModifierOp::Add,
				),
				Modifier::new(
					ModifierTarget::Trait(Trait::Size),
					ModifierValue::Num(1),
					ModifierOp::Add,
				),
				Modifier::new(
					ModifierTarget::Trait(Trait::Speed),
					ModifierValue::Num(3),
					ModifierOp::Add,
				),
				Modifier::new(
					ModifierTarget::Trait(Trait::Preception),
					ModifierValue::Num(3),
					ModifierOp::Add,
				),
			],
			Form::Urhan => vec![
				Modifier::new(
					ModifierTarget::Attribute(Attribute::Dexterity),
					ModifierValue::Num(2),
					ModifierOp::Add,
				),
				Modifier::new(
					ModifierTarget::Attribute(Attribute::Stamina),
					ModifierValue::Num(1),
					ModifierOp::Add,
				),
				Modifier::new(
					ModifierTarget::Attribute(Attribute::Manipulation),
					ModifierValue::Num(-1),
					ModifierOp::Add,
				),
				Modifier::new(
					ModifierTarget::Trait(Trait::Size),
					ModifierValue::Num(-1),
					ModifierOp::Add,
				),
				Modifier::new(
					ModifierTarget::Trait(Trait::Speed),
					ModifierValue::Num(3),
					ModifierOp::Add,
				),
				Modifier::new(
					ModifierTarget::Trait(Trait::Preception),
					ModifierValue::Num(4),
					ModifierOp::Add,
				),
			],
		}
	}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, Hash)]
pub enum WerewolfMerit {
	FavoredForm(Option<Form>),
	EfficientKiller,
	Totem,
}

impl WerewolfMerit {
	pub fn all() -> Vec<WerewolfMerit> {
		vec![Self::FavoredForm(None), Self::EfficientKiller, Self::Totem]
	}
}

impl From<WerewolfMerit> for Merit {
	fn from(merit: WerewolfMerit) -> Self {
		Merit::Werewolf(merit)
	}
}
