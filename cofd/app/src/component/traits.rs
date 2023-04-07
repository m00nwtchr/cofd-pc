use iced::{
	widget::{column, row, text, text_input},
	Length,
};
use iced_lazy::Component;

use cofd::{character::ArmorStruct, prelude::*};

use crate::{fl, i18n::flt, Element, INPUT_PADDING};

struct Traits {
	size: u16,
	speed: u16,
	defense: u16,
	armor: ArmorStruct,
	initative: u16,
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

impl<Message> Component<Message, iced::Renderer> for TraitsComponent<Message> {
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

	fn view(&self, _state: &Self::State) -> Element<Self::Event> {
		let beats = row![
			text(format!("{}:", fl!("beats"))),
			text_input("", &format!("{}", self.traits.beats), |val| {
				// if let Some(val) = val.parse() {
				Event(val, Trait::Beats)
				// }
			})
			.padding(INPUT_PADDING)
		];

		let alternate_beats = if !self.traits.alt_opt {
			row![
				text(format!(
					"{}:",
					flt(&self.traits.splat, Some("beats")).unwrap()
				)),
				text_input("", &format!("{}", self.traits.alternate_beats), |val| {
					Event(val, Trait::AlternateBeats)
				})
				.padding(INPUT_PADDING)
			]
		} else {
			row![]
		};

		let alternate_xp = if !self.traits.alt_opt {
			row![text(format!(
				"{}: {}",
				flt(&self.traits.splat, Some("experience")).unwrap(),
				self.traits.alternate_experience
			)),]
		} else {
			row![]
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
				text(format!("{}: {}", fl!("initative"), self.traits.initative)),
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

impl<'a, Message> From<TraitsComponent<Message>> for Element<'a, Message>
where
	Message: 'a,
{
	fn from(info_bar: TraitsComponent<Message>) -> Self {
		iced_lazy::component(info_bar)
	}
}
