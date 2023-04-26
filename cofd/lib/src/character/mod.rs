use serde::{Deserialize, Serialize};
use std::{
	cmp::{max, min},
	collections::HashMap,
	ops::{Add, Index, Sub},
};

use crate::splat::{ability::Ability, Merit, Splat};
use crate::{dice_pool::DicePool, prelude::VariantName};

pub mod modifier;
pub mod traits;

use modifier::*;
use traits::*;

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn add(a: u16, b: i16) -> u16 {
	let res = i32::from(a) + i32::from(b);

	if res >= 255 {
		u16::MAX
	} else if res <= 0 {
		u16::MIN
	} else {
		res as u16
	}
}

#[derive(Default)]
pub struct CharacterBuilder {
	splat: Splat,
	info: CharacterInfo,
	attributes: Attributes,
	skills: Skills,
	specialties: HashMap<Skill, Vec<String>>,
	merits: Vec<(Merit, u16)>,

	abilities: HashMap<Ability, u16>,
	power: u16,
	fuel: Option<u16>,

	flag: bool,
}

impl CharacterBuilder {
	#[must_use]
	pub fn with_splat(mut self, splat: Splat) -> Self {
		self.splat = splat;
		self
	}

	#[must_use]
	pub fn with_info(mut self, info: CharacterInfo) -> Self {
		self.info = info;
		self
	}

	#[must_use]
	pub fn with_attributes(mut self, attributes: Attributes) -> Self {
		self.attributes = attributes;
		self
	}

	#[must_use]
	pub fn with_skills(mut self, skills: Skills) -> Self {
		self.skills = skills;
		self
	}

	#[must_use]
	pub fn with_specialties(mut self, skill: Skill, specialties: Vec<String>) -> Self {
		self.specialties.insert(skill, specialties);
		self
	}

	#[must_use]
	pub fn with_abilities<const N: usize>(mut self, abilities: [(Ability, u16); N]) -> Self {
		self.abilities = HashMap::from(abilities);

		self.flag = true;
		self
	}

	#[must_use]
	pub fn with_merits<const N: usize>(mut self, merits: [(Merit, u16); N]) -> Self {
		self.merits = Vec::from(merits);
		self.flag = true;
		self
	}

	#[must_use]
	pub fn with_st(mut self, st: u16) -> Self {
		self.power = st;
		self
	}

	#[must_use]
	pub fn with_fuel(mut self, fuel: u16) -> Self {
		self.fuel = Some(fuel);
		self
	}

	#[must_use]
	pub fn build(self) -> Character {
		let power = if let Splat::Mortal = &self.splat {
			0
		} else if self.power > 0 {
			self.power
		} else {
			1
		};

		let mut character = Character {
			splat: self.splat,
			info: self.info,
			power,
			_attributes: self.attributes,
			skills: self.skills,
			abilities: self.abilities,
			merits: self.merits,
			specialties: self.specialties,
			..Default::default()
		};

		if self.flag {
			character.calc_mod_map();
		}

		character.fuel = self.fuel.unwrap_or_else(|| character.max_fuel());
		character.willpower = character.max_willpower();

		if let Splat::Werewolf(Some(auspice), .., data) = &mut character.splat {
			data.hunters_aspect = Some(auspice.get_hunters_aspect().clone());
		}

		character
	}
}

#[derive(Default, Debug, Clone)]
pub enum Wound {
	#[default]
	None,
	Bashing,
	Lethal,
	Aggravated,
}

impl Wound {
	#[must_use]
	pub fn inc(&self) -> Wound {
		match self {
			Wound::None => Wound::Bashing,
			Wound::Bashing => Wound::Lethal,
			Wound::Lethal => Wound::Aggravated,
			Wound::Aggravated => Wound::Aggravated,
		}
	}

	#[must_use]
	pub fn poke(&self) -> Wound {
		if let Wound::Aggravated = self {
			Wound::None
		} else {
			self.inc()
		}
	}

	pub fn poke_mut(&mut self) {
		*self = self.poke();
	}
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Damage {
	#[serde(skip_serializing_if = "is_zero")]
	aggravated: u16,
	#[serde(skip_serializing_if = "is_zero")]
	lethal: u16,
	#[serde(skip_serializing_if = "is_zero")]
	bashing: u16,
}

impl Damage {
	pub fn new(bashing: u16, lethal: u16, aggravated: u16) -> Self {
		Self {
			aggravated,
			lethal,
			bashing,
		}
	}

	pub fn get(&self, wound: &Wound) -> u16 {
		match wound {
			Wound::None => 0,
			Wound::Bashing => self.bashing,
			Wound::Lethal => self.lethal,
			Wound::Aggravated => self.aggravated,
		}
	}

	pub fn get_i(&self, i: usize) -> Wound {
		// println!("{i}");
		if i < self.aggravated as usize {
			Wound::Aggravated
		} else if i >= self.aggravated as usize && i < (self.aggravated + self.lethal) as usize {
			Wound::Lethal
		} else if i >= (self.aggravated + self.lethal) as usize
			&& i < (self.aggravated + self.lethal + self.bashing) as usize
		{
			Wound::Bashing
		} else {
			Wound::None
		}
	}

	pub fn sum(&self) -> u16 {
		self.bashing + self.lethal + self.aggravated
	}

	pub fn dec(&mut self, wound: &Wound) {
		match wound {
			Wound::None => {}
			Wound::Bashing => {
				if self.bashing > 0 {
					self.bashing -= 1;
				}
			}
			Wound::Lethal => {
				if self.lethal > 0 {
					self.lethal -= 1;
				}
			}
			Wound::Aggravated => {
				if self.aggravated > 0 {
					self.aggravated -= 1;
				}
			}
		}
	}

	pub fn inc(&mut self, wound: &Wound) {
		match wound {
			Wound::None => {}
			Wound::Bashing => self.bashing += 1,
			Wound::Lethal => self.lethal += 1,
			Wound::Aggravated => self.aggravated += 1,
		}
	}

	pub fn poke(&mut self, wound: &Wound) {
		match wound {
			Wound::None => self.bashing += 1,
			Wound::Bashing => {
				if self.bashing > 0 {
					self.bashing -= 1;
				}
				self.lethal += 1;
			}
			Wound::Lethal => {
				if self.lethal > 0 {
					self.lethal -= 1;
				}
				self.aggravated += 1;
			}
			Wound::Aggravated => {
				if self.aggravated > 0 {
					self.aggravated -= 1;
				}
			}
		}
	}
}

pub fn defense_pool() -> DicePool {
	DicePool::min(Attribute::Wits, Attribute::Dexterity) + Skill::Athletics
}

pub fn is_empty_vec(vec: &Vec<String>) -> bool {
	vec.is_empty()
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_five(n: &u16) -> bool {
	*n == 5
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Character {
	pub splat: Splat,

	pub info: CharacterInfo,

	#[serde(rename = "attributes")]
	_attributes: Attributes,
	skills: Skills,
	pub specialties: HashMap<Skill, Vec<String>>,

	health: Damage,

	pub willpower: u16,
	pub power: u16,
	pub fuel: u16,
	pub integrity: u16,

	#[serde(skip_serializing_if = "is_empty_vec")]
	pub touchstones: Vec<String>,

	// #[serde(skip)]
	pub abilities: HashMap<Ability, u16>,
	// #[serde(skip)]
	pub merits: Vec<(Merit, u16)>,

	pub weapons: Vec<Weapon>,

	#[serde(skip_serializing_if = "is_five")]
	pub base_size: u16,
	base_armor: ArmorStruct,
	pub beats: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub alternate_beats: u16,

	pub conditions: Vec<String>,
	pub aspirations: Vec<String>,

	#[serde(skip)]
	modifiers: Modifiers,
}

impl Character {
	pub fn builder() -> CharacterBuilder {
		CharacterBuilder::default()
	}

	pub fn add_ability(&mut self, ability: Ability, val: u16) {
		self.abilities.insert(ability, val);
	}

	pub fn has_ability(&self, key: &Ability) -> bool {
		self.abilities.contains_key(key)
	}

	pub fn remove_ability(&mut self, key: &Ability) -> Option<u16> {
		self.abilities.remove(key)
	}

	pub fn get_ability_value(&self, key: &Ability) -> Option<&u16> {
		self.abilities.get(key)
	}

	pub fn get_ability_value_mut(&mut self, key: &Ability) -> Option<&mut u16> {
		self.abilities.get_mut(key)
	}

	pub fn add_merit(&mut self, key: Merit) {
		self.merits.push((key, 0));
	}

	pub fn remove_merit(&mut self, i: usize) -> (Merit, u16) {
		self.merits.remove(i)
	}

	pub fn get_merit(&self, i: usize) -> Option<&(Merit, u16)> {
		self.merits.get(i)
	}

	pub fn get_merit_mut(&mut self, i: usize) -> Option<&mut (Merit, u16)> {
		self.merits.get_mut(i)
	}

	pub fn get_trait(&self, trait_: &Trait) -> u16 {
		match trait_ {
			Trait::Speed => self.speed(),
			Trait::Defense => self.defense(),
			Trait::Initative => self.initative(),
			Trait::Perception => self.perception(),
			Trait::Health => self.max_health(),
			Trait::Size => self.size(),
			Trait::Beats => self.beats,
			Trait::Armor(Some(armor)) => match armor {
				Armor::General => self.armor().general,
				Armor::Ballistic => self.armor().ballistic,
			},
			Trait::Willpower => self.max_willpower(),
			Trait::Power => self.power,
			Trait::Fuel => self.fuel,
			Trait::Integrity => self.integrity,
			_ => 0,
		}
	}

	#[allow(clippy::too_many_lines)]
	pub fn calc_mod_map(&self) {
		self.modifiers.update(self);
	}

	pub fn get_modifier(&self, target: impl Into<ModifierTarget>) -> i16 {
		self.modifiers.get_modifier(self, target)
	}

	pub fn get_conditional_modifier(
		&self,
		target: impl Into<ModifierTarget>,
		condition: impl Into<Condition>,
	) -> Option<i16> {
		self.modifiers
			.get_conditional_modifier(self, target, condition)
	}

	pub fn get_pool(&self, target: impl Into<ModifierTarget>) -> Option<DicePool> {
		self.modifiers.get_pool(self, target)
	}

	pub fn get_conditional_pool(
		&self,
		target: impl Into<ModifierTarget>,
		condition: impl Into<Condition>,
	) -> Option<DicePool> {
		self.modifiers.get_conditional_pool(target, condition)
	}

	pub fn attributes(&self) -> Attributes {
		Attributes {
			intelligence: self._modified(Attribute::Intelligence),
			wits: self._modified(Attribute::Wits),
			resolve: self._modified(Attribute::Resolve),
			strength: self._modified(Attribute::Strength),
			dexterity: self._modified(Attribute::Dexterity),
			stamina: self._modified(Attribute::Stamina),
			presence: self._modified(Attribute::Presence),
			manipulation: self._modified(Attribute::Manipulation),
			composure: self._modified(Attribute::Composure),
		}
	}
	pub fn base_attributes_mut(&mut self) -> &mut Attributes {
		&mut self._attributes
	}
	pub fn base_attributes(&self) -> &Attributes {
		&self._attributes
	}

	pub fn _modified(&self, target: impl Into<ModifierTarget>) -> u16 {
		let target = &target.into();
		let base = match &target {
			ModifierTarget::BaseAttribute(attr) | ModifierTarget::Attribute(attr) => {
				*self._attributes.get(attr)
			}
			ModifierTarget::BaseSkill(skill) | ModifierTarget::Skill(skill) => {
				self.skills.get(*skill)
			}
			_ => 0,
		};

		let modifier = match &target {
			ModifierTarget::BaseAttribute(_) => 0,
			ModifierTarget::BaseSkill(_) => 0,
			_ => self.modifiers.get_modifier(self, target.clone()),
		};
		let base_modifier = self.modifiers.get_modifier(
			self,
			match *target {
				ModifierTarget::Attribute(attr) => ModifierTarget::BaseAttribute(attr),
				ModifierTarget::Skill(skill) => ModifierTarget::BaseSkill(skill),
				_ => target.clone(),
			},
		);

		if add(base, base_modifier) > 5 {
			add(base, modifier)
		} else {
			add(base, base_modifier + modifier)
		}
	}

	pub fn skills(&self) -> Skills {
		Skills {
			academics: self._modified(Skill::Academics),
			computer: self._modified(Skill::Computer),
			crafts: self._modified(Skill::Crafts),
			investigation: self._modified(Skill::Investigation),
			medicine: self._modified(Skill::Medicine),
			occult: self._modified(Skill::Occult),
			politics: self._modified(Skill::Politics),
			science: self._modified(Skill::Science),

			athletics: self._modified(Skill::Athletics),
			brawl: self._modified(Skill::Brawl),
			drive: self._modified(Skill::Drive),
			firearms: self._modified(Skill::Firearms),
			larceny: self._modified(Skill::Larceny),
			stealth: self._modified(Skill::Stealth),
			survival: self._modified(Skill::Survival),
			weaponry: self._modified(Skill::Weaponry),

			animal_ken: self._modified(Skill::AnimalKen),
			empathy: self._modified(Skill::Empathy),
			expression: self._modified(Skill::Expression),
			intimidation: self._modified(Skill::Intimidation),
			persuasion: self._modified(Skill::Persuasion),
			socialize: self._modified(Skill::Socialize),
			streetwise: self._modified(Skill::Streetwise),
			subterfuge: self._modified(Skill::Subterfuge),
		}
	}
	pub fn base_skills(&self) -> &Skills {
		&self.skills
	}
	pub fn base_skills_mut(&mut self) -> &mut Skills {
		&mut self.skills
	}

	pub fn max_health(&self) -> u16 {
		let attributes = self.attributes();

		add(
			self.size() + attributes.stamina,
			self.modifiers.get_modifier(self, Trait::Health),
		)
	}

	pub fn health(&self) -> &Damage {
		&self.health
	}

	pub fn health_mut(&mut self) -> &mut Damage {
		&mut self.health
	}

	pub fn wound_penalty(&self) -> u16 {
		let mh = self.max_health();
		match mh - min(self.health.sum(), mh) {
			2 => 1,
			1 => 2,
			0 => 3,
			_ => 0,
		}
	}

	pub fn max_willpower(&self) -> u16 {
		let attributes = self.attributes();

		attributes.resolve + attributes.composure
	}

	pub fn size(&self) -> u16 {
		add(
			self.base_size,
			self.modifiers.get_modifier(self, Trait::Size),
		)
	}
	pub fn speed(&self) -> u16 {
		let attributes = self.attributes();

		add(
			5 + attributes.dexterity + attributes.strength,
			self.modifiers.get_modifier(self, Trait::Speed),
		)
	}
	pub fn defense(&self) -> u16 {
		max::<i16>(0, self.get_pool(Trait::Defense).unwrap().value(self)) as u16
	}
	pub fn armor(&self) -> ArmorStruct {
		ArmorStruct {
			general: self.base_armor.general,
			ballistic: self.base_armor.ballistic,
		}
	}
	pub fn base_armor_mut(&mut self) -> &mut ArmorStruct {
		&mut self.base_armor
	}
	pub fn initative(&self) -> u16 {
		let attributes = self.attributes();

		add(
			attributes.dexterity + attributes.composure,
			self.modifiers.get_modifier(self, Trait::Initative),
		)
	}
	pub fn perception(&self) -> u16 {
		let attributes = self.attributes();

		add(
			attributes.wits + attributes.composure,
			self.modifiers.get_modifier(self, Trait::Perception),
		)
	}
	pub fn experience(&self) -> u16 {
		self.beats / 5
	}
	pub fn alternate_experience(&self) -> u16 {
		self.alternate_beats / 5
	}

	pub fn max_fuel(&self) -> u16 {
		match self.power {
			0 => self.attributes().stamina,
			1..=4 => 10 + self.power - 1,
			5..=8 => 10 + (self.power - 4) * 5,
			9 => 50,
			10 => 75,
			_ => 0,
		}
	}
}

impl Default for Character {
	fn default() -> Self {
		Self {
			splat: Default::default(),
			info: Default::default(),
			_attributes: Default::default(),
			skills: Default::default(),
			base_size: 5,
			abilities: Default::default(),
			merits: Default::default(),
			health: Default::default(),

			modifiers: Default::default(),

			power: Default::default(),
			integrity: 7,
			fuel: Default::default(),
			willpower: Default::default(),
			beats: Default::default(),
			alternate_beats: Default::default(),
			base_armor: Default::default(),
			specialties: Default::default(),
			touchstones: Default::default(),
			conditions: Default::default(),
			aspirations: Default::default(),
			weapons: Default::default(),
		}
	}
}

fn is_empty(str: &String) -> bool {
	str.is_empty()
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct CharacterInfo {
	#[serde(skip_serializing_if = "is_empty")]
	pub name: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub player: String,

	#[serde(skip_serializing_if = "is_empty")]
	pub virtue_anchor: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub vice_anchor: String,

	#[serde(skip_serializing_if = "is_empty")]
	pub faction: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub group_name: String,

	#[serde(skip_serializing_if = "is_empty")]
	pub concept: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub chronicle: String,

	#[serde(skip_serializing_if = "is_empty")]
	pub age: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub date_of_birth: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub hair: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub eyes: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub race: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub nationality: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub height: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub weight: String,
	#[serde(skip_serializing_if = "is_empty")]
	pub sex: String,

	#[serde(skip_serializing_if = "is_empty")]
	pub other: String,
}

#[derive(Debug, Clone, Copy, VariantName)]
pub enum InfoTrait {
	Name,
	Age,
	Player,
	VirtueAnchor,
	ViceAnchor,
	Concept,
	Chronicle,

	Faction,
	GroupName,

	DateOfBirth,
	Hair,
	Eyes,
	Race,
	Nationality,
	Height,
	Weight,
	Sex,
}

impl CharacterInfo {
	pub fn get(&self, info: InfoTrait) -> &String {
		match info {
			InfoTrait::Name => &self.name,
			InfoTrait::Age => &self.age,
			InfoTrait::Player => &self.player,
			InfoTrait::Concept => &self.concept,
			InfoTrait::Chronicle => &self.chronicle,
			InfoTrait::DateOfBirth => &self.date_of_birth,
			InfoTrait::Hair => &self.hair,
			InfoTrait::Eyes => &self.eyes,
			InfoTrait::Race => &self.race,
			InfoTrait::Nationality => &self.nationality,
			InfoTrait::Height => &self.height,
			InfoTrait::Weight => &self.weight,
			InfoTrait::Sex => &self.sex,
			InfoTrait::VirtueAnchor => &self.virtue_anchor,
			InfoTrait::ViceAnchor => &self.vice_anchor,
			InfoTrait::Faction => &self.faction,
			InfoTrait::GroupName => &self.group_name,
		}
	}

	pub fn get_mut(&mut self, info: InfoTrait) -> &mut String {
		match info {
			InfoTrait::Name => &mut self.name,
			InfoTrait::Age => &mut self.age,
			InfoTrait::Player => &mut self.player,
			InfoTrait::Concept => &mut self.concept,
			InfoTrait::Chronicle => &mut self.chronicle,
			InfoTrait::DateOfBirth => &mut self.date_of_birth,
			InfoTrait::Hair => &mut self.hair,
			InfoTrait::Eyes => &mut self.eyes,
			InfoTrait::Race => &mut self.race,
			InfoTrait::Nationality => &mut self.nationality,
			InfoTrait::Height => &mut self.height,
			InfoTrait::Weight => &mut self.weight,
			InfoTrait::Sex => &mut self.sex,
			InfoTrait::VirtueAnchor => &mut self.virtue_anchor,
			InfoTrait::ViceAnchor => &mut self.vice_anchor,
			InfoTrait::Faction => &mut self.faction,
			InfoTrait::GroupName => &mut self.group_name,
		}
	}
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_one(num: &u16) -> bool {
	num.eq(&1)
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct Attributes {
	#[serde(skip_serializing_if = "is_one")]
	pub intelligence: u16,
	#[serde(skip_serializing_if = "is_one")]
	pub wits: u16,
	#[serde(skip_serializing_if = "is_one")]
	pub resolve: u16,

	#[serde(skip_serializing_if = "is_one")]
	pub strength: u16,
	#[serde(skip_serializing_if = "is_one")]
	pub dexterity: u16,
	#[serde(skip_serializing_if = "is_one")]
	pub stamina: u16,

	#[serde(skip_serializing_if = "is_one")]
	pub presence: u16,
	#[serde(skip_serializing_if = "is_one")]
	pub manipulation: u16,
	#[serde(skip_serializing_if = "is_one")]
	pub composure: u16,
}

impl Attributes {
	pub fn get(&self, attr: &Attribute) -> &u16 {
		match attr {
			Attribute::Intelligence => &self.intelligence,
			Attribute::Wits => &self.wits,
			Attribute::Resolve => &self.resolve,
			//
			Attribute::Strength => &self.strength,
			Attribute::Dexterity => &self.dexterity,
			Attribute::Stamina => &self.stamina,
			//
			Attribute::Presence => &self.presence,
			Attribute::Manipulation => &self.manipulation,
			Attribute::Composure => &self.composure,
		}
	}

	pub fn get_mut(&mut self, attr: &Attribute) -> &mut u16 {
		match attr {
			Attribute::Intelligence => &mut self.intelligence,
			Attribute::Wits => &mut self.wits,
			Attribute::Resolve => &mut self.resolve,
			//
			Attribute::Strength => &mut self.strength,
			Attribute::Dexterity => &mut self.dexterity,
			Attribute::Stamina => &mut self.stamina,
			//
			Attribute::Presence => &mut self.presence,
			Attribute::Manipulation => &mut self.manipulation,
			Attribute::Composure => &mut self.composure,
		}
	}
}

impl Default for Attributes {
	fn default() -> Self {
		Self {
			intelligence: 1,
			wits: 1,
			resolve: 1,
			strength: 1,
			dexterity: 1,
			stamina: 1,
			presence: 1,
			manipulation: 1,
			composure: 1,
		}
	}
}

impl Sub for Attributes {
	type Output = Self;

	fn sub(self, rhs: Attributes) -> Self::Output {
		Self {
			intelligence: self.intelligence - rhs.intelligence,
			wits: self.wits - rhs.wits,
			resolve: self.resolve - rhs.resolve,
			strength: self.strength - rhs.strength,
			dexterity: self.dexterity - rhs.dexterity,
			stamina: self.stamina - rhs.stamina,
			presence: self.presence - rhs.presence,
			manipulation: self.manipulation - rhs.manipulation,
			composure: self.composure - rhs.composure,
		}
	}
}

impl Add for Attributes {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self {
			intelligence: self.intelligence + rhs.intelligence,
			wits: self.wits + rhs.wits,
			resolve: self.resolve + rhs.resolve,
			strength: self.strength + rhs.strength,
			dexterity: self.dexterity + rhs.dexterity,
			stamina: self.stamina + rhs.stamina,
			presence: self.presence + rhs.presence,
			manipulation: self.manipulation + rhs.manipulation,
			composure: self.composure + rhs.composure,
		}
	}
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_zero(n: &u16) -> bool {
	*n == 0
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct Skills {
	#[serde(skip_serializing_if = "is_zero")]
	pub academics: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub computer: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub crafts: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub investigation: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub medicine: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub occult: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub politics: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub science: u16,

	#[serde(skip_serializing_if = "is_zero")]
	pub athletics: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub brawl: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub drive: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub firearms: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub larceny: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub stealth: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub survival: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub weaponry: u16,

	#[serde(skip_serializing_if = "is_zero")]
	pub animal_ken: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub empathy: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub expression: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub intimidation: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub persuasion: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub socialize: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub streetwise: u16,
	#[serde(skip_serializing_if = "is_zero")]
	pub subterfuge: u16,
}

impl Skills {
	pub fn get(&self, skill: Skill) -> u16 {
		match skill {
			Skill::Academics => self.academics,
			Skill::Computer => self.computer,
			Skill::Crafts => self.crafts,
			Skill::Investigation => self.investigation,
			Skill::Medicine => self.medicine,
			Skill::Occult => self.occult,
			Skill::Politics => self.politics,
			Skill::Science => self.science,
			//
			Skill::Athletics => self.athletics,
			Skill::Brawl => self.brawl,
			Skill::Drive => self.drive,
			Skill::Firearms => self.firearms,
			Skill::Larceny => self.larceny,
			Skill::Stealth => self.stealth,
			Skill::Survival => self.survival,
			Skill::Weaponry => self.weaponry,
			//
			Skill::AnimalKen => self.animal_ken,
			Skill::Empathy => self.empathy,
			Skill::Expression => self.expression,
			Skill::Intimidation => self.intimidation,
			Skill::Persuasion => self.persuasion,
			Skill::Socialize => self.socialize,
			Skill::Streetwise => self.streetwise,
			Skill::Subterfuge => self.subterfuge,
		}
	}

	pub fn get_mut(&mut self, skill: Skill) -> &mut u16 {
		match skill {
			Skill::Academics => &mut self.academics,
			Skill::Computer => &mut self.computer,
			Skill::Crafts => &mut self.crafts,
			Skill::Investigation => &mut self.investigation,
			Skill::Medicine => &mut self.medicine,
			Skill::Occult => &mut self.occult,
			Skill::Politics => &mut self.politics,
			Skill::Science => &mut self.science,
			//
			Skill::Athletics => &mut self.athletics,
			Skill::Brawl => &mut self.brawl,
			Skill::Drive => &mut self.drive,
			Skill::Firearms => &mut self.firearms,
			Skill::Larceny => &mut self.larceny,
			Skill::Stealth => &mut self.stealth,
			Skill::Survival => &mut self.survival,
			Skill::Weaponry => &mut self.weaponry,
			//
			Skill::AnimalKen => &mut self.animal_ken,
			Skill::Empathy => &mut self.empathy,
			Skill::Expression => &mut self.expression,
			Skill::Intimidation => &mut self.intimidation,
			Skill::Persuasion => &mut self.persuasion,
			Skill::Socialize => &mut self.socialize,
			Skill::Streetwise => &mut self.streetwise,
			Skill::Subterfuge => &mut self.subterfuge,
		}
	}
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ArmorStruct {
	pub general: u16,
	pub ballistic: u16,
}

#[derive(Clone, Copy, Hash, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Armor {
	General,
	Ballistic,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Weapon {
	pub name: String,
	pub dice_pool: String,
	pub damage: String,
	pub range: String,
	pub initative: i16,
	pub size: u16,
}
