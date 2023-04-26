use std::collections::HashMap;

use cofd_util::scraper::{Merit, MeritType};
use convert_case::{Case, Casing};

pub fn parse_merits(map: HashMap<String, Vec<Vec<String>>>) -> Vec<Merit> {
	let mut merits = Vec::new();

	for (cat, vec) in map {
		for vec in vec {
			let cost = vec[1].trim();
			let mut rating = cofd_util::scraper::MeritRating::Number(0);

			if cost.chars().all(|char| char.eq(&'•')) {
				rating = cofd_util::scraper::MeritRating::Number(
					cost.chars().count().try_into().unwrap(),
				);
			} else if cost.contains("or") {
				let cost = cost.replace("or", ",");
				let mut vec: Vec<u8> = Vec::new();

				for part in cost.split(',') {
					let part = part.trim();
					if !part.is_empty() {
						vec.push(part.chars().count().try_into().unwrap());
					}
				}

				if !vec.is_empty() {
					rating = cofd_util::scraper::MeritRating::Vec(vec);
				}
			} else if cost.contains("to") {
				let mut split = cost.split("to");

				let a = split.next().unwrap().chars().count();
				let b = split.next().unwrap().chars().count();

				rating = cofd_util::scraper::MeritRating::Range(
					a.try_into().unwrap(),
					b.try_into().unwrap(),
				);
			}

			let type_ = match cat.as_str() {
				"Mental Merits" => MeritType::Mental,
				"Social Merits" => MeritType::Social,
				"Physical Merits" => MeritType::Physical,
				"Fighting Merits" => MeritType::Fightning,
				"Supernatural Merits" => MeritType::Supernatural,
				_ => todo!(),
			};

			merits.push(Merit::new(
				vec[0].to_case(Case::Pascal),
				rating,
				if vec[2].is_empty() {
					None
				} else {
					Some(vec[2].clone())
				},
				type_,
			));
		}
	}

	merits
}

// fn facet_name_to_id(name: &str) -> String {
// 	name.replace(['\'', ','], "")
// 		.to_case(convert_case::Case::Pascal)
// }
