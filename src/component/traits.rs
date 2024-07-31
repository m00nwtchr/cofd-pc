use cofd::prelude::*;
use cofd::traits::DerivedTrait;
use iced::{
	widget::{column, row, text, text_input},
	Length,
};

use crate::{fl, i18n, Element, INPUT_PADDING};

#[derive(Debug, Clone)]
pub struct TraitsComponent;

#[derive(Clone)]
pub struct Message(u16, Trait);

impl TraitsComponent {
	pub fn new() -> Self {
		Self
	}

	pub fn update(&mut self, message: Message, character: &mut Character) {
		let Message(val, _trait) = message;

		match _trait {
			Trait::DerivedTrait(DerivedTrait::Size) => {
				character.base_size = (val as i16
					- character.get_modifier(Trait::DerivedTrait(DerivedTrait::Size)))
					as u16;
			}
			Trait::Willpower => character.willpower = val,
			Trait::Power => character.power = val,
			Trait::Fuel => character.fuel = val,
			Trait::Integrity => character.integrity = val,
			Trait::Beats => character.beats = val,
			Trait::AlternateBeats => character.alternate_beats = val,
			_ => {}
		}
	}

	pub fn view(&self, character: &Character) -> Element<Message> {
		let beats = row![
			text(format!("{}:", fl!("beats"))),
			text_input("", &format!("{}", character.beats))
				.on_input(|val| { Message(val.parse().unwrap_or(0), Trait::Beats) })
				.padding(INPUT_PADDING)
		];

		let alternate_beats = if character.splat.alternate_beats_optional() {
			row![]
		} else {
			let name = i18n::LANGUAGE_LOADER.get(&format!(
				"{}-experience",
				character.splat.alternate_beats().unwrap()
			));

			row![
				text(format!("{name}:")),
				text_input("", &format!("{}", character.alternate_beats))
					.on_input(|val| { Message(val.parse().unwrap_or(0), Trait::AlternateBeats) })
					.padding(INPUT_PADDING)
			]
		};

		let alternate_xp = if character.splat.alternate_beats_optional() {
			row![]
		} else {
			let name = i18n::LANGUAGE_LOADER.get(&format!(
				"{}-beats",
				character.splat.alternate_beats().unwrap()
			));

			row![text(format!(
				"{name}: {}",
				character.alternate_experience()
			))]
		};

		let armor = character.armor();
		column![
			row![
				text(format!("{}: {}", fl!("size"), character.size())),
				// text_input("", &format!("{}", self.traits.size), |val| {
				// 	Event(val, Trait::Size)
				// })
			],
			row![text(format!("{}: {}", fl!("speed"), character.speed())),],
			row![text(format!("{}: {}", fl!("defense"), character.defense())),],
			row![
				text(format!(
					"{}: {}/{}",
					fl!("armor"),
					armor.general,
					armor.ballistic
				)),
				// text_input("", &format!("{}", self.traits.beats), |val| {
				// 	// if let Some(val) = val.parse() {
				// 	Event(val, Trait::Armor(Armor::General))
				// 	// }
				// }),
				// text("/"),
				// text_input("", &format!("{}", self.traits.beats), |val| {
				// 	// if let Some(val) = val.parse() {
				// 	Event(val, Trait::Armor(Armor::Ballistic))
				// 	// }
				// })
			],
			row![text(format!(
				"{}: {}",
				fl!("initiative"),
				character.initiative()
			)),],
			beats,
			row![text(format!(
				"{}: {}",
				fl!("experience"),
				character.experience()
			)),],
			alternate_beats,
			alternate_xp
		]
		// .padding(0)
		.width(Length::Fill)
		.into()
	}
}
