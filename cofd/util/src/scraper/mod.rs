use serde::{Deserialize, Serialize};

pub enum Data {
	Gift(Gift),
	Facet(Facet),
}

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
