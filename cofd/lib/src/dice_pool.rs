use serde::{Deserialize, Serialize};
use std::{
	cmp::{max, min},
	fmt::Display,
	ops::{Add, Sub},
};

use cofd_util::VariantName;

use crate::{
	character::traits::Trait,
	prelude::{Attribute, Character, Skill},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DicePool {
	Mod(i16),
	Attribute(Attribute),
	Skill(Skill),
	Trait(Trait),

	Min(Box<DicePool>, Box<DicePool>),
	Max(Box<DicePool>, Box<DicePool>),

	Add(Box<DicePool>, Box<DicePool>),
	Sub(Box<DicePool>, Box<DicePool>),
}

impl DicePool {
	pub fn value(&self, character: &Character) -> i16 {
		match self {
			Self::Mod(val) => *val,
			Self::Attribute(attr) => *character.attributes().get(attr) as i16,
			Self::Skill(skill) => character.skills().get(*skill) as i16,
			Self::Trait(trait_) => character.get_trait(trait_) as i16,
			Self::Add(p1, p2) => p1.value(character) + p2.value(character),
			Self::Sub(p1, p2) => p1.value(character) - p2.value(character),
			Self::Max(p1, p2) => max(p1.value(character), p2.value(character)),
			Self::Min(p1, p2) => min(p1.value(character), p2.value(character)),
		}
	}

	pub fn min(p1: impl Into<DicePool>, p2: impl Into<DicePool>) -> DicePool {
		DicePool::Min(Box::new(p1.into()), Box::new(p2.into()))
	}

	pub fn max(p1: impl Into<DicePool>, p2: impl Into<DicePool>) -> DicePool {
		DicePool::Max(Box::new(p1.into()), Box::new(p2.into()))
	}
}

impl Default for DicePool {
	fn default() -> Self {
		Self::Mod(0)
	}
}

impl Add for DicePool {
	type Output = DicePool;

	fn add(self, rhs: Self) -> Self::Output {
		DicePool::Add(Box::new(self), Box::new(rhs))
	}
}

impl Sub for DicePool {
	type Output = DicePool;

	fn sub(self, rhs: Self) -> Self::Output {
		DicePool::Sub(Box::new(self), Box::new(rhs))
	}
}

impl From<Attribute> for DicePool {
	fn from(attr: Attribute) -> Self {
		DicePool::Attribute(attr)
	}
}

impl From<Skill> for DicePool {
	fn from(skill: Skill) -> Self {
		DicePool::Skill(skill)
	}
}

impl From<Trait> for DicePool {
	fn from(trait_: Trait) -> Self {
		DicePool::Trait(trait_)
	}
}

impl From<i16> for DicePool {
	fn from(val: i16) -> Self {
		DicePool::Mod(val)
	}
}

impl Add for Attribute {
	type Output = DicePool;

	fn add(self, rhs: Self) -> Self::Output {
		DicePool::Attribute(self) + DicePool::Attribute(rhs)
	}
}

impl Add<Skill> for Attribute {
	type Output = DicePool;

	fn add(self, rhs: Skill) -> Self::Output {
		DicePool::Attribute(self) + DicePool::Skill(rhs)
	}
}

impl Add<Skill> for DicePool {
	type Output = DicePool;

	fn add(self, rhs: Skill) -> Self::Output {
		self + DicePool::Skill(rhs)
	}
}

impl Display for DicePool {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			DicePool::Mod(val) => val.fmt(f),
			DicePool::Attribute(attr) => f.write_str(attr.name()),
			DicePool::Skill(skill) => f.write_str(skill.name()),
			DicePool::Trait(trait_) => f.write_str(trait_.name().unwrap()),
			DicePool::Min(p1, p2) => f.write_fmt(format_args!("min({p1}, {p2})")),
			DicePool::Max(p1, p2) => f.write_fmt(format_args!("max({p1}, {p2})")),
			DicePool::Add(p1, p2) => f.write_fmt(format_args!("{p1} + {p2}")),
			DicePool::Sub(p1, p2) => f.write_fmt(format_args!("{p1} - {p2}")),
		}
	}
}
