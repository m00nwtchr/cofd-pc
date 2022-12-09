use serde::{Deserialize, Serialize};
use std::{
	cmp::min,
	collections::HashMap,
	ops::{Add, Sub},
};

use crate::prelude::VariantName;
use crate::splat::{ability::Ability, Merit, NameKey, Splat};

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
	fuel: u16,

	flag: bool,
	flag2: bool,
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
		self.fuel = fuel;
		self.flag2 = true;
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

		if self.flag2 {
			character.fuel = self.fuel;
		} else {
			character.fuel = character.max_fuel();
		}

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

pub fn athletics() -> Skill {
	Skill::Athletics
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
	pub alternate_beats: u16,

	pub conditions: Vec<String>,
	pub aspirations: Vec<String>,

	#[serde(skip)]
	_mod_map: HashMap<ModifierTarget, Vec<ModifierValue>>,
	#[serde(skip_serializing, default = "athletics")]
	_defense_skill: Skill,
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

	pub fn get_trait(&self, trait_: Trait) -> u16 {
		match trait_ {
			Trait::Speed => self.speed(),
			Trait::Defense => self.defense(),
			Trait::DefenseSkill => 0,
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

	pub fn calc_mod_map(&mut self) {
		self._mod_map.clear();

		let mut modifiers: Vec<Modifier> = Vec::new();

		modifiers.extend(
			self.abilities
				.iter()
				.flat_map(|(ability, val)| ability.get_modifiers(*val)),
		);
		modifiers.extend(
			self.merits
				.iter()
				.flat_map(|(merit, val)| merit.get_modifiers(*val)),
		);

		match &self.splat {
			Splat::Werewolf(auspice, .., data) => {
				modifiers.extend(data.form.get_modifiers());
				if let Some(auspice) = auspice {
					modifiers.extend(
						auspice.get_moon_gift().get_modifiers(
							*self
								.get_ability_value(&auspice.get_renown().clone().into())
								.unwrap_or(&0),
						),
					);

					if let Some(skill_bonus) = data.skill_bonus {
						if auspice.get_skills().contains(&skill_bonus) {
							modifiers.push(Modifier::new(
								ModifierTarget::BaseSkill(skill_bonus),
								ModifierValue::Num(1),
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
						ModifierValue::Num(1),
						ModifierOp::Add,
					));
				}

				if let Some(attr_bonus) = data.attr_bonus {
					if attr_bonus.get_type() == AttributeType::Resistance {
						modifiers.push(Modifier::new(
							ModifierTarget::BaseAttribute(attr_bonus),
							ModifierValue::Num(1),
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
							ModifierValue::Num(1),
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
							ModifierValue::Num(1),
							ModifierOp::Add,
						));
					}
				}
			}

			_ => {}
		}

		self._defense_skill = Skill::Athletics;

		// for ability in vec {
		for modifier in modifiers {
			match modifier.op {
				ModifierOp::Add => {
					let mut vecc = self._mod_map.remove(&modifier.target).unwrap_or_default();
					vecc.push(modifier.value);
					self._mod_map.insert(modifier.target, vecc);
				}
				ModifierOp::Set => {
					if let ModifierTarget::Trait(Trait::DefenseSkill) = modifier.target {
						if let ModifierValue::Skill(skill) = modifier.value {
							self._defense_skill = skill;
						}
					}
				}
			}
		}
		// }
	}

	pub fn attributes(&self) -> Attributes {
		Attributes {
			intelligence: self._modified_attr(Attribute::Intelligence),
			wits: self._modified_attr(Attribute::Wits),
			resolve: self._modified_attr(Attribute::Resolve),
			strength: self._modified_attr(Attribute::Strength),
			dexterity: self._modified_attr(Attribute::Dexterity),
			stamina: self._modified_attr(Attribute::Stamina),
			presence: self._modified_attr(Attribute::Presence),
			manipulation: self._modified_attr(Attribute::Manipulation),
			composure: self._modified_attr(Attribute::Composure),
		}
	}
	pub fn base_attributes_mut(&mut self) -> &mut Attributes {
		&mut self._attributes
	}
	pub fn base_attributes(&self) -> &Attributes {
		&self._attributes
	}

	pub fn skills(&self) -> Skills {
		Skills {
			academics: self._modified_skll(Skill::Academics),
			computer: self._modified_skll(Skill::Computer),
			crafts: self._modified_skll(Skill::Crafts),
			investigation: self._modified_skll(Skill::Investigation),
			medicine: self._modified_skll(Skill::Medicine),
			occult: self._modified_skll(Skill::Occult),
			politics: self._modified_skll(Skill::Politics),
			science: self._modified_skll(Skill::Science),

			athletics: self._modified_skll(Skill::Athletics),
			brawl: self._modified_skll(Skill::Brawl),
			drive: self._modified_skll(Skill::Drive),
			firearms: self._modified_skll(Skill::Firearms),
			larceny: self._modified_skll(Skill::Larceny),
			stealth: self._modified_skll(Skill::Stealth),
			survival: self._modified_skll(Skill::Survival),
			weaponry: self._modified_skll(Skill::Weaponry),

			animal_ken: self._modified_skll(Skill::AnimalKen),
			empathy: self._modified_skll(Skill::Empathy),
			expression: self._modified_skll(Skill::Expression),
			intimidation: self._modified_skll(Skill::Intimidation),
			persuasion: self._modified_skll(Skill::Persuasion),
			socialize: self._modified_skll(Skill::Socialize),
			streetwise: self._modified_skll(Skill::Streetwise),
			subterfuge: self._modified_skll(Skill::Subterfuge),
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
			self._mod(ModifierTarget::Trait(Trait::Health)),
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
			self._mod(ModifierTarget::Trait(Trait::Size)),
		)
	}
	pub fn speed(&self) -> u16 {
		let attributes = self.attributes();

		add(
			5 + attributes.dexterity + attributes.strength,
			self._mod(ModifierTarget::Trait(Trait::Speed)),
		)
	}
	pub fn defense(&self) -> u16 {
		let attributes = self.attributes();

		add(
			min(attributes.wits, attributes.dexterity) + *self.skills.get(self._defense_skill),
			self._mod(ModifierTarget::Trait(Trait::Defense)),
		)
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
			self._mod(ModifierTarget::Trait(Trait::Initative)),
		)
	}
	pub fn perception(&self) -> u16 {
		let attributes = self.attributes();

		add(
			attributes.wits + attributes.composure,
			self._mod(ModifierTarget::Trait(Trait::Perception)),
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

	#[warn(clippy::cast_possible_wrap)]
	pub fn _mod(&self, attr: ModifierTarget) -> i16 {
		let mut count = 0;
		if let Some(vec) = self._mod_map.get(&attr) {
			for value in vec {
				count += match value {
					ModifierValue::Num(value) => *value,
					ModifierValue::Ability(ability) => {
						*self.get_ability_value(ability).unwrap_or(&0) as i16
					}
					ModifierValue::Skill(skill) => *self.skills.get(*skill) as i16,
				}
			}
		}

		count
	}

	fn _modified_attr(&self, attr: Attribute) -> u16 {
		self._modified(ModifierTarget::Attribute(attr))
	}

	fn _modified_skll(&self, skill: Skill) -> u16 {
		self._modified(ModifierTarget::Skill(skill))
	}

	pub fn _modified(&self, target: ModifierTarget) -> u16 {
		// let base = base as i16;
		let base = *match target {
			ModifierTarget::BaseAttribute(attr) | ModifierTarget::Attribute(attr) => {
				self._attributes.get(attr)
			}
			ModifierTarget::BaseSkill(skill) | ModifierTarget::Skill(skill) => {
				self.skills.get(skill)
			}
			_ => &0,
		};

		let base_mod = self._mod(match target {
			ModifierTarget::Attribute(attr) => ModifierTarget::BaseAttribute(attr),
			ModifierTarget::Skill(skill) => ModifierTarget::BaseSkill(skill),
			_ => target,
		});
		let _mod = match target {
			ModifierTarget::BaseAttribute(_) => 0,
			ModifierTarget::BaseSkill(_) => 0,
			_ => self._mod(target),
		};

		if add(base, base_mod) > 5 {
			add(base, _mod)
		} else {
			add(base, base_mod + _mod)
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
			_mod_map: Default::default(),
			power: Default::default(),
			integrity: 7,
			fuel: Default::default(),
			_defense_skill: Skill::Athletics,
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
	pub fn get(&self, attr: Attribute) -> &u16 {
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

	pub fn get_mut(&mut self, attr: Attribute) -> &mut u16 {
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

// #[allow(clippy::trivially_copy_pass_by_ref)]
// fn is_empty(n: &String) -> bool {
// 	*n == 0
// }

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
	pub fn get(&self, skill: Skill) -> &u16 {
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

#[derive(Debug, PartialEq, Eq)]
pub struct Modifier {
	pub target: ModifierTarget,
	value: ModifierValue,
	op: ModifierOp,
}

impl Modifier {
	pub fn new(target: ModifierTarget, value: ModifierValue, op: ModifierOp) -> Self {
		Self { target, value, op }
	}

	pub fn val(&self) -> Option<i16> {
		match self.value {
			ModifierValue::Num(val) => Some(val),
			ModifierValue::Skill(_) | ModifierValue::Ability(_) => None,
		}
	}
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModifierValue {
	Num(i16),
	Ability(Ability),
	Skill(Skill),
}

#[derive(Clone, Debug, PartialEq, Eq, VariantName)]
pub enum ModifierOp {
	Add,
	Set,
}

#[derive(VariantName)]
pub enum TraitCategory {
	Mental,
	Physical,
	Social,
}

impl TraitCategory {
	pub fn unskilled(&self) -> u16 {
		match self {
			TraitCategory::Mental => 3,
			TraitCategory::Physical => 1,
			TraitCategory::Social => 1,
		}
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum AttributeType {
	Power,
	Finesse,
	Resistance,
}

pub enum AttributeCategory {
	Type(AttributeType),
	Trait(TraitCategory),
}

#[derive(
	PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize, AllVariants, VariantName,
)]
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

	pub fn get_attr(_trait: &TraitCategory, _type: &AttributeType) -> Attribute {
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

	#[allow(clippy::trivially_copy_pass_by_ref)]
	pub fn get_type(&self) -> AttributeType {
		match self {
			Attribute::Intelligence => AttributeType::Power,
			Attribute::Wits => AttributeType::Finesse,
			Attribute::Resolve => AttributeType::Resistance,
			Attribute::Strength => AttributeType::Power,
			Attribute::Dexterity => AttributeType::Finesse,
			Attribute::Stamina => AttributeType::Resistance,
			Attribute::Presence => AttributeType::Power,
			Attribute::Manipulation => AttributeType::Finesse,
			Attribute::Composure => AttributeType::Resistance,
		}
	}
}

#[derive(
	Clone,
	Copy,
	Debug,
	Hash,
	PartialEq,
	PartialOrd,
	Eq,
	Ord,
	Serialize,
	Deserialize,
	AllVariants,
	VariantName,
)]
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
}

impl NameKey for Skill {
	fn name_key(&self) -> String {
		format!("skill.{}", self.name())
	}
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Trait {
	Speed,
	Defense,
	DefenseSkill,
	Initative,
	Perception,
	Health,
	Size,

	Beats,
	AlternateBeats,

	Armor(Option<Armor>),

	Willpower,
	Power,
	Fuel,
	Integrity,
}

impl Trait {
	pub fn name(&self) -> Option<&str> {
		match self {
			Trait::Speed => Some("speed"),
			Trait::Defense => Some("defense"),
			Trait::DefenseSkill => None,
			Trait::Initative => Some("initative"),
			Trait::Perception => Some("perception"),
			Trait::Health => Some("health"),
			Trait::Size => Some("size"),
			Trait::Beats => Some("beats"),
			Trait::Armor(_) => Some("armor"),
			Trait::Willpower => Some("willpower"),
			Trait::Power => None,
			Trait::Fuel => None,
			Trait::Integrity => None,
			Trait::AlternateBeats => None,
		}
	}
}

// enum VirtueAnchor {
// 	// Virtue(Virtue),
// 	Mask(MaskDirge),
// 	_Custom(String),
// }

// enum ViceAnchor {
// 	// Vice(Vice),
// 	Dirge(MaskDirge),
// 	_Custom(String),
// }

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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Weapon {
	pub name: String,
	pub dice_pool: String,
	pub damage: String,
	pub range: String,
	pub initative: i16,
	pub size: u16,
}
