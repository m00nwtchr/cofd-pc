use cofd_util::scraper::{Facet, Gift};
use convert_case::Casing;

pub fn parse_gifts(vec: Vec<Vec<String>>) -> Vec<Gift> {
	let mut gift = None;
	let mut gifts = Vec::new();

	for vec in vec {
		if vec.len() == 2 {
			if let Some(g) = gift {
				gifts.push(g);
				gift = None;
			}

			if let Some(name) = vec.first() {
				if !name.contains('(') {
					gift = Some(Gift::new(
						gift_name_to_id(name).to_string(),
						vec.last().unwrap().split(' ').next().unwrap().to_string(),
					));
				}
			}
		} else if let Some(gift) = &mut gift {
			let id = facet_name_to_id(&vec[0]);

			let str = vec[1].clone();

			if str.contains('•') {
				gift.facets.push(Facet::Moon {
					name: id,
					level: str.chars().count() as u16,
				});
			} else {
				gift.facets.push(Facet::Other {
					name: id,
					renown: str,
				});
			}
		}
	}
	if let Some(g) = gift {
		gifts.push(g);
	}

	gifts
}

fn gift_name_to_id(name: &str) -> &str {
	if name.contains("of") {
		name.split(' ').last().unwrap()
	} else {
		let next = name.split(' ').next().unwrap();
		if next.contains('\'') {
			next.strip_suffix("\'s").unwrap()
		} else {
			next
		}
	}
}

fn facet_name_to_id(name: &str) -> String {
	name.replace(['\'', ','], "")
		.to_case(convert_case::Case::Pascal)
}
