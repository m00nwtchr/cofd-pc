use std::{cell::RefCell, rc::Rc};

use iced::{
	widget::{column, pick_list, row, text, text_input, Column},
	Alignment, Length,
};
use iced_lazy::Component;
use iced_native::Element;

use cofd::{prelude::Character, splat::Merit};

use crate::{
	fl,
	i18n::Translated,
	widget::{
		self,
		dots::{Shape, SheetDots},
	},
	H3_SIZE, INPUT_PADDING, TITLE_SPACING,
};

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

impl<Message, Renderer> Component<Message, Renderer> for MeritComponent<Message>
where
	Renderer: iced_native::text::Renderer + 'static,
	Renderer::Theme: iced::widget::text::StyleSheet
		+ iced::widget::pick_list::StyleSheet
		+ iced::widget::text_input::StyleSheet
		+ widget::dots::StyleSheet
		+ iced::widget::scrollable::StyleSheet
		+ iced::overlay::menu::StyleSheet
		+ iced::widget::container::StyleSheet,
	<<Renderer as iced_native::Renderer>::Theme as iced::overlay::menu::StyleSheet>::Style: From<
		<<Renderer as iced_native::Renderer>::Theme as iced::widget::pick_list::StyleSheet>::Style,
	>,
{
	type State = ();
	type Event = Event;

	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		if let Merit::_Custom(str) = &event.1 {
			if str.contains("---") {
				return None;
			}
		}

		Some((self.on_change)(event.0, event.1, event.2))
	}

	fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
		let character = self.character.borrow();

		let mut col1 = Column::new().spacing(3).width(Length::Fill);
		let mut col2 = Column::new()
			.spacing(4)
			.width(Length::Fill)
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
			character.splat.name()
		)));
		vec.extend(character.splat.merits());

		vec.push(Merit::_Custom(fl!("custom")));

		let vec: Vec<Translated<Merit>> = vec
			.iter()
			.cloned()
			.filter(|e| {
				character
					.merits
					.iter()
					.filter(|(merit, _)| *merit == *e)
					.count() == 0 && e.is_available(&character)
			})
			.map(Into::into)
			.collect();

		for (i, (merit, val)) in character.merits.iter().cloned().enumerate() {
			if let Merit::_Custom(str) = &merit {
				col1 = col1.push(
					text_input("", str, move |key| {
						Event(i, Merit::_Custom(key), val.clone())
					})
					.padding(INPUT_PADDING),
				);
			} else {
				col1 = col1
					.push(
						pick_list(vec.clone(), Some(merit.clone().into()), move |key| {
							Event(i, key.unwrap().clone(), val.clone())
						})
						.padding(INPUT_PADDING)
						.text_size(20)
						.width(Length::Fill),
					)
					.spacing(1);
			}

			col2 = col2.push(SheetDots::new(val.clone(), 0, 5, Shape::Dots, None, {
				let merit = merit.clone();
				move |val| Event(i, merit.clone(), val)
			}));
		}

		let new = pick_list(vec, None, |key| {
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

impl<'a, Message, Renderer> From<MeritComponent<Message>> for Element<'a, Message, Renderer>
where
	Message: 'a,
	Renderer: 'static + iced_native::text::Renderer,
	Renderer::Theme: iced::widget::text::StyleSheet
		+ iced::widget::pick_list::StyleSheet
		+ iced::widget::text_input::StyleSheet
		+ widget::dots::StyleSheet
		+ iced::widget::scrollable::StyleSheet
		+ iced::overlay::menu::StyleSheet
		+ iced::widget::container::StyleSheet,
	<<Renderer as iced_native::Renderer>::Theme as iced::overlay::menu::StyleSheet>::Style: From<
		<<Renderer as iced_native::Renderer>::Theme as iced::widget::pick_list::StyleSheet>::Style,
	>,
{
	fn from(info_bar: MeritComponent<Message>) -> Self {
		iced_lazy::component(info_bar)
	}
}
