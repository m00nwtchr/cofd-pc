use iced::{
	widget::{column, text, Column},
	Alignment,
};
use iced_lazy::Component;
use iced_native::Element;

use crate::{fl, widget, H3_SIZE, TITLE_SPACING};

pub struct List<'a, T, Message, Renderer> {
	str: String,
	min: usize,
	vec: Vec<T>,
	f: Box<dyn Fn(usize, T) -> Element<'a, Message, Renderer>>,
	max_width: Option<u32>, // on_change: Box<dyn Fn(usize, T) -> Message>,
}

pub fn list<'a, T, Message, Renderer>(
	str: String,
	min: usize,
	vec: Vec<T>,
	f: impl Fn(usize, T) -> Element<'a, Message, Renderer> + 'static,
	// on_change: impl Fn(usize, T) -> Message + 'static,
) -> List<'a, T, Message, Renderer>
where
	Renderer: iced_native::text::Renderer + 'static,
	Renderer::Theme: iced::widget::text::StyleSheet + iced::widget::text_input::StyleSheet,
{
	List::new(str, min, vec, f)
}

// #[derive(Clone)]
// pub struct Event<Message>(Message);

impl<'a, T, Message, Renderer> List<'a, T, Message, Renderer>
where
	Renderer: iced_native::text::Renderer + 'static,
	Renderer::Theme: iced::widget::text::StyleSheet + iced::widget::text_input::StyleSheet,
{
	fn new(
		str: String,
		min: usize,
		vec: Vec<T>,
		f: impl Fn(usize, T) -> Element<'a, Message, Renderer> + 'static,
		// on_change: impl Fn(usize, T) -> Message + 'static,
	) -> Self {
		Self {
			str,
			min,
			vec,
			f: Box::new(f),
			max_width: None,
			// on_change: Box::new(on_change),
		}
	}

	pub fn max_width(mut self, width: u32) -> Self {
		self.max_width = Some(width);
		self
	}
}

impl<'a, T, Message, Renderer> Component<Message, Renderer> for List<'a, T, Message, Renderer>
where
	Renderer: iced_native::text::Renderer + 'static,
	Renderer::Theme: iced::widget::text::StyleSheet + iced::widget::text_input::StyleSheet,
	T: Clone + Default,
{
	type State = ();
	type Event = Message;

	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		Some(event)
	}

	fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
		let mut col = Column::new();

		for i in 0..self.min {
			let val = self.vec.get(i).cloned().unwrap_or_default();

			col = col.push((self.f)(i, val));
		}

		let mut col = Column::new()
			.push(text(self.str.clone()).size(H3_SIZE))
			.push(col)
			.spacing(TITLE_SPACING)
			.align_items(Alignment::Center);

		if let Some(max_width) = self.max_width {
			col = col.max_width(max_width);
		}

		col.into()
	}
}

impl<'a, T, Message, Renderer> From<List<'a, T, Message, Renderer>>
	for Element<'a, Message, Renderer>
where
	T: 'a + Clone + Default,
	Message: 'a,
	Renderer: 'static + iced_native::text::Renderer,
	Renderer::Theme: iced::widget::text::StyleSheet + iced::widget::text_input::StyleSheet,
{
	fn from(list: List<'a, T, Message, Renderer>) -> Self {
		iced_lazy::component(list)
	}
}
