use serde::{Deserialize, Serialize};

use super::{traits::*, Character};
use crate::{
	dice_pool::DicePool,
	splat::{ability::Ability, werewolf::Form},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Modifier {
	pub target: ModifierTarget,
	pub value: ModifierValue,
	pub op: ModifierOp,
}

impl Modifier {
	pub fn new(
		target: impl Into<ModifierTarget>,
		value: impl Into<ModifierValue>,
		op: ModifierOp,
	) -> Self {
		Self {
			target: target.into(),
			value: value.into(),
			op,
		}
	}

	pub fn val(&self) -> Option<i16> {
		match self.value {
			ModifierValue::Num(val) => Some(val),
			ModifierValue::Skill(_) | ModifierValue::Ability(_) => None,
			ModifierValue::DicePool(_) => unreachable!(),
		}
	}
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
pub enum ModifierTarget {
	BaseAttribute(Attribute),
	BaseSkill(Skill),
	Attribute(Attribute),
	Skill(Skill),
	Trait(Trait),

	WerewolfForm(Form, Box<ModifierTarget>),
}

impl From<Attribute> for ModifierTarget {
	fn from(attr: Attribute) -> Self {
		ModifierTarget::Attribute(attr)
	}
}

impl From<Skill> for ModifierTarget {
	fn from(value: Skill) -> Self {
		ModifierTarget::Skill(value)
	}
}

impl From<Trait> for ModifierTarget {
	fn from(value: Trait) -> Self {
		ModifierTarget::Trait(value)
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModifierValue {
	Num(i16),
	Ability(Ability),
	Skill(Skill),
	DicePool(DicePool),
}

impl ModifierValue {
	pub fn value(&self, character: &Character) -> i16 {
		match self {
			ModifierValue::Num(value) => *value,
			ModifierValue::Ability(ability) => {
				*character.get_ability_value(ability).unwrap_or(&0) as i16
			}
			ModifierValue::Skill(skill) => *character.skills.get(skill) as i16,
			ModifierValue::DicePool(_) => unreachable!(),
		}
	}
}

impl From<i16> for ModifierValue {
	fn from(value: i16) -> Self {
		ModifierValue::Num(value)
	}
}

impl From<u16> for ModifierValue {
	fn from(value: u16) -> Self {
		ModifierValue::Num(value as i16)
	}
}

impl From<i32> for ModifierValue {
	fn from(value: i32) -> Self {
		ModifierValue::Num(value as i16)
	}
}

impl From<Ability> for ModifierValue {
	fn from(value: Ability) -> Self {
		ModifierValue::Ability(value)
	}
}

impl From<Skill> for ModifierValue {
	fn from(value: Skill) -> Self {
		ModifierValue::Skill(value)
	}
}

impl From<DicePool> for ModifierValue {
	fn from(value: DicePool) -> Self {
		ModifierValue::DicePool(value)
	}
}

#[derive(Clone, Debug, PartialEq, Eq, VariantName)]
pub enum ModifierOp {
	Add,
	Set,
}
