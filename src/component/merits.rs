use iced::{
	widget::{column, pick_list, row, text, text_input, Column},
	Alignment, Length,
};
use std::{cell::RefCell, rc::Rc};

use crate::i18n::Translated;
use crate::widget::dots;
use crate::{
	fl, i18n,
	i18n::Translate,
	widget::dots::{Shape, SheetDots},
	Element, H3_SIZE, INPUT_PADDING, TITLE_SPACING,
};
use cofd::{
	prelude::*,
	splat::{Merit, SplatTrait},
};
use iced::widget::{component, container, overlay, scrollable, Component};

pub struct MeritComponent<Message> {
	character: Rc<RefCell<Character>>,
	on_change: Box<dyn Fn(usize, Merit, u16) -> Message>,
}

pub fn merit_component<Message>(
	character: Rc<RefCell<Character>>,
	on_change: impl Fn(usize, Merit, u16) -> Message + 'static,
) -> MeritComponent<Message> {
	MeritComponent::new(character, on_change)
}

#[derive(Clone)]
pub struct Event(usize, Merit, u16);

impl<Message> MeritComponent<Message> {
	fn new(
		character: Rc<RefCell<Character>>,
		on_change: impl Fn(usize, Merit, u16) -> Message + 'static,
	) -> Self {
		Self {
			character,
			on_change: Box::new(on_change),
		}
	}
}

impl<Message, Theme> Component<Message, Theme> for MeritComponent<Message>
where
	Theme: text::StyleSheet
		+ dots::StyleSheet
		+ text_input::StyleSheet
		+ pick_list::StyleSheet
		+ scrollable::StyleSheet
		+ overlay::menu::StyleSheet
		+ container::StyleSheet
		+ 'static,
	<Theme as overlay::menu::StyleSheet>::Style: From<<Theme as pick_list::StyleSheet>::Style>,
{
	type State = ();
	type Event = Event;

	fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Message> {
		if let Merit::_Custom(str) = &event.1 {
			if str.contains("---") {
				return None;
			}
		}

		Some((self.on_change)(event.0, event.1, event.2))
	}

	fn view(&self, _state: &Self::State) -> Element<Event, Theme> {
		let character = self.character.borrow();

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
					.count() == 0 && e.is_available(&character, attributes, skills)
			})
			.cloned()
			.map(Into::into)
			.collect();

		for (i, (merit, val)) in character.merits.iter().cloned().enumerate() {
			if let Merit::_Custom(str) = &merit {
				col1 = col1.push(
					text_input("", str)
						.on_input(move |key| Event(i, Merit::_Custom(key), val))
						.padding(INPUT_PADDING),
				);
			} else {
				col1 = col1
					.push(
						pick_list(
							vec.clone(),
							Some::<Translated<Merit>>(merit.clone().into()),
							move |key| Event(i, key.unwrap(), val),
						)
						.padding(INPUT_PADDING)
						.text_size(20)
						.width(Length::Fill),
					)
					.spacing(1);
			}

			col2 = col2.push(SheetDots::new(val, 0, 5, Shape::Dots, None, {
				let merit = merit.clone();
				move |val| Event(i, merit.clone(), val)
			}));
		}

		let new = pick_list(vec, None::<Translated<Merit>>, |key| {
			Event(self.character.borrow().merits.len(), key.unwrap(), 0)
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

impl<'a, Message, Theme> From<MeritComponent<Message>> for Element<'a, Message, Theme>
where
	Message: 'a,
	Theme: text::StyleSheet
		+ dots::StyleSheet
		+ text_input::StyleSheet
		+ pick_list::StyleSheet
		+ scrollable::StyleSheet
		+ overlay::menu::StyleSheet
		+ container::StyleSheet
		+ 'static,
	<Theme as overlay::menu::StyleSheet>::Style: From<<Theme as pick_list::StyleSheet>::Style>,
{
	fn from(info_bar: MeritComponent<Message>) -> Self {
		component(info_bar)
	}
}
