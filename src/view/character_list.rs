use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use crate::i18n::flt;
use cofd::prelude::*;
use iced::{
	widget::{button, column, component, row, text, Column, Component},
	Alignment, Element, Length,
};

pub struct CharacterList<Message> {
	characters: Vec<Rc<RefCell<Character>>>,
	on_pick: Box<dyn Fn(usize) -> Message + 'static>,

	phantom: PhantomData<Message>,
}

pub fn character_list<Message>(
	characters: Vec<Rc<RefCell<Character>>>,
	on_pick: impl Fn(usize) -> Message + 'static,
) -> CharacterList<Message> {
	CharacterList::new(characters, on_pick)
}

#[derive(Clone)]
pub enum Event {
	PickCharacter(usize),
}

impl<Message> CharacterList<Message> {
	pub fn new(
		characters: Vec<Rc<RefCell<Character>>>,
		on_pick: impl Fn(usize) -> Message + 'static,
	) -> Self {
		Self {
			characters,
			phantom: PhantomData,
			on_pick: Box::new(on_pick),
		}
	}

	fn mk_char<Theme>(&self, i: usize, character: &Character) -> Element<Event, Theme>
	where
		Theme: button::StyleSheet + text::StyleSheet + 'static,
	{
		let mut subtitle = flt(character.splat.name(), None).unwrap();

		if let Some(ysplat) = character.splat.ysplat() {
			subtitle = subtitle
				+ " " + &flt(character.splat.name(), Some(ysplat.name()))
				.unwrap_or_else(|| ysplat.name().to_owned());
		}

		if let Some(xsplat) = character.splat.xsplat() {
			subtitle = subtitle
				+ " " + &flt(character.splat.name(), Some(xsplat.name()))
				.unwrap_or_else(|| xsplat.name().to_owned());
		}

		let name = if character.info.name.is_empty() {
			"Unnamed"
		} else {
			&character.info.name
		};

		button(column![text(name), text(subtitle)])
			.width(Length::Fill)
			.on_press(Event::PickCharacter(i))
			.into()
	}
}

impl<Message, Theme> Component<Message, Theme> for CharacterList<Message>
where
	Theme: button::StyleSheet + text::StyleSheet + 'static,
{
	type State = ();
	type Event = Event;

	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		match event {
			Event::PickCharacter(i) => Some((self.on_pick)(i)),
		}
	}

	fn view(&self, _state: &Self::State) -> Element<'_, Event, Theme> {
		let mut list = Column::new()
			.width(Length::FillPortion(4))
			.spacing(5);

		for (i, character) in self.characters.iter().enumerate() {
			list = list.push(self.mk_char(i, &character.borrow()));
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

impl<'a, Message, Theme> From<CharacterList<Message>> for Element<'a, Message, Theme>
where
	Theme: button::StyleSheet + text::StyleSheet + 'static,
	Message: 'a,
{
	fn from(character_list: CharacterList<Message>) -> Self {
		component(character_list)
	}
}
