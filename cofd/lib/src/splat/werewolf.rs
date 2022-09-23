use serde::{Deserialize, Serialize};

use crate::character::{
	ability::Ability, Attribute, Modifier, ModifierOp, ModifierTarget, ModifierValue, Skill, Trait,
};

use super::AbilityKey;

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct WerewolfData {
	form: Form,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Auspice {
	Cahalith,
	Elodoth,
	Irraka,
	Ithaeur,
	Rahu,
	_Custom(String, [Skill; 3], Renown, MoonGift, [ShadowGift; 2]),
}

impl Auspice {
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

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
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
			Tribe::_Custom(_, _, gifts) => &gifts,
		}
	}
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Renown {
	Cunning,
	Glory,
	Honor,
	Purity,
	Wisdom,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct RenownAbility(pub u8);

impl Ability for RenownAbility {
	fn value(&self) -> &u8 {
		&self.0
	}

	fn value_mut(&mut self) -> &mut u8 {
		&mut self.0
	}

	fn get_modifiers(&self) -> Vec<crate::character::Modifier> {
		vec![]
	}
}

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum Gift {
	Moon(MoonGift),
	Shadow(ShadowGift),
	Wolf(WolfGift),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct MoonGiftAbility(u8, MoonGift);

impl Ability for MoonGiftAbility {
	fn value(&self) -> &u8 {
		&self.0
	}

	fn value_mut(&mut self) -> &mut u8 {
		&mut self.0
	}

	fn get_modifiers(&self) -> Vec<crate::character::Modifier> {
		match &self.1 {
			MoonGift::Crescent => vec![],
			MoonGift::Full => {
				if self.0 > 2 {
					vec![Modifier::new(
						ModifierTarget::Trait(Trait::Health),
						ModifierValue::Ability(AbilityKey::Renown(Renown::Purity)),
						ModifierOp::Add,
					)]
				} else {
					vec![]
				}
			}
			MoonGift::Gibbous => vec![],
			MoonGift::Half => vec![],
			MoonGift::New => vec![],
			MoonGift::_Custom(_) => todo!(),
		}
	}
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum MoonGift {
	Crescent,
	Full,
	Gibbous,
	Half,
	New,
	_Custom(String),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
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

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum WolfGift {
	Change,
	Hunting,
	Pack,
	_Custom(String),
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub enum Form {
	#[default]
	Hishu,
	Dalu,
	Gauru,
	Urshul,
	Urhan,
}

impl Ability for Form {
	fn value(&self) -> &u8 {
		&0
	}

	fn value_mut(&mut self) -> &mut u8 {
		panic!()
	}

	fn get_modifiers(&self) -> Vec<Modifier> {
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
