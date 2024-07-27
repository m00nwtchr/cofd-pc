use crate::{fl, i18n::flt, Element, INPUT_PADDING};
use cofd::{character::ArmorStruct, prelude::*};
use iced::widget::{component, Component};
use iced::{
	widget::{column, row, text, text_input},
	Length,
};

struct Traits {
	size: u16,
	speed: u16,
	defense: u16,
	armor: ArmorStruct,
	initiative: u16,
	beats: u16,
	alternate_beats: u16,
	alternate_experience: u16,
	experience: u16,

	alt_opt: bool,
	splat: String,
	// alt_name: Option<String>,
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
pub struct Event(u16, Trait);

impl<Message> TraitsComponent<Message> {
	fn new(character: &Character, on_change: impl Fn(u16, Trait) -> Message + 'static) -> Self {
		Self {
			traits: Traits {
				size: character.size(),
				speed: character.speed(),
				defense: character.defense(),
				initiative: character.initative(),
				beats: character.beats,
				experience: character.experience(),
				alternate_beats: character.alternate_beats,
				alternate_experience: character.alternate_experience(),
				armor: character.armor(),

				alt_opt: character.splat.alternate_beats_optional(),
				splat: character.splat.name().to_string(),
				// alt_name: character.splat.alternate_beats().map(|el| el.to_string()),
			},
			on_change: Box::new(on_change),
		}
	}
}

impl<Message, Theme> Component<Message, Theme> for TraitsComponent<Message>
where
	Theme: text::StyleSheet + text_input::StyleSheet + 'static,
{
	type State = ();
	type Event = Event;

	fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Message> {
		Some((self.on_change)(event.0, event.1))
	}

	fn view(&self, _state: &Self::State) -> Element<Event, Theme> {
		let beats = row![
			text(format!("{}:", fl!("beats"))),
			text_input("", &format!("{}", self.traits.beats))
				.on_input(|val| { Event(val.parse().unwrap_or(0), Trait::Beats) })
				.padding(INPUT_PADDING)
		];

		let alternate_beats = if self.traits.alt_opt {
			row![]
		} else {
			row![
				text(format!(
					"{}:",
					flt(&self.traits.splat, Some("beats")).unwrap()
				)),
				text_input("", &format!("{}", self.traits.alternate_beats))
					.on_input(|val| { Event(val.parse().unwrap_or(0), Trait::AlternateBeats) })
					.padding(INPUT_PADDING)
			]
		};

		let alternate_xp = if self.traits.alt_opt {
			row![]
		} else {
			row![text(format!(
				"{}: {}",
				flt(&self.traits.splat, Some("experience")).unwrap(),
				self.traits.alternate_experience
			)),]
		};

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
				text(format!("{}: {}", fl!("initative"), self.traits.initiative)),
				// text(self.traits.initative)
			],
			beats,
			row![
				text(format!("{}: {}", fl!("experience"), self.traits.experience)),
				// text(self.traits.experience)
			],
			alternate_beats,
			alternate_xp
		]
		// .padding(0)
		.width(Length::Fill)
		.into()
	}
}

impl<'a, Message, Theme> From<TraitsComponent<Message>> for Element<'a, Message, Theme>
where
	Message: 'a,
	Theme: text::StyleSheet + text_input::StyleSheet + 'static,
{
	fn from(traits_component: TraitsComponent<Message>) -> Self {
		component(traits_component)
	}
}
