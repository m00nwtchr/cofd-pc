use std::fmt::Write;

use crate::i18n::Translate;
use cofd::prelude::*;
use iced::{
	widget::{button, column, row, Column, text},
	Alignment, Element, Length,
};

pub struct CharacterList;

pub enum Action {
	PickCharacter(usize),
}

#[derive(Clone, Copy)]
pub enum Message {
	PickCharacter(usize),
}

impl CharacterList {
	pub fn new() -> Self {
		Self
	}

	pub fn update(&mut self, message: Message) -> Action {
		match message {
			Message::PickCharacter(i) => Action::PickCharacter(i),
		}
	}

	pub fn view(&self, characters: &[Character]) -> Element<Message> {
		let mut list = Column::new().width(Length::FillPortion(4)).spacing(5);

		for (i, character) in characters.iter().enumerate() {
			let mut subtitle = character.splat.translated();

			if let Some(ysplat) = character.splat.ysplat() {
				write!(subtitle, " {}", ysplat.translated()).unwrap();
			}

			if let Some(xsplat) = character.splat.xsplat() {
				write!(subtitle, " {}", xsplat.translated()).unwrap();
			}

			let name = if character.info.name.is_empty() {
				"Unnamed"
			} else {
				&character.info.name
			};

			list = list.push(
				button(column![text(name), text(subtitle)])
					.width(Length::Fill)
					.on_press(Message::PickCharacter(i)),
			);
		}

		row![
			column![].width(Length::Fill),
			list,
			column![].width(Length::Fill)
		]
		.padding(5)
		.width(Length::Fill)
		.align_items(Alignment::Center)
		.into()
	}
}
