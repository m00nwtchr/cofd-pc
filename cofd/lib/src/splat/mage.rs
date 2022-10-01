use serde::{Deserialize, Serialize};

use crate::character::Skill;

use super::{ability::Ability, XSplat};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Path {
	Acanthus,
	Mastigos,
	Moros,
	Obrimos,
	Thyrsus,
	_Custom(String, [Arcanum; 2], Arcanum),
}

impl Path {
	pub fn all() -> [Path; 5] {
		[
			Path::Acanthus,
			Path::Mastigos,
			Path::Moros,
			Path::Obrimos,
			Path::Thyrsus,
		]
	}

	pub fn name(&self) -> &str {
		match self {
			Path::Acanthus => "acanthus",
			Path::Mastigos => "mastigos",
			Path::Moros => "moros",
			Path::Obrimos => "obrimos",
			Path::Thyrsus => "thyrsus",
			Path::_Custom(name, _, _) => name,
		}
	}

	fn get_ruling_arcana(&self) -> &[Arcanum; 2] {
		match self {
			Path::Acanthus => &[Arcanum::Time, Arcanum::Fate],
			Path::Mastigos => &[Arcanum::Space, Arcanum::Mind],
			Path::Moros => &[Arcanum::Matter, Arcanum::Death],
			Path::Obrimos => &[Arcanum::Forces, Arcanum::Prime],
			Path::Thyrsus => &[Arcanum::Life, Arcanum::Spirit],
			Path::_Custom(_, ruling, _) => ruling,
		}
	}
	fn get_inferior_arcanum(&self) -> &Arcanum {
		match self {
			Path::Acanthus => &Arcanum::Forces,
			Path::Mastigos => &Arcanum::Matter,
			Path::Moros => &Arcanum::Spirit,
			Path::Obrimos => &Arcanum::Death,
			Path::Thyrsus => &Arcanum::Mind,
			Path::_Custom(_, _, inferior) => inferior,
		}
	}
}

impl Into<XSplat> for Path {
	fn into(self) -> XSplat {
		XSplat::Mage(self)
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Order {
	AdamantineArrow,
	GuardiansOfTheVeil,
	Mysterium,
	SilverLadder,
	FreeCouncil,
	SeersOfTheThrone(Option<Ministry>),
	_Custom(String, [Skill; 3]),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Ministry {
	Hegemony,
	Panopticon,
	Paternoster,
	Praetorian,
	_Custom(String, [Skill; 3]),
}

impl Order {
	pub fn get_rote_skills(&self) -> &[Skill; 3] {
		match self {
			Order::AdamantineArrow => &[Skill::Athletics, Skill::Intimidation, Skill::Medicine],
			Order::GuardiansOfTheVeil => &[Skill::Investigation, Skill::Stealth, Skill::Subterfuge],
			Order::Mysterium => &[Skill::Investigation, Skill::Occult, Skill::Survival],
			Order::SilverLadder => &[Skill::Expression, Skill::Persuasion, Skill::Subterfuge],
			Order::FreeCouncil => &[Skill::Crafts, Skill::Persuasion, Skill::Science],
			Order::SeersOfTheThrone(ministry) => match ministry {
				Some(ministry) => match ministry {
					Ministry::Hegemony => &[Skill::Politics, Skill::Persuasion, Skill::Empathy],
					Ministry::Panopticon => {
						&[Skill::Investigation, Skill::Stealth, Skill::Subterfuge]
					}
					Ministry::Paternoster => &[Skill::Academics, Skill::Occult, Skill::Expression],
					Ministry::Praetorian => {
						&[Skill::Athletics, Skill::Larceny, Skill::Intimidation]
					}
					Ministry::_Custom(_, skills) => skills,
				},
				None => &[Skill::Investigation, Skill::Occult, Skill::Persuasion],
			},
			Order::_Custom(_, skills) => skills,
		}
	}
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Legacy {
	_Custom(String, Arcanum),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub enum Arcanum {
	Death,
	Fate,
	Forces,
	Life,
	Matter,
	Mind,
	Prime,
	Space,
	Spirit,
	Time,
}
impl Arcanum {
	pub fn all() -> [Arcanum; 10] {
		[
			Arcanum::Death,
			Arcanum::Fate,
			Arcanum::Forces,
			Arcanum::Life,
			Arcanum::Matter,
			Arcanum::Mind,
			Arcanum::Prime,
			Arcanum::Space,
			Arcanum::Spirit,
			Arcanum::Time,
		]
	}

	pub fn name(&self) -> &str {
		match self {
			Arcanum::Death => "death",
			Arcanum::Fate => "fate",
			Arcanum::Forces => "forces",
			Arcanum::Life => "life",
			Arcanum::Matter => "matter",
			Arcanum::Mind => "mind",
			Arcanum::Prime => "prime",
			Arcanum::Space => "space",
			Arcanum::Spirit => "spirit",
			Arcanum::Time => "time",
		}
	}
}

impl Into<Ability> for Arcanum {
	fn into(self) -> Ability {
		Ability::Arcanum(self)
	}
}
