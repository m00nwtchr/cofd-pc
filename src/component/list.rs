use iced::widget::{component, Component};
use iced::{
	widget::{text, Column},
	Alignment, Element, Pixels,
};

use crate::{H3_SIZE, TITLE_SPACING};

pub struct List<'a, T, Message, Theme> {
	str: String,
	min: Option<usize>,
	max: Option<usize>,
	vec: Vec<T>,
	f: Box<dyn Fn(usize, Option<T>) -> Element<'a, Message, Theme>>,
	max_width: f32, // on_change: Box<dyn Fn(usize, T) -> Message>,
}

pub fn list<'a, T, Message, Theme>(
	str: String,
	min: Option<usize>,
	max: Option<usize>,
	vec: Vec<T>,
	f: impl Fn(usize, Option<T>) -> Element<'a, Message, Theme> + 'static,
	// on_change: impl Fn(usize, T) -> Message + 'static,
) -> List<'a, T, Message, Theme> {
	List::new(str, min, max, vec, f)
}

impl<'a, T, Message, Theme> List<'a, T, Message, Theme> {
	fn new(
		str: String,
		min: Option<usize>,
		max: Option<usize>,
		vec: Vec<T>,
		f: impl Fn(usize, Option<T>) -> Element<'a, Message, Theme> + 'static,
		// on_change: impl Fn(usize, T) -> Message + 'static,
	) -> Self {
		Self {
			str,
			min,
			max,
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

impl<'a, T, Message, Theme> Component<Message, Theme> for List<'a, T, Message, Theme>
where
	T: Clone,
	Theme: text::StyleSheet,
{
	type State = ();
	type Event = Message;

	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		Some(event)
	}

	fn view(&self, _state: &Self::State) -> Element<'_, Message, Theme> {
		let mut col = Column::<'_, Message, Theme>::new();

		for i in 0..std::cmp::min(
			self.max.unwrap_or(usize::MAX),
			std::cmp::max(self.min.unwrap_or(0), self.vec.len()),
		) {
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

impl<'a, T, Message, Theme> From<List<'a, T, Message, Theme>> for Element<'a, Message, Theme>
where
	T: 'a + Clone,
	Message: 'a,
	Theme: text::StyleSheet + 'static,
{
	fn from(list: List<'a, T, Message, Theme>) -> Self {
		component(list)
	}
}
