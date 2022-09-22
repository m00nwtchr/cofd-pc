use std::{cmp::min, collections::HashMap, default};

pub mod ability;

use crate::splat::{vampire::MaskDirge, AbilityKey, MeritAbility, Splat};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use self::ability::Ability;

#[derive(Default)]
pub struct CharacterBuilder {
	splat: Splat,
	attributes: Attributes,
	skills: Skills,
	// abilities: HashMap<AbilityKey, Box<dyn ability::Ability>>,
	merits: HashMap<AbilityKey, MeritAbility>,
	flag: bool,
}

impl CharacterBuilder {
	pub fn with_splat(mut self, splat: Splat) -> Self {
		self.splat = splat;
		self
	}

	pub fn with_attributes(mut self, attributes: Attributes) -> Self {
		self.attributes = attributes;
		self
	}

	pub fn with_skills(mut self, skills: Skills) -> Self {
		self.skills = skills;
		self
	}

	// pub fn with_abilities<const N: usize>(
	// 	mut self,
	// 	abilities: [(AbilityKey, Box<dyn ability::Ability>); N],
	// ) -> Self {
	// 	self.abilities = HashMap::from(abilities);
	// 	self.flag = true;
	// 	self
	// }

	pub fn with_merits<const N: usize>(mut self, merits: [(AbilityKey, MeritAbility); N]) -> Self {
		self.merits = HashMap::from(merits);
		self.flag = true;
		self
	}

	pub fn build(self) -> Character {
		let power = if let Splat::Mortal = &self.splat {
			0
		} else {
			1
		};
		let mut character = Character {
			splat: self.splat,
			power,
			_attributes: self.attributes,
			skills: self.skills,
			// abilities: self.abilities,
			merits: self.merits,
			..Default::default()
		};

		if self.flag {
			character.calc_mod_map();
		}

		character
			.health_track
			.resize((character.max_health() as u8).into(), Wound::None);

		character.health_track.get_mut(0).unwrap().poke();

		character
	}
}

#[derive(Default, Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Wound {
	#[default]
	None = 0,
	Bashing = 1,
	Lethal = 2,
	Aggravated = 3,
}

impl Wound {
	pub fn inc(&mut self) {
		*self = match self {
			Wound::None => Wound::Bashing,
			Wound::Bashing => Wound::Lethal,
			Wound::Lethal => Wound::Aggravated,
			Wound::Aggravated => Wound::Aggravated,
		};
	}

	pub fn poke(&mut self) {
		if let Wound::Aggravated = self {
			*self = Wound::None;
		} else {
			self.inc();
		}
	}
}

pub fn athletics() -> Skill {
	Skill::Athletics
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
	splat: Splat,

	name: String,
	player: String,

	concept: String,
	chronicle: String,

	age: u8,

	virtue_anchor: String,
	vice_anchor: String,

	size: u8,
	integrity: u8,

	#[serde(rename = "attributes")]
	_attributes: Attributes,
	skills: Skills,

	health_track: Vec<Wound>,

	pub power: u8,
	fuel: u8,

	// #[serde(skip)]
	// abilities: HashMap<AbilityKey, Box<dyn ability::Ability>>,
	// #[serde(skip)]
	merits: HashMap<AbilityKey, MeritAbility>,

	#[serde(skip)]
	_mod_map: HashMap<ModifierTarget, Vec<ModifierValue>>,
	#[serde(skip_serializing, default = "athletics")]
	_defense_skill: Skill,
}

fn _mod(attr: ModifierTarget, character: &Character) -> i8 {
	let mut count = 0;
	if let Some(vec) = character._mod_map.get(&attr) {
		for value in vec {
			count += match value {
				ModifierValue::Num(value) => *value,
				// ModifierValue::Ability(key) => character
				// 	.abilities
				// 	.get(key)
				// 	.map(|a| *a.value() as i8)
				// 	.unwrap_or(0),
				ModifierValue::Skill(skill) => character.skills.get(skill) as i8,
				_ => 0,
			}
		}
	}

	count
}

fn attr_mod(attr: Attribute, character: &Character) -> i8 {
	_mod(ModifierTarget::Attribute(attr), character)
}

impl Character {
	pub fn builder() -> CharacterBuilder {
		CharacterBuilder::default()
	}

	// pub fn add_ability(&mut self, key: AbilityKey, ability: Box<dyn ability::Ability>) {
	// 	self.abilities.insert(key, ability);
	// }

	// pub fn remove_ability(&mut self, key: &AbilityKey) {
	// 	self.abilities.remove(key);
	// }

	// pub fn get_ability(&self, key: &AbilityKey) -> Option<&Box<dyn ability::Ability>> {
	// 	self.abilities.get(key)
	// }

	// pub fn get_ability_mut(&mut self, key: &AbilityKey) -> Option<&mut Box<dyn ability::Ability>> {
	// 	self.abilities.get_mut(key)
	// }

	pub fn add_merit(&mut self, key: AbilityKey, merit: MeritAbility) {
		self.merits.insert(key, merit);
	}

	pub fn remove_merit(&mut self, key: &AbilityKey) -> Option<MeritAbility> {
		self.merits.remove(key)
	}

	pub fn get_merit(&self, key: &AbilityKey) -> Option<&MeritAbility> {
		self.merits.get(key)
	}

	pub fn get_merit_mut(&mut self, key: &AbilityKey) -> Option<&mut MeritAbility> {
		self.merits.get_mut(key)
	}

	pub fn calc_mod_map(&mut self) {
		self._mod_map.clear();

		let mut vec = Vec::new();

		// vec.extend(self.abilities.values().map(|v| v.as_ref()));
		vec.extend(self.merits.values().map(|v| v));

		self._defense_skill = Skill::Athletics;

		for ability in vec {
			for modifier in ability.get_modifiers() {
				match modifier.op {
					ModifierOp::Add => {
						let mut vecc = self._mod_map.remove(&modifier.target).unwrap_or(Vec::new());
						vecc.push(modifier.value);
						self._mod_map.insert(modifier.target, vecc);
					}
					ModifierOp::Set => match modifier.target {
						ModifierTarget::Trait(_trait) => match _trait {
							Trait::DefenseSkill => {
								if let ModifierValue::Skill(skill) = modifier.value {
									self._defense_skill = skill
								}
							}
							_ => {}
						},
						_ => {}
					},
				}
			}
		}
	}

	pub fn attributes(&self) -> Attributes {
		Attributes {
			intelligence: self._attributes.intelligence + attr_mod(Attribute::Intelligence, &self),
			wits: self._attributes.wits + attr_mod(Attribute::Wits, &self),
			resolve: self._attributes.resolve + attr_mod(Attribute::Resolve, &self),
			strength: self._attributes.strength + attr_mod(Attribute::Strength, &self),
			dexterity: self._attributes.dexterity + attr_mod(Attribute::Dexterity, &self),
			stamina: self._attributes.stamina + attr_mod(Attribute::Stamina, &self),
			presence: self._attributes.presence + attr_mod(Attribute::Presence, &self),
			manipulation: self._attributes.manipulation + attr_mod(Attribute::Manipulation, &self),
			composure: self._attributes.composure + attr_mod(Attribute::Composure, &self),
		}
	}
	pub fn base_attributes_mut(&mut self) -> &mut Attributes {
		&mut self._attributes
	}

	pub fn skills(&self) -> &Skills {
		&self.skills
	}
	pub fn skills_mut(&mut self) -> &mut Skills {
		&mut self.skills
	}

	pub fn max_health(&self) -> i8 {
		let attributes = self.attributes();

		self.size as i8 + attributes.stamina
	}
	pub fn speed(&self) -> i8 {
		let attributes = self.attributes();

		5 + attributes.dexterity
			+ attributes.strength
			+ _mod(ModifierTarget::Trait(Trait::Speed), &self)
	}
	pub fn defense(&self) -> i8 {
		let attributes = self.attributes();

		min(attributes.wits, attributes.dexterity)
			+ self.skills.get(&self._defense_skill) as i8
			+ _mod(ModifierTarget::Trait(Trait::Defense), &self)
	}
	pub fn initative(&self) -> i8 {
		let attributes = self.attributes();

		attributes.dexterity
			+ attributes.composure
			+ _mod(ModifierTarget::Trait(Trait::Initative), &self)
	}
	pub fn perception(&self) -> i8 {
		let attributes = self.attributes();

		attributes.wits
			+ attributes.composure
			+ _mod(ModifierTarget::Trait(Trait::Preception), &self)
	}

	pub fn max_fuel(&self) -> i8 {
		match self.power {
			0 => self.attributes().stamina,
			1..=4 => 10 + self.power as i8 - 1,
			5..=8 => 10 + (self.power as i8 - 4) * 5,
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
			name: Default::default(),
			concept: Default::default(),
			chronicle: Default::default(),
			age: Default::default(),
			virtue_anchor: Default::default(),
			vice_anchor: Default::default(),
			_attributes: Default::default(),
			skills: Default::default(),
			size: 5,
			// abilities: Default::default(),
			merits: Default::default(),
			health_track: Default::default(),
			_mod_map: Default::default(),
			power: Default::default(),
			player: Default::default(),
			integrity: 7,
			fuel: Default::default(),
			_defense_skill: Skill::Athletics,
		}
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(default)]
pub struct Attributes {
	pub intelligence: i8,
	pub wits: i8,
	pub resolve: i8,

	pub strength: i8,
	pub dexterity: i8,
	pub stamina: i8,

	pub presence: i8,
	pub manipulation: i8,
	pub composure: i8,
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

fn is_zero(n: &u8) -> bool {
	*n == 0
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(default)]
pub struct Skills {
	#[serde(skip_serializing_if = "is_zero")]
	pub academics: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub computer: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub crafts: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub investigation: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub medicine: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub occult: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub politics: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub science: u8,

	#[serde(skip_serializing_if = "is_zero")]
	pub athletics: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub brawl: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub drive: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub firearms: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub larceny: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub stealth: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub survival: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub weaponry: u8,

	#[serde(skip_serializing_if = "is_zero")]
	pub animal_ken: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub empathy: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub expression: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub intimidation: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub persuasion: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub socialize: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub streetwise: u8,
	#[serde(skip_serializing_if = "is_zero")]
	pub subterfuge: u8,
}

impl Skills {
	pub fn get(&self, skill: &Skill) -> u8 {
		match skill {
			Skill::Academics => self.academics,
			Skill::Computer => self.computer,
			Skill::Crafts => self.crafts,
			Skill::Investigation => self.investigation,
			Skill::Medicine => self.medicine,
			Skill::Occult => self.occult,
			Skill::Politics => self.politics,
			Skill::Science => self.science,
			Skill::Athletics => self.athletics,
			Skill::Brawl => self.brawl,
			Skill::Drive => self.drive,
			Skill::Firearms => self.firearms,
			Skill::Larceny => self.larceny,
			Skill::Stealth => self.stealth,
			Skill::Survival => self.survival,
			Skill::Weaponry => self.weaponry,
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
}

#[derive(Debug)]
pub struct Modifier {
	target: ModifierTarget,
	value: ModifierValue,
	op: ModifierOp,
}

impl Modifier {
	pub fn new(target: ModifierTarget, value: ModifierValue, op: ModifierOp) -> Self {
		Self { target, value, op }
	}
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ModifierTarget {
	Attribute(Attribute),
	Trait(Trait),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ModifierValue {
	Num(i8),
	Ability(AbilityKey),
	Skill(Skill),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ModifierOp {
	Add,
	Set,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Attribute {
	Intelligence,
	Wits,
	Resolve,

	Strength,
	Dexterity,
	Stamina,

	Presence,
	Manipulation,
	Composure,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum Skill {
	Academics,
	Computer,
	Crafts,
	Investigation,
	Medicine,
	Occult,
	Politics,
	Science,

	Athletics,
	Brawl,
	Drive,
	Firearms,
	Larceny,
	Stealth,
	Survival,
	Weaponry,

	AnimalKen,
	Empathy,
	Expression,
	Intimidation,
	Persuasion,
	Socialize,
	Streetwise,
	Subterfuge,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Trait {
	Speed,
	Defense,
	DefenseSkill,
	Initative,
	Preception,
	Health,
	Size,
}

enum VirtueAnchor {
	// Virtue(Virtue),
	Mask(MaskDirge),
	_Custom(String),
}

enum ViceAnchor {
	// Vice(Vice),
	Dirge(MaskDirge),
	_Custom(String),
}
