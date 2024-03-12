use serde::{Deserialize, Serialize};

// pub enum Data {
// 	Gift(Gift),
// 	Facet(Facet),
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Gift {
	pub name: String,
	#[serde(rename = "type")]
	pub type_: String,
	pub facets: Vec<Facet>,
}

impl Gift {
	pub fn new(name: String, type_: String) -> Self {
		Self {
			name,
			type_,
			facets: vec![],
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Facet {
	// Moon Gift
	Moon { name: String, level: u16 },
	// Wolf/Shadow Gift
	Other { name: String, renown: String },
	//                        // pub cost: String,
	//                        // pub pool: String,
	//                        // pub action: String,
	//                        // pub duration: String,
	//                       // pub description: String,
	//                        // pub reference: String,
}

impl Facet {
	pub fn name(&self) -> &String {
		match self {
			Self::Moon { name, .. } => name,
			Self::Other { name, .. } => name,
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MeritType {
	Mental,
	Physical,
	Social,
	Supernatural,
	Fightning,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MeritRating {
	Number(u8),
	Range(u8, u8),
	Vec(Vec<u8>),
}

// #[derive(Debug, Serialize, Deserialize)]
// pub enum Requirement {
// 	Attribute(String, u8),
// 	Skill(String, u8),
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Merit {
	name: String,
	cost: MeritRating,
	// requirements: Vec<Requirement>,
	requirements: Option<String>,
	type_: MeritType,
}

impl Merit {
	pub fn new(
		name: String,
		cost: MeritRating,
		requirements: Option<String>,
		type_: MeritType,
	) -> Self {
		Self {
			name,
			cost,
			requirements,
			type_,
		}
	}
}
