use iced::{
	widget::{text, Column},
	Alignment,
};
use iced_lazy::Component;
use iced_native::Pixels;

use crate::{Element, H3_SIZE, TITLE_SPACING};

pub struct List<'a, T, Message> {
	str: String,
	min: usize,
	vec: Vec<T>,
	f: Box<dyn Fn(usize, Option<T>) -> Element<'a, Message>>,
	max_width: f32, // on_change: Box<dyn Fn(usize, T) -> Message>,
}

pub fn list<'a, T, Message>(
	str: String,
	min: usize,
	vec: Vec<T>,
	f: impl Fn(usize, Option<T>) -> Element<'a, Message> + 'static,
	// on_change: impl Fn(usize, T) -> Message + 'static,
) -> List<'a, T, Message> {
	List::new(str, min, vec, f)
}

// #[derive(Clone)]
// pub struct Event<Message>(Message);

impl<'a, T, Message> List<'a, T, Message> {
	fn new(
		str: String,
		min: usize,
		vec: Vec<T>,
		f: impl Fn(usize, Option<T>) -> Element<'a, Message> + 'static,
		// on_change: impl Fn(usize, T) -> Message + 'static,
	) -> Self {
		Self {
			str,
			min,
			vec,
			f: Box::new(f),
			max_width: f32::INFINITY,
			// on_change: Box::new(on_change),
		}
	}

	pub fn max_width(mut self, width: impl Into<Pixels>) -> Self {
		self.max_width = width.into().0;
		self
	}
}

impl<'a, T, Message> Component<Message, iced::Renderer> for List<'a, T, Message>
where
	T: Clone,
{
	type State = ();
	type Event = Message;

	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		Some(event)
	}

	fn view(&self, _state: &Self::State) -> Element<Self::Event> {
		let mut col = Column::new();

		for i in 0..self.min {
			let val = self.vec.get(i).cloned();

			col = col.push((self.f)(i, val));
		}

		Column::new()
			.push(text(self.str.clone()).size(H3_SIZE))
			.push(col)
			.spacing(TITLE_SPACING)
			.align_items(Alignment::Center)
			.max_width(self.max_width)
			.into()
	}
}

impl<'a, T, Message> From<List<'a, T, Message>> for Element<'a, Message>
where
	T: 'a + Clone,
	Message: 'a,
{
	fn from(list: List<'a, T, Message>) -> Self {
		iced_lazy::component(list)
	}
}
