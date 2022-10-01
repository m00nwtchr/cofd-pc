use std::{
	cmp::min,
	collections::{BTreeMap, BTreeSet, HashMap, HashSet},
	default,
};

use crate::splat::{
	ability::{Ability, AbilityVal},
	vampire::MaskDirge,
	Splat,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Default)]
pub struct CharacterBuilder {
	splat: Splat,
	attributes: Attributes,
	skills: Skills,
	abilities: BTreeMap<Ability, AbilityVal>,
	// abilities: HashMap<AbilityKey, Box<dyn ability::Ability>>,
	// merits: HashMap<AbilityKey, MeritAbility>,
	merits: Vec<AbilityVal>,
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

	pub fn with_abilities<const N: usize>(mut self, abilities: [AbilityVal; N]) -> Self {
		self.abilities = BTreeMap::new();

		for ability in abilities {
			self.abilities.insert(ability.0.clone(), ability);
		}

		self.flag = true;
		self
	}

	pub fn with_merits<const N: usize>(mut self, merits: [AbilityVal; N]) -> Self {
		self.merits = Vec::from(merits);
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
			abilities: self.abilities,
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
	pub splat: Splat,

	pub info: CharacterInfo,

	virtue_anchor: String,
	vice_anchor: String,

	pub base_size: u8,
	#[serde(rename = "attributes")]
	_attributes: Attributes,
	skills: Skills,

	pub health_track: Vec<Wound>,

	pub willpower: u8,
	pub power: u8,
	fuel: u8,
	pub integrity: u8,

	// #[serde(skip)]
	pub abilities: BTreeMap<Ability, AbilityVal>,
	// #[serde(skip)]
	merits: Vec<AbilityVal>,

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
				ModifierValue::Ability(ability) => character
					.get_ability(&ability)
					.map(|a| a.1 as i8)
					.unwrap_or_default(),
				ModifierValue::Skill(skill) => character.skills.get(skill).clone() as i8,
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

	pub fn add_ability(&mut self, ability: AbilityVal) {
		self.abilities.insert(ability.0.clone(), ability);
	}

	pub fn has_ability(&self, key: &Ability) -> bool {
		self.abilities.contains_key(key)
	}

	pub fn remove_ability(&mut self, key: &Ability) -> Option<AbilityVal> {
		self.abilities.remove(key)
	}

	pub fn get_ability(&self, key: &Ability) -> Option<&AbilityVal> {
		self.abilities.get(key)
	}

	pub fn get_ability_mut(&mut self, key: &Ability) -> Option<&mut AbilityVal> {
		self.abilities.get_mut(key)
	}

	pub fn add_merit(&mut self, key: AbilityVal) {
		self.merits.push(key)
	}

	pub fn remove_merit(&mut self, i: usize) -> AbilityVal {
		self.merits.remove(i)
	}

	pub fn get_merit(&self, i: usize) -> Option<&AbilityVal> {
		self.merits.get(i)
	}

	pub fn get_merit_mut(&mut self, i: usize) -> Option<&mut AbilityVal> {
		self.merits.get_mut(i)
	}

	pub fn calc_mod_map(&mut self) {
		self._mod_map.clear();

		let mut modifiers: Vec<Modifier> = Vec::new();

		modifiers.extend(self.abilities.values().flat_map(|f| f.get_modifiers()));
		modifiers.extend(self.merits.iter().flat_map(|m| m.get_modifiers()));

		match &self.splat {
			Splat::Werewolf(auspice, _, _, data) => {
				modifiers.extend(data.form.get_modifiers());
				if let Some(auspice) = auspice {
					modifiers.extend(
						auspice.get_moon_gift().get_modifiers(
							self.get_ability(&Ability::Renown(auspice.get_renown().clone()))
								.map(|f| f.1)
								.unwrap_or(0),
						),
					);
				}
			}
			_ => {}
		}

		self._defense_skill = Skill::Athletics;

		// for ability in vec {
		for modifier in modifiers {
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
		// }
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
	pub fn base_attributes(&self) -> &Attributes {
		&self._attributes
	}

	pub fn skills(&self) -> &Skills {
		&self.skills
	}
	pub fn skills_mut(&mut self) -> &mut Skills {
		&mut self.skills
	}

	pub fn max_health(&self) -> i8 {
		let attributes = self.attributes();

		self.size() + attributes.stamina + _mod(ModifierTarget::Trait(Trait::Health), &self)
	}

	pub fn max_willpower(&self) -> u8 {
		let attributes = self.attributes();

		attributes.resolve as u8 + attributes.composure as u8
	}

	pub fn size(&self) -> i8 {
		self.base_size as i8 + _mod(ModifierTarget::Trait(Trait::Size), &self)
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
			+ self.skills.get(&self._defense_skill).clone() as i8
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
		let mut s = Self {
			splat: Default::default(),
			info: Default::default(),
			virtue_anchor: Default::default(),
			vice_anchor: Default::default(),
			_attributes: Default::default(),
			skills: Default::default(),
			base_size: 5,
			abilities: Default::default(),
			merits: Default::default(),
			health_track: Default::default(),
			_mod_map: Default::default(),
			power: Default::default(),
			integrity: 7,
			fuel: Default::default(),
			_defense_skill: Skill::Athletics,
			willpower: 0,
		};

		s.willpower = s.max_willpower();

		s
	}
}

#[derive(Clone, Default, Serialize, Deserialize, Debug, PartialEq)]
#[serde(default)]
pub struct CharacterInfo {
	name: String,
	player: String,

	virtue_anchor: String,
	vice_anchor: String,

	faction: String,
	group_name: String,

	concept: String,
	chronicle: String,

	age: String,
	date_of_birth: String,
	hair: String,
	eyes: String,
	race: String,
	nationality: String,
	height: String,
	weight: String,
	sex: String,

	other: String,
}

#[derive(Debug, Clone, Copy)]
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

impl InfoTrait {
	pub fn name(&self) -> &str {
		match self {
			InfoTrait::Name => "name",
			InfoTrait::Age => "age",
			InfoTrait::Player => "player",
			InfoTrait::Concept => "concept",
			InfoTrait::Chronicle => "chronicle",
			InfoTrait::DateOfBirth => "dob",
			InfoTrait::Hair => "hair",
			InfoTrait::Eyes => "eyes",
			InfoTrait::Race => "race",
			InfoTrait::Nationality => "nationality",
			InfoTrait::Height => "height",
			InfoTrait::Weight => "weight",
			InfoTrait::Sex => "sex",
			InfoTrait::VirtueAnchor => "virtue",
			InfoTrait::ViceAnchor => "vice",
			InfoTrait::Faction => "faction",
			InfoTrait::GroupName => "group_name",
		}
	}
}

impl CharacterInfo {
	pub fn get(&self, info: &InfoTrait) -> &String {
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

	pub fn get_mut(&mut self, info: &InfoTrait) -> &mut String {
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

impl Attributes {
	pub fn get(&self, attr: &Attribute) -> i8 {
		match attr {
			Attribute::Intelligence => self.intelligence,
			Attribute::Wits => self.wits,
			Attribute::Resolve => self.resolve,
			//
			Attribute::Strength => self.strength,
			Attribute::Dexterity => self.dexterity,
			Attribute::Stamina => self.stamina,
			//
			Attribute::Presence => self.presence,
			Attribute::Manipulation => self.manipulation,
			Attribute::Composure => self.composure,
		}
	}

	pub fn get_mut(&mut self, attr: &Attribute) -> &mut i8 {
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
	pub fn get(&self, skill: &Skill) -> &u8 {
		match skill {
			Skill::Academics => &self.academics,
			Skill::Computer => &self.computer,
			Skill::Crafts => &self.crafts,
			Skill::Investigation => &self.investigation,
			Skill::Medicine => &self.medicine,
			Skill::Occult => &self.occult,
			Skill::Politics => &self.politics,
			Skill::Science => &self.science,
			//
			Skill::Athletics => &self.athletics,
			Skill::Brawl => &self.brawl,
			Skill::Drive => &self.drive,
			Skill::Firearms => &self.firearms,
			Skill::Larceny => &self.larceny,
			Skill::Stealth => &self.stealth,
			Skill::Survival => &self.survival,
			Skill::Weaponry => &self.weaponry,
			//
			Skill::AnimalKen => &self.animal_ken,
			Skill::Empathy => &self.empathy,
			Skill::Expression => &self.expression,
			Skill::Intimidation => &self.intimidation,
			Skill::Persuasion => &self.persuasion,
			Skill::Socialize => &self.socialize,
			Skill::Streetwise => &self.streetwise,
			Skill::Subterfuge => &self.subterfuge,
		}
	}

	pub fn get_mut(&mut self, skill: &Skill) -> &mut u8 {
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
	Ability(Ability),
	Skill(Skill),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ModifierOp {
	Add,
	Set,
}

pub enum TraitCategory {
	Mental,
	Physical,
	Social,
}

impl TraitCategory {
	pub fn name(&self) -> &str {
		match self {
			TraitCategory::Mental => "mental",
			TraitCategory::Physical => "physical",
			TraitCategory::Social => "social",
		}
	}

	pub fn unskilled(&self) -> u8 {
		match self {
			TraitCategory::Mental => 3,
			TraitCategory::Physical => 1,
			TraitCategory::Social => 1,
		}
	}
}

pub enum AttributeType {
	Power,
	Finesse,
	Resistance,
}

pub enum AttributeCategory {
	Type(AttributeType),
	Trait(TraitCategory),
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

impl Attribute {
	pub fn all() -> [Attribute; 9] {
		[
			Self::Intelligence,
			Self::Wits,
			Self::Resolve,
			//
			Self::Strength,
			Self::Dexterity,
			Self::Stamina,
			//
			Self::Presence,
			Self::Manipulation,
			Self::Composure,
		]
	}

	pub fn mental() -> [Attribute; 3] {
		[Self::Intelligence, Self::Wits, Self::Resolve]
	}

	pub fn physical() -> [Attribute; 3] {
		[Self::Strength, Self::Dexterity, Self::Stamina]
	}

	pub fn social() -> [Attribute; 3] {
		[Self::Presence, Self::Manipulation, Self::Composure]
	}

	pub fn power() -> [Attribute; 3] {
		[Self::Intelligence, Self::Strength, Self::Presence]
	}

	pub fn finesse() -> [Attribute; 3] {
		[Self::Wits, Self::Dexterity, Self::Manipulation]
	}

	pub fn resistance() -> [Attribute; 3] {
		[Self::Resolve, Self::Stamina, Self::Composure]
	}

	pub fn get(cat: AttributeCategory) -> [Attribute; 3] {
		match cat {
			AttributeCategory::Type(_type) => match _type {
				AttributeType::Power => Self::power(),
				AttributeType::Finesse => Self::finesse(),
				AttributeType::Resistance => Self::resistance(),
			},
			AttributeCategory::Trait(_trait) => match _trait {
				TraitCategory::Mental => Self::mental(),
				TraitCategory::Physical => Self::physical(),
				TraitCategory::Social => Self::social(),
			},
		}
	}

	pub fn get_attr(_trait: TraitCategory, _type: AttributeType) -> Attribute {
		match _trait {
			TraitCategory::Mental => match _type {
				AttributeType::Power => Self::Intelligence,
				AttributeType::Finesse => Self::Wits,
				AttributeType::Resistance => Self::Resolve,
			},
			TraitCategory::Physical => match _type {
				AttributeType::Power => Self::Strength,
				AttributeType::Finesse => Self::Dexterity,
				AttributeType::Resistance => Self::Stamina,
			},
			TraitCategory::Social => match _type {
				AttributeType::Power => Self::Presence,
				AttributeType::Finesse => Self::Manipulation,
				AttributeType::Resistance => Self::Composure,
			},
		}
	}

	pub fn name(&self) -> &str {
		match self {
			Attribute::Intelligence => "intelligence",
			Attribute::Wits => "wits",
			Attribute::Resolve => "resolve",
			//
			Attribute::Strength => "strength",
			Attribute::Dexterity => "dexterity",
			Attribute::Stamina => "stamina",
			//
			Attribute::Presence => "presence",
			Attribute::Manipulation => "manipulation",
			Attribute::Composure => "composure",
		}
	}
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
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

impl Skill {
	fn all() -> [Skill; 24] {
		[
			Self::Academics,
			Self::Computer,
			Self::Crafts,
			Self::Investigation,
			Self::Medicine,
			Self::Occult,
			Self::Politics,
			Self::Science,
			//
			Self::Athletics,
			Self::Brawl,
			Self::Drive,
			Self::Firearms,
			Self::Larceny,
			Self::Stealth,
			Self::Survival,
			Self::Weaponry,
			//
			Self::AnimalKen,
			Self::Empathy,
			Self::Expression,
			Self::Intimidation,
			Self::Persuasion,
			Self::Socialize,
			Self::Streetwise,
			Self::Subterfuge,
		]
	}

	fn mental() -> [Skill; 8] {
		[
			Self::Academics,
			Self::Computer,
			Self::Crafts,
			Self::Investigation,
			Self::Medicine,
			Self::Occult,
			Self::Politics,
			Self::Science,
		]
	}

	fn physical() -> [Skill; 8] {
		[
			Self::Athletics,
			Self::Brawl,
			Self::Drive,
			Self::Firearms,
			Self::Larceny,
			Self::Stealth,
			Self::Survival,
			Self::Weaponry,
		]
	}

	fn social() -> [Skill; 8] {
		[
			Self::AnimalKen,
			Self::Empathy,
			Self::Expression,
			Self::Intimidation,
			Self::Persuasion,
			Self::Socialize,
			Self::Streetwise,
			Self::Subterfuge,
		]
	}

	pub fn get(cat: &TraitCategory) -> [Skill; 8] {
		match cat {
			TraitCategory::Mental => Self::mental(),
			TraitCategory::Physical => Self::physical(),
			TraitCategory::Social => Self::social(),
		}
	}

	pub fn name(&self) -> &str {
		match self {
			Skill::Academics => "academics",
			Skill::Computer => "computer",
			Skill::Crafts => "crafts",
			Skill::Investigation => "investigation",
			Skill::Medicine => "medicine",
			Skill::Occult => "occult",
			Skill::Politics => "politics",
			Skill::Science => "science",
			//
			Skill::Athletics => "athletics",
			Skill::Brawl => "brawl",
			Skill::Drive => "drive",
			Skill::Firearms => "firearms",
			Skill::Larceny => "larceny",
			Skill::Stealth => "stealth",
			Skill::Survival => "survival",
			Skill::Weaponry => "weaponry",
			//
			Skill::AnimalKen => "animal_ken",
			Skill::Empathy => "empathy",
			Skill::Expression => "expression",
			Skill::Intimidation => "intimidation",
			Skill::Persuasion => "persuasion",
			Skill::Socialize => "socialize",
			Skill::Streetwise => "streetwise",
			Skill::Subterfuge => "subterfuge",
		}
	}
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

	Willpower,
	Power,
	Fuel,
	Integrity,
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
