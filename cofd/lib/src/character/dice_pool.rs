use std::ops::{Add, Sub};

use super::{Attribute, Character, Skill, Trait};

pub enum PoolComponent {
	Attribute(Attribute),
	Skill(Skill),
	Trait(Trait),

	Neg(Box<PoolComponent>),
	Pool(Box<DicePool>),
}

impl PoolComponent {
	pub fn value(&self, character: &Character) -> i16 {
		match self {
			Self::Attribute(attr) => *character.attributes().get(*attr) as i16,
			Self::Skill(skill) => *character.skills().get(*skill) as i16,
			Self::Trait(trait_) => character.get_trait(*trait_) as i16,
			Self::Neg(n) => -n.value(character),
			Self::Pool(pool) => pool.total(character),
		}
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

pub struct DicePool(PoolComponent, PoolComponent);

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

impl DicePool {
	pub fn total(&self, character: &Character) -> i16 {
		self.0.value(character) + self.1.value(character)
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

impl Add for Attribute {
	type Output = DicePool;

	fn add(self, rhs: Self) -> Self::Output {
		DicePool(
			PoolComponent::Attribute(self),
			PoolComponent::Attribute(rhs),
		)
	}
}

pub fn uwu() {
	let dice_pool: DicePool = Attribute::Resolve + Attribute::Composure;
}
