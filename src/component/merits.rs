use cofd::{
	prelude::*,
	splat::{Merit, SplatTrait},
};
use iced::{
	widget::{column, pick_list, row, text, text_input, Column},
	Alignment, Length,
};

use crate::{
	fl, i18n,
	i18n::Translated,
	widget::dots::{Shape, SheetDots},
	Element, H3_SIZE, INPUT_PADDING, TITLE_SPACING,
};

#[derive(Debug, Clone)]
pub struct MeritComponent;

#[derive(Clone)]
pub struct Message(usize, Merit, u16);

impl MeritComponent {
	pub fn new() -> Self {
		Self
	}

	pub fn update(&mut self, message: Message, character: &mut Character) {
		let Message(i, ability, val) = message;
		if let Merit::_Custom(str) = &ability {
			if str.contains("---") {
				return;
			}
		}

		let mut flag = false;

		if character.merits.len() == i {
			if !ability.get_modifiers(val).is_empty() {
				flag = true;
			}
			character.merits.push((ability, val));
		} else {
			let old = character.merits.remove(i);
			if old.0.get_modifiers(old.1) != ability.get_modifiers(val) {
				flag = true;
			}

			if !ability.name().is_empty() {
				character.merits.insert(i, (ability, val));
			}
		}

		if flag {
			character.calc_mod_map();
		}
	}

	pub fn view(&self, character: &Character) -> Element<Message> {
		let mut col1 = Column::new().spacing(3).width(Length::FillPortion(3));
		let mut col2 = Column::new()
			.spacing(4)
			.width(Length::FillPortion(2))
			.align_items(Alignment::End);

		let mut vec = Vec::new();

		vec.push(Merit::_Custom(String::from("--- Mental Merits ---")));
		vec.extend(Merit::mental());

		vec.push(Merit::_Custom(String::from("--- Physical Merits ---")));
		vec.extend(Merit::physical());

		vec.push(Merit::_Custom(String::from("--- Social Merits ---")));
		vec.extend(Merit::social());

		vec.push(Merit::_Custom(format!(
			"--- {} Merits ---",
			i18n::LANGUAGE_LOADER.get(character.splat.name())
		)));
		vec.extend(character.splat.merits());

		vec.push(Merit::_Custom(fl!("custom")));

		let attributes = &character.attributes();
		let skills = &character.skills();

		let vec: Vec<Translated<Merit>> = vec
			.iter()
			.filter(|&e| {
				character
					.merits
					.iter()
					.filter(|(merit, _)| *merit == *e)
					.count() == 0 && e.is_available(character, attributes, skills)
			})
			.cloned()
			.map(Into::into)
			.collect();

		for (i, (merit, val)) in character.merits.iter().cloned().enumerate() {
			if let Merit::_Custom(str) = &merit {
				col1 = col1.push(
					text_input("", str)
						.on_input(move |key| Message(i, Merit::_Custom(key), val))
						.padding(INPUT_PADDING),
				);
			} else {
				col1 = col1
					.push(
						pick_list(
							vec.clone(),
							Some::<Translated<Merit>>(merit.clone().into()),
							move |key| Message(i, key.unwrap(), val),
						)
						.padding(INPUT_PADDING)
						.text_size(20)
						.width(Length::Fill),
					)
					.spacing(1);
			}

			col2 = col2.push(SheetDots::new(val, 0, 5, Shape::Dots, None, {
				let merit = merit.clone();
				move |val| Message(i, merit.clone(), val)
			}));
		}

		let new = pick_list(vec, None::<Translated<Merit>>, {
			let len = character.merits.len();
			move |key| Message(len, key.unwrap(), 0)
		})
		.padding(INPUT_PADDING)
		.text_size(20)
		.width(Length::Fill);

		column![
			text(fl!("merits")).size(H3_SIZE),
			column![row![col1, col2], new]
		]
		.spacing(TITLE_SPACING)
		.align_items(Alignment::Center)
		.into()
	}
}
