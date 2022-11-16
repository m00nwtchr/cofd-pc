use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use iced::{
	widget::{button, column, row, text, Column},
	Alignment, Length,
};
use iced_lazy::Component;

use cofd::prelude::*;

use crate::{fl, Element};

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

	fn mk_char(&self, i: usize, character: &Character) -> Element<Event> {
		let mut subtitle = fl(character.splat.name(), None).unwrap();

		if let Some(ysplat) = character.splat.ysplat() {
			subtitle = subtitle
				+ " " + &fl(character.splat.name(), Some(ysplat.name()))
				.unwrap_or_else(|| ysplat.name().to_owned());
		}

		if let Some(xsplat) = character.splat.xsplat() {
			subtitle = subtitle
				+ " " + &fl(character.splat.name(), Some(xsplat.name()))
				.unwrap_or_else(|| xsplat.name().to_owned());
		}

		// button(text(fl!("select"))).on_press(Event::PickCharacter(i))

		button(column![text(&character.info.name), text(subtitle)])
			.width(Length::Fill)
			.on_press(Event::PickCharacter(i))
			.into()
	}
}

impl<Message> Component<Message, iced::Renderer> for CharacterList<Message> {
	type State = ();

	type Event = Event;

	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		// let mut character = self.character.borrow_mut();

		match event {
			Event::PickCharacter(i) => Some((self.on_pick)(i)),
		}
	}

	#[allow(clippy::too_many_lines)]
	fn view(&self, _state: &Self::State) -> Element<Self::Event> {
		let mut list = Column::new().width(Length::FillPortion(4)).spacing(5);

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

impl<'a, Message> From<CharacterList<Message>> for Element<'a, Message>
where
	Message: 'a,
{
	fn from(character_list: CharacterList<Message>) -> Self {
		iced_lazy::component(character_list)
	}
}
