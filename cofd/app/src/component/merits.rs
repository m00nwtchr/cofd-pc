use iced::{
	widget::{column, pick_list, row, text, text_input, Column},
	Alignment, Length,
};
use iced_lazy::Component;
use iced_native::Element;

use cofd::splat::{ability::Ability, Merit, SplatType};
use itertools::Itertools;

use crate::{
	fl,
	widget::{
		self,
		dots::{Shape, SheetDots},
	},
	H2_SIZE, H3_SIZE, TITLE_SPACING,
};

pub struct MeritComponent<Message> {
	splat: SplatType,
	merits: Vec<(Merit, u16)>,
	on_change: Box<dyn Fn(usize, Merit, u16) -> Message>,
}

pub fn merit_component<Message>(
	splat: SplatType,
	merits: Vec<(Merit, u16)>,
	on_change: impl Fn(usize, Merit, u16) -> Message + 'static,
) -> MeritComponent<Message> {
	MeritComponent::new(splat, merits, on_change)
}

#[derive(Clone)]
pub struct Event(usize, Merit, u16);

impl<Message> MeritComponent<Message> {
	fn new(
		splat: SplatType,
		merits: Vec<(Merit, u16)>,
		on_change: impl Fn(usize, Merit, u16) -> Message + 'static,
	) -> Self {
		Self {
			splat,
			merits,
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
		+ widget::dots::StyleSheet,
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
			self.splat.name()
		)));
		vec.extend(Merit::get(self.splat));

		vec.push(Merit::_Custom(String::from("Custom")));

		let vec: Vec<Merit> = vec
			.iter()
			.cloned()
			.filter(|e| self.merits.iter().filter(|(merit, _)| *merit == *e).count() == 0)
			.collect();

		for (i, (merit, val)) in self.merits.iter().enumerate() {
			if let Merit::_Custom(str) = merit {
				col1 = col1.push(text_input("", &str, move |key| {
					Event(i, Merit::_Custom(key), val.clone())
				}));
			} else {
				col1 = col1
					.push(
						pick_list(vec.clone(), Some(merit.clone()), move |key| {
							Event(i, key, val.clone())
						})
						.padding(1)
						.text_size(20)
						.width(Length::Fill),
					)
					.spacing(1);
			}

			col2 = col2.push(SheetDots::new(val.clone(), 0, 5, Shape::Dots, None, {
				let key = merit.clone();
				move |val| Event(i, key.clone(), val)
			}));
		}

		let new = pick_list(vec, None, |key| Event(self.merits.len(), key, 0))
			.padding(1)
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
		+ widget::dots::StyleSheet,
{
	fn from(info_bar: MeritComponent<Message>) -> Self {
		iced_lazy::component(info_bar)
	}
}
