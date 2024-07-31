use crate::i18n::Translated;
use crate::{i18n::Translate, INPUT_PADDING};
use cofd::splat::SplatKind;
use cofd::{prelude::*, splat::Splat};
use iced::overlay::menu;
use iced::widget::{container, scrollable};
use iced::{
	widget::{button, column, component, pick_list, row, text, Component},
	Element, Length,
};

pub struct CreatorView<Message> {
	on_done: Box<dyn Fn(Character) -> Message>,
	splat: SplatKind,
}

pub fn creator_view<Message>(
	on_done: impl Fn(Character) -> Message + 'static,
) -> CreatorView<Message> {
	CreatorView::new(on_done)
}

#[derive(Clone)]
pub enum Event {
	SplatChanged(SplatKind),
	Done,
}

impl<Message> CreatorView<Message> {
	pub fn new(on_done: impl Fn(Character) -> Message + 'static) -> Self {
		Self {
			on_done: Box::new(on_done),
			splat: SplatKind::Mortal,
		}
	}

	fn splat<Theme>(&self) -> Element<Event, Theme>
	where
		Theme: pick_list::StyleSheet
			+ scrollable::StyleSheet
			+ menu::StyleSheet
			+ container::StyleSheet
			+ 'static,
		<Theme as menu::StyleSheet>::Style: From<<Theme as pick_list::StyleSheet>::Style>,
	{
		let splats: Vec<Translated<SplatKind>> = SplatKind::all()
			.into_iter()
			.copied()
			.map(Into::into)
			.collect();

		let splat: Translated<SplatKind> = self.splat.clone().into();
		pick_list(splats, Some(splat), |val| Event::SplatChanged(val.unwrap()))
			.padding(INPUT_PADDING)
			.width(Length::Fill)
			.into()
	}
}

impl<Message, Theme> Component<Message, Theme> for CreatorView<Message>
where
	Theme: button::StyleSheet + text::StyleSheet + menu::StyleSheet + 'static,
	Theme:
		pick_list::StyleSheet + scrollable::StyleSheet + menu::StyleSheet + container::StyleSheet,
	<Theme as menu::StyleSheet>::Style: From<<Theme as pick_list::StyleSheet>::Style>,
{
	type State = ();
	type Event = Event;

	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		match event {
			Event::SplatChanged(splat) => {
				self.splat = splat;
				None
			}
			Event::Done => Some((self.on_done)(
				Character::builder().with_splat(self.splat).build(),
			)),
		}
	}

	fn view(&self, _state: &Self::State) -> Element<'_, Event, Theme> {
		column![
			text("Character Creator"),
			row![self.splat()],
			row![
				button("Done").on_press(Event::Done), // button("Previous"), button("Next")
			]
		]
		.into()
	}
}

impl<'a, Message, Theme> From<CreatorView<Message>> for Element<'a, Message, Theme>
where
	Theme: button::StyleSheet + text::StyleSheet + menu::StyleSheet + 'static,
	Theme:
		pick_list::StyleSheet + scrollable::StyleSheet + menu::StyleSheet + container::StyleSheet,
	<Theme as menu::StyleSheet>::Style: From<<Theme as pick_list::StyleSheet>::Style>,
	Message: 'a,
{
	fn from(creator: CreatorView<Message>) -> Self {
		component(creator)
	}
}
