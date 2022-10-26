use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use iced::{
	alignment,
	widget::{button, column, text, Row},
	Element, Length,
};
use iced_lazy::Component;

use crate::fl;
use cofd::{
	prelude::*,
	splat::{werewolf::Form, Splat},
};

pub struct FormTab<Message> {
	character: Rc<RefCell<Character>>,
	on_change: Message,
}

pub fn form_tab<Message>(
	character: Rc<RefCell<Character>>,
	on_change: Message,
) -> FormTab<Message> {
	FormTab::new(character, on_change)
}

#[derive(Clone)]
pub enum Event {
	FormChanged(Form),
}

impl<Message> FormTab<Message> {
	pub fn new(character: Rc<RefCell<Character>>, on_change: Message) -> Self {
		Self {
			character,
			on_change,
		}
	}

	fn mk_col<Renderer>(&self, form: Form, character: &Character) -> Element<Event, Renderer>
	where
		Renderer: iced_native::text::Renderer + 'static,
		Renderer::Theme: iced::widget::text::StyleSheet
			+ iced::widget::text_input::StyleSheet
			+ iced::widget::button::StyleSheet,
	{
		column![button(
			text(fl("werewolf", Some(form.name())).unwrap())
				.width(Length::Fill)
				.horizontal_alignment(alignment::Horizontal::Center)
		)
		.on_press(Event::FormChanged(form))
		.width(Length::Fill)]
		.width(Length::Fill)
		.into()
	}
}

impl<Message, Renderer> Component<Message, Renderer> for FormTab<Message>
where
	Message: Clone,
	Renderer: iced_native::text::Renderer + 'static,
	Renderer::Theme: iced::widget::text_input::StyleSheet
		+ iced::widget::text::StyleSheet
		+ iced::widget::button::StyleSheet,
{
	type State = ();

	type Event = Event;

	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		let mut character = self.character.borrow_mut();

		if let Splat::Werewolf(_, _, _, data) = &mut character.splat {
			match event {
				Event::FormChanged(form) => {
					data.form = form;
					character.calc_mod_map();
					Some(self.on_change.clone())
				}
			}
		} else {
			None
		}
	}

	fn view(&self, _state: &Self::State) -> iced_native::Element<Self::Event, Renderer> {
		let character = self.character.borrow();

		let mut row = Row::new();

		for form in Form::all() {
			row = row.push(self.mk_col(form, &character));
		}

		row.into()
	}
}

impl<'a, Message, Renderer> From<FormTab<Message>> for Element<'a, Message, Renderer>
where
	Message: 'a + Clone,
	Renderer: 'static + iced_native::text::Renderer,
	Renderer::Theme: iced::widget::text_input::StyleSheet
		+ iced::widget::text::StyleSheet
		+ iced::widget::button::StyleSheet,
{
	fn from(form_tab: FormTab<Message>) -> Self {
		iced_lazy::component(form_tab)
	}
}
