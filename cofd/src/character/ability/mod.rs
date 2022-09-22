use std::fmt::Debug;

use serde::Serialize;

use super::{Attribute, Modifier, ModifierTarget};

pub trait Ability: PartialEq + Debug + erased_serde::Serialize {
	fn value(&self) -> &u8;
	fn value_mut(&mut self) -> &mut u8;
	fn get_modifiers(&self) -> Vec<Modifier>;
}

// trait AbilityClone {
// 	fn clone_box(&self) -> Box<dyn Ability>;
// }

// impl<T> AbilityClone for T
// where
//     T: 'static + Ability + Clone,
// {
//     fn clone_box(&self) -> Box<dyn Ability> {
//         Box::new(self.clone())
//     }
// }

// impl Clone for Box<dyn Ability> {
// 	fn clone(&self) -> Box<dyn Ability> {
// 		self.clone_box()
// 	}
// }

// pub struct TraitModAbility {
// 	target: ModifierTarget,
// 	value: u8,
// }

// impl TraitModAbility {
// 	pub fn new(value: u8, target: ModifierTarget) -> Self {
// 		Self { value, target }
// 	}
// }

// impl Ability for TraitModAbility {
// 	fn value(&self) -> &u8 {
// 		&self.value
// 	}

// 	fn value_mut(&mut self) -> &mut u8 {
// 		&mut self.value
// 	}
// }

// impl HasModifiers for TraitModAbility {
// 	fn get_modifiers(&self) -> Vec<Modifier> {
// 		vec![Modifier {
// 			target: self.target,
// 			value: self.value,
// 		}]
// 	}
// }

// pub struct NoOpAbility {
// 	value: u8,
// }

// impl NoOpAbility {
// 	pub fn new(value: u8) -> Self {
// 		Self { value }
// 	}
// }

// impl Ability for NoOpAbility {
// 	fn value(&self) -> &u8 {
// 		&self.value
// 	}

// 	fn value_mut(&mut self) -> &mut u8 {
// 		&mut self.value
// 	}
// }

// impl HasModifiers for NoOpAbility {
// 	fn get_modifiers(&self) -> Vec<Modifier> {
// 		vec![]
// 	}
// }

// #[derive(PartialEq, Eq, Hash)]
// pub enum Abilities {
//     Vigor,
//     Resilience,
//     Celerity,
//     _Custom(String)
// }

// #[derive(PartialEq, Eq, Hash)]

// pub enum Merits {
//     _Custom(String)
// }
