use iced::{
	widget::{column, row, text, text_input},
	Length,
};
use iced_lazy::Component;
use iced_native::Element;

use cofd::{
	character::{ArmorStruct, Trait},
	prelude::Character,
};

use crate::fl;

struct Traits {
	size: u16,
	speed: u16,
	defense: u16,
	armor: ArmorStruct,
	initative: u16,
	beats: u16,
	experience: u16,
}

pub struct TraitsComponent<Message> {
	traits: Traits,
	on_change: Box<dyn Fn(u16, Trait) -> Message>,
}

pub fn traits_component<Message>(
	character: &Character,
	on_change: impl Fn(u16, Trait) -> Message + 'static,
) -> TraitsComponent<Message> {
	TraitsComponent::new(character, on_change)
}

#[derive(Clone)]
pub struct Event(String, Trait);

impl<Message> TraitsComponent<Message> {
	fn new(character: &Character, on_change: impl Fn(u16, Trait) -> Message + 'static) -> Self {
		Self {
			traits: Traits {
				size: character.size(),
				speed: character.speed(),
				defense: character.defense(),
				initative: character.initative(),
				beats: character.beats,
				experience: character.experience(),
				armor: character.armor(),
			},
			on_change: Box::new(on_change),
		}
	}
}

impl<Message, Renderer> Component<Message, Renderer> for TraitsComponent<Message>
where
	Renderer: iced_native::text::Renderer + 'static,
	Renderer::Theme: iced::widget::text::StyleSheet + iced::widget::text_input::StyleSheet,
{
	type State = ();
	type Event = Event;

	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		if let Ok(val) = event.0.parse() {
			Some((self.on_change)(val, event.1))
		} else if event.0.is_empty() {
			Some((self.on_change)(0, event.1))
		} else {
			None
		}
	}

	fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
		column![
			row![
				text(format!("{}: {}", fl!("size"), self.traits.size)),
				// text_input("", &format!("{}", self.traits.size), |val| {
				// 	Event(val, Trait::Size)
				// })
			],
			row![
				text(format!("{}: {}", fl!("speed"), self.traits.speed)),
				// text(self.traits.speed)
			],
			row![
				text(format!("{}: {}", fl!("defense"), self.traits.defense)),
				// text(self.traits.defense)
			],
			row![
				text(format!(
					"{}: {}/{}",
					fl!("armor"),
					self.traits.armor.general,
					self.traits.armor.ballistic
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
			row![
				text(format!("{}: {}", fl!("initative"), self.traits.initative)),
				// text(self.traits.initative)
			],
			row![
				text(format!("{}:", fl!("beats"))),
				text_input("", &format!("{}", self.traits.beats), |val| {
					// if let Some(val) = val.parse() {
					Event(val, Trait::Beats)
					// }
				})
			],
			row![
				text(format!("{}: {}", fl!("experience"), self.traits.experience)),
				// text(self.traits.experience)
			]
		]
		// .padding(0)
		.width(Length::Fill)
		.into()
	}
}

impl<'a, Message, Renderer> From<TraitsComponent<Message>> for Element<'a, Message, Renderer>
where
	Message: 'a,
	Renderer: 'static + iced_native::text::Renderer,
	Renderer::Theme: iced::widget::text::StyleSheet + iced::widget::text_input::StyleSheet,
{
	fn from(info_bar: TraitsComponent<Message>) -> Self {
		iced_lazy::component(info_bar)
	}
}
