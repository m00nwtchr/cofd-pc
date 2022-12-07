use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

use crate::{
	character::Trait,
	prelude::{Attribute, Character, Skill},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoolComponent {
	Mod(u16),
	Attribute(Attribute),
	Skill(Skill),
	Trait(Trait),

	Neg(Box<PoolComponent>),
	Pool(Box<DicePool>),
}

impl PoolComponent {
	pub fn value(&self, character: &Character) -> i16 {
		match self {
			Self::Mod(val) => *val as i16,
			Self::Attribute(attr) => *character.attributes().get(*attr) as i16,
			Self::Skill(skill) => *character.skills().get(*skill) as i16,
			Self::Trait(trait_) => character.get_trait(*trait_) as i16,
			Self::Neg(n) => -n.value(character),
			Self::Pool(pool) => pool.total(character),
		}
	}
}

impl Default for PoolComponent {
	fn default() -> Self {
		Self::Mod(0)
	}
}

impl Add for PoolComponent {
	type Output = DicePool;

	fn add(self, rhs: Self) -> Self::Output {
		DicePool(self, rhs)
	}
}

impl Sub for PoolComponent {
	type Output = DicePool;

	fn sub(self, rhs: Self) -> Self::Output {
		DicePool(self, PoolComponent::Neg(Box::new(rhs)))
	}
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DicePool(PoolComponent, PoolComponent);

impl DicePool {
	pub fn total(&self, character: &Character) -> i16 {
		self.0.value(character) + self.1.value(character)
	}
}

impl Add for DicePool {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		DicePool(
			PoolComponent::Pool(Box::new(self)),
			PoolComponent::Pool(Box::new(rhs)),
		)
	}
}

impl Sub for DicePool {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		DicePool(
			PoolComponent::Pool(Box::new(self)),
			PoolComponent::Neg(Box::new(PoolComponent::Pool(Box::new(rhs)))),
		)
	}
}

impl From<Attribute> for PoolComponent {
	fn from(attr: Attribute) -> Self {
		PoolComponent::Attribute(attr)
	}
}

impl From<Skill> for PoolComponent {
	fn from(skill: Skill) -> Self {
		PoolComponent::Skill(skill)
	}
}

impl From<Trait> for PoolComponent {
	fn from(trait_: Trait) -> Self {
		PoolComponent::Trait(trait_)
	}
}

impl From<DicePool> for PoolComponent {
	fn from(pool: DicePool) -> Self {
		PoolComponent::Pool(Box::new(pool))
	}
}

impl From<u16> for PoolComponent {
	fn from(val: u16) -> Self {
		PoolComponent::Mod(val)
	}
}

impl Add for Attribute {
	type Output = DicePool;

	fn add(self, rhs: Self) -> Self::Output {
		PoolComponent::Attribute(self) + PoolComponent::Attribute(rhs)
	}
}

impl Add<Skill> for Attribute {
	type Output = DicePool;

	fn add(self, rhs: Skill) -> Self::Output {
		PoolComponent::Attribute(self) + PoolComponent::Skill(rhs)
	}
}
