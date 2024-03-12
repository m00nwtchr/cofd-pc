use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};

use super::{traits::*, Character};
use crate::{
	dice_pool::DicePool,
	splat::{ability::Ability, werewolf::Form, Splat},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Modifier {
	pub target: ModifierTarget,
	pub value: ModifierValue,
	pub op: ModifierOp,
	pub condition: Option<Condition>,
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
			condition: None,
		}
	}

	pub fn conditional(
		target: impl Into<ModifierTarget>,
		value: impl Into<ModifierValue>,
		op: ModifierOp,
		condition: impl Into<Condition>,
	) -> Self {
		Self {
			target: target.into(),
			value: value.into(),
			op,
			condition: Some(condition.into()),
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
			ModifierValue::Skill(skill) => character.skills.get(*skill) as i16,
			ModifierValue::DicePool(pool) => pool.value(character),
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

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub enum Condition {
	WerewolfForm(Form),
}

impl Condition {
	pub fn check(&self, character: &Character) -> bool {
		match self {
			Self::WerewolfForm(form) => {
				if let Splat::Werewolf(.., data) = &character.splat {
					form.eq(&data.form)
				} else {
					false
				}
			}
		}
	}
}

impl From<Form> for Condition {
	fn from(value: Form) -> Self {
		Condition::WerewolfForm(value)
	}
}

type ModMap = HashMap<ModifierTarget, Vec<ModifierValue>>;
type CondModMap = HashMap<ModifierTarget, HashMap<Condition, Vec<ModifierValue>>>;

#[derive(Debug, Default, Clone)]
pub struct Modifiers {
	modifier_map: Arc<RwLock<ModMap>>,
	conditional_modifier_map: Arc<RwLock<CondModMap>>,
}

fn push_or_init<K, V>(map: &mut HashMap<K, Vec<V>>, key: K, value: V)
where
	K: Eq + std::hash::Hash,
{
	let vec = map.get_mut(&key);

	if let Some(vec) = vec {
		vec.push(value);
	} else {
		map.insert(key, vec![value]);
	}
}

fn handle_modifier(
	modifier: Modifier,
	modifier_map: &mut ModMap,
	conditional_modifier_map: &mut CondModMap,
) {
	if let Some(condition) = modifier.condition {
		let target_map = conditional_modifier_map.get_mut(&modifier.target);

		if let Some(target_map) = target_map {
			push_or_init(target_map, condition, modifier.value);
		} else {
			conditional_modifier_map.insert(
				modifier.target,
				HashMap::from([(condition, vec![modifier.value])]),
			);
		}
	} else {
		push_or_init(modifier_map, modifier.target, modifier.value);
	}
}

impl Modifiers {
	#[allow(clippy::too_many_lines)]
	pub fn update(&self, character: &Character) {
		let mut conditional_modifier_map = self.conditional_modifier_map.write().unwrap();
		let mut modifier_map = self.modifier_map.write().unwrap();

		modifier_map.clear();
		conditional_modifier_map.clear();

		let mut modifiers: Vec<Modifier> = Vec::new();

		modifiers.extend(
			character
				.abilities
				.iter()
				.flat_map(|(ability, val)| ability.get_modifiers(*val)),
		);
		modifiers.extend(
			character
				.merits
				.iter()
				.flat_map(|(merit, val)| merit.get_modifiers(*val)),
		);

		match &character.splat {
			Splat::Werewolf(auspice, .., data) => {
				// modifiers.extend(data.form.get_modifiers());
				modifiers.extend(Form::modifiers());

				if let Some(auspice) = auspice {
					modifiers.extend(
						auspice.get_moon_gift().get_modifiers(
							*character
								.get_ability_value(&auspice.get_renown().clone().into())
								.unwrap_or(&0),
						),
					);

					if let Some(skill_bonus) = data.skill_bonus {
						if auspice.get_skills().contains(&skill_bonus) {
							modifiers.push(Modifier::new(
								ModifierTarget::BaseSkill(skill_bonus),
								1,
								ModifierOp::Add,
							));
						}
					}
				}
			}
			Splat::Mage(_, order, _, data) => {
				// TODO: High Speech merit, Order Status merit
				if order.is_some() {
					modifiers.push(Modifier::new(
						ModifierTarget::BaseSkill(Skill::Occult),
						1,
						ModifierOp::Add,
					));
				}

				if let Some(attr_bonus) = data.attr_bonus {
					if attr_bonus.get_type() == AttributeType::Resistance {
						modifiers.push(Modifier::new(
							ModifierTarget::BaseAttribute(attr_bonus),
							1,
							ModifierOp::Add,
						));
					}
				}
			}
			Splat::Vampire(clan, .., data) => {
				if let Some(attr_bonus) = data.attr_bonus {
					if clan.get_favored_attributes().contains(&attr_bonus) {
						modifiers.push(Modifier::new(
							ModifierTarget::BaseAttribute(attr_bonus),
							1,
							ModifierOp::Add,
						));
					}
				}
			}
			Splat::Changeling(seeming, .., data) => {
				if let Some(attr_bonus) = data.attr_bonus {
					if seeming.get_favored_attributes().contains(&attr_bonus) {
						modifiers.push(Modifier::new(
							ModifierTarget::BaseAttribute(attr_bonus),
							1,
							ModifierOp::Add,
						));
					}
				}
			}

			_ => {}
		}

		// let mut defense_skill = Skill::Athletics;
		// let mut defense_attr = DicePool::min(Attribute::Wits, Attribute::Dexterity);

		for modifier in modifiers {
			handle_modifier(modifier, &mut modifier_map, &mut conditional_modifier_map);
		}
		// character._defense_pool = defense_attr + defense_skill;
	}

	pub fn get_modifier(&self, character: &Character, target: impl Into<ModifierTarget>) -> i16 {
		let target = &target.into();
		let mut count = 0;

		let modifier_map = self.modifier_map.read().unwrap();
		let conditional_modifier_map = self.conditional_modifier_map.read().unwrap();

		if let Some(vec) = modifier_map.get(target) {
			for value in vec {
				count += value.value(character);
			}
		}
		if let Some(vec) = conditional_modifier_map.get(target) {
			for (cond, vec) in vec {
				if cond.check(character) {
					for value in vec {
						count += value.value(character);
					}
				}
			}
		}

		count
	}

	pub fn get_conditional_modifier(
		&self,
		character: &Character,
		target: impl Into<ModifierTarget>,
		condition: impl Into<Condition>,
	) -> Option<i16> {
		let target = &target.into();
		let condition = &condition.into();

		let conditional_modifier_map = self.conditional_modifier_map.read().unwrap();
		conditional_modifier_map
			.get(target)
			.and_then(|map2| map2.get(condition))
			.map(|vec| vec.iter().fold(0, |acc, e| acc + e.value(character)))
	}

	pub fn get_pool(
		&self,
		character: &Character,
		target: impl Into<ModifierTarget>,
	) -> Option<DicePool> {
		let modifier_map = self.modifier_map.read().unwrap();
		let conditional_modifier_map = self.conditional_modifier_map.read().unwrap();

		let target = target.into();

		match target {
			ModifierTarget::Trait(Trait::Defense) => {
				let mut defense_attribute = DicePool::min(Attribute::Wits, Attribute::Dexterity);
				let mut defense_skill = Skill::Athletics;

				if let Some(vec) = modifier_map.get(&target) {
					for value in vec {
						if let ModifierValue::Skill(skill) = value {
							defense_skill = *skill;
						} else if let ModifierValue::DicePool(pool) = value {
							defense_attribute = pool.clone();
						}
					}
				}
				if let Some(vec) = conditional_modifier_map.get(&target) {
					for (cond, vec) in vec {
						if cond.check(character) {
							for value in vec {
								if let ModifierValue::Skill(skill) = value {
									defense_skill = *skill;
								} else if let ModifierValue::DicePool(pool) = value {
									defense_attribute = pool.clone();
								}
							}
						}
					}
				}

				Some(defense_attribute + defense_skill)
			}
			_ => None,
		}
	}

	pub fn get_conditional_pool(
		&self,
		target: impl Into<ModifierTarget>,
		condition: impl Into<Condition>,
	) -> Option<DicePool> {
		let modifier_map = self.modifier_map.read().unwrap();
		let conditional_modifier_map = self.conditional_modifier_map.read().unwrap();

		let target = target.into();
		let condition = condition.into();

		match target {
			ModifierTarget::Trait(Trait::Defense) => {
				let mut defense_attribute = DicePool::min(Attribute::Wits, Attribute::Dexterity);
				let mut defense_skill = Skill::Athletics;

				if let Some(vec) = modifier_map.get(&target) {
					for value in vec {
						if let ModifierValue::Skill(skill) = value {
							defense_skill = *skill;
						} else if let ModifierValue::DicePool(pool) = value {
							defense_attribute = pool.clone();
						}
					}
				}
				if let Some(vec) = conditional_modifier_map.get(&target) {
					for (cond, vec) in vec {
						if condition.eq(cond) {
							for value in vec {
								if let ModifierValue::Skill(skill) = value {
									defense_skill = *skill;
								} else if let ModifierValue::DicePool(pool) = value {
									defense_attribute = pool.clone();
								}
							}
						}
					}
				}

				Some(defense_attribute + defense_skill)
			}
			_ => None,
		}
	}
}
