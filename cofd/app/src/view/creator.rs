use iced::{
	widget::{button, column, pick_list, row, text},
	Length,
};
use iced_lazy::Component;

use cofd::{prelude::*, splat::Splat};

use crate::{i18n::Translated, Element, INPUT_PADDING};

pub struct CreatorView<Message> {
	on_done: Box<dyn Fn(Character) -> Message>,
	splat: Splat,
}

pub fn creator_view<Message>(
	on_done: impl Fn(Character) -> Message + 'static,
) -> CreatorView<Message> {
	CreatorView::new(on_done)
}

#[derive(Clone)]
pub enum Event {
	SplatChanged(Splat),
	Done,
}

impl<Message> CreatorView<Message> {
	pub fn new(on_done: impl Fn(Character) -> Message + 'static) -> Self {
		Self {
			on_done: Box::new(on_done),
			splat: Splat::default(),
		}
	}

	fn splat(&self) -> Element<Event> {
		let splats: Vec<Translated<Splat>> = Splat::all().into_iter().map(Into::into).collect();

		pick_list(splats, Some(self.splat.clone().into()), |val| {
			Event::SplatChanged(val.unwrap())
		})
		.padding(INPUT_PADDING)
		.width(Length::Fill)
		.into()
	}
}

impl<Message> Component<Message, iced::Renderer> for CreatorView<Message> {
	type State = ();

	type Event = Event;

	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		match event {
			Event::SplatChanged(splat) => {
				self.splat = splat;
				None
			}
			Event::Done => Some((self.on_done)(
				Character::builder().with_splat(self.splat.clone()).build(),
			)),
		}
	}

	#[allow(clippy::too_many_lines)]
	fn view(&self, _state: &Self::State) -> Element<Self::Event> {
		column![
			text("Character Creator"),
			row![self.splat()],
			row![
				button("Done").on_press(Event::Done)
			// button("Previous"), button("Next")
			]
		]
		.into()
	}
}

impl<'a, Message> From<CreatorView<Message>> for Element<'a, Message>
where
	Message: 'a,
{
	fn from(creator: CreatorView<Message>) -> Self {
		iced_lazy::component(creator)
	}
}
