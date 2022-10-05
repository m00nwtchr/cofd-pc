use iced::{
	widget::{column, pick_list, row, text, text_input, Column},
	Alignment, Length,
};
use iced_lazy::Component;
use iced_native::Element;

use cofd::splat::{
	ability::{Ability, AbilityVal},
	Merit,
};

use crate::{
	fl,
	widget::{
		self,
		dots::{Shape, SheetDots},
	},
	H2_SIZE, H3_SIZE,
};

pub struct MeritComponent<Message> {
	merits: Vec<AbilityVal>,
	on_change: Box<dyn Fn(usize, AbilityVal) -> Message>,
}

pub fn merit_component<Message>(
	merits: Vec<AbilityVal>,
	on_change: impl Fn(usize, AbilityVal) -> Message + 'static,
) -> MeritComponent<Message> {
	MeritComponent::new(merits, on_change)
}

#[derive(Clone)]
pub struct Event(usize, AbilityVal);

impl<Message> MeritComponent<Message> {
	fn new(
		merits: Vec<AbilityVal>,
		on_change: impl Fn(usize, AbilityVal) -> Message + 'static,
	) -> Self {
		Self {
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
		Some((self.on_change)(event.0, event.1))
	}

	fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
		let mut col1 = Column::new().spacing(3).width(Length::Fill);
		let mut col2 = Column::new()
			.spacing(4)
			.width(Length::Fill)
			.align_items(Alignment::End);

		for (i, ability) in self.merits.iter().enumerate() {
			let merit = ability.0.clone();
			let val = ability.1;

			if let Ability::Merit(merit) = merit {
				if let Merit::_Custom(str) = merit {
					col1 = col1.push(text_input("", &str, move |key| {
						Event(i, AbilityVal(Ability::Merit(Merit::_Custom(key)), val))
					}));
				} else {
					let mut e = Vec::new();
					// let mut e: Vec<Ability> = Merit::all()
					// 	.unwrap()
					// 	.iter()
					// 	.filter(|e| !self.merits.contains(e))
					// 	.cloned()
					// 	.collect();

					e.push(Merit::custom(String::from("Custom")));

					col1 = col1
						.push(
							pick_list(e, Some(merit.clone()), move |key| {
								Event(i, AbilityVal(Ability::Merit(key), val))
							})
							.padding(1)
							.text_size(20)
							.width(Length::Fill),
						)
						.spacing(1);
				}
			}

			col2 = col2.push(SheetDots::new(ability.1, 0, 5, Shape::Dots, None, {
				let key = ability.0.clone();
				move |val| Event(i, AbilityVal(key.clone(), val))
			}));
		}

		column![text(fl!("merits")).size(H3_SIZE), column![row![col1, col2]]]
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
