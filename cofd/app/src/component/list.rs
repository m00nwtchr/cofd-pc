use iced::{
	widget::{column, text, Column},
	Alignment,
};
use iced_lazy::Component;
use iced_native::Element;



use crate::{
	fl,
	widget::{
		self,
	}, H3_SIZE,
};

pub struct List<'a, T, Message, Renderer> {
	vec: Vec<T>,
	f: Box<dyn Fn(&T) -> Element<'a, Event<T>, Renderer>>,
	on_change: Box<dyn Fn(usize, T) -> Message>,
}

pub fn list<'a, T, Message, Renderer>(
	vec: Vec<T>,
	f: impl Fn(&T) -> Element<'a, Event<T>, Renderer> + 'static,
	on_change: impl Fn(usize, T) -> Message + 'static,
) -> List<'a, T, Message, Renderer>
where
	Renderer: iced_native::text::Renderer + 'static,
	Renderer::Theme: iced::widget::text::StyleSheet
		+ iced::widget::pick_list::StyleSheet
		+ iced::widget::text_input::StyleSheet
		+ widget::dots::StyleSheet,
{
	List::new(vec, f, on_change)
}

#[derive(Clone)]
pub struct Event<T>(usize, T);

impl<'a, T, Message, Renderer> List<'a, T, Message, Renderer>
where
	Renderer: iced_native::text::Renderer + 'static,
	Renderer::Theme: iced::widget::text::StyleSheet
		+ iced::widget::pick_list::StyleSheet
		+ iced::widget::text_input::StyleSheet
		+ widget::dots::StyleSheet,
{
	fn new(
		vec: Vec<T>,
		f: impl Fn(&T) -> Element<'a, Event<T>, Renderer> + 'static,
		on_change: impl Fn(usize, T) -> Message + 'static,
	) -> Self {
		Self {
			vec,
			f: Box::new(f),
			on_change: Box::new(on_change),
		}
	}
}

impl<'a, T, Message, Renderer> Component<Message, Renderer> for List<'a, T, Message, Renderer>
where
	Renderer: iced_native::text::Renderer + 'static,
	Renderer::Theme: iced::widget::text::StyleSheet
		+ iced::widget::pick_list::StyleSheet
		+ iced::widget::text_input::StyleSheet
		+ widget::dots::StyleSheet,
{
	type State = ();
	type Event = Event<T>;

	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		Some((self.on_change)(event.0, event.1))
	}

	fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
		let mut col = Column::new();

		for el in &self.vec {
			col = col.push((self.f)(el));
		}

		column![text(fl!("merits")).size(H3_SIZE), col]
			.align_items(Alignment::Center)
			.into()
	}
}

impl<'a, T, Message, Renderer> From<List<'a, T, Message, Renderer>>
	for Element<'a, Message, Renderer>
where
	T: 'a,
	Message: 'a,
	Renderer: 'static + iced_native::text::Renderer,
	Renderer::Theme: iced::widget::text::StyleSheet
		+ iced::widget::pick_list::StyleSheet
		+ iced::widget::text_input::StyleSheet
		+ widget::dots::StyleSheet,
{
	fn from(info_bar: List<'a, T, Message, Renderer>) -> Self {
		iced_lazy::component(info_bar)
	}
}
