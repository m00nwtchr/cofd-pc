use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use iced::{
	widget::{column, pick_list, row, text, text_input, Column},
	Alignment, Element, Length,
};
use iced_lazy::Component;

use cofd::{
	character::{ModifierTarget, Trait, Wound},
	prelude::*,
	splat::{ability::Ability, changeling::Regalia, Splat, SplatType},
};

use crate::{
	component::{
		attributes::attribute_bar, info::info_bar, merits::merit_component,
		skills::skills_component, traits::traits_component,
	},
	fl,
	// i18n::fl,
	widget::{self, dots::Shape, dots::SheetDots, track::HealthTrack},
	H2_SIZE,
	H3_SIZE,
};

pub struct EquipmentTab<Message> {
	character: Rc<RefCell<Character>>,
	phantom: PhantomData<Message>,
}

pub fn equipment_tab<Message>(character: Rc<RefCell<Character>>) -> EquipmentTab<Message> {
	EquipmentTab::new(character)
}

#[derive(Clone)]
pub enum Event {}

impl<Message> EquipmentTab<Message> {
	pub fn new(character: Rc<RefCell<Character>>) -> Self {
		Self {
			character,
			phantom: PhantomData,
		}
	}

	fn abilities<Renderer>(&self, character: &Character) -> Element<Event, Renderer>
	where
		Renderer: iced_native::text::Renderer + 'static,
		Renderer::Theme: iced::widget::text::StyleSheet
			+ widget::dots::StyleSheet
			+ iced::widget::text_input::StyleSheet
			+ iced::widget::pick_list::StyleSheet,
	{
		todo!()
	}
}

impl<Message, Renderer> Component<Message, Renderer> for EquipmentTab<Message>
where
	Message: Clone,
	Renderer: iced_native::text::Renderer + 'static,
	Renderer::Theme: iced::widget::pick_list::StyleSheet
		+ iced::widget::text_input::StyleSheet
		+ iced::widget::text::StyleSheet
		+ iced::widget::button::StyleSheet
		+ widget::dots::StyleSheet
		+ widget::track::StyleSheet,
{
	type State = ();

	type Event = Event;

	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		let mut character = self.character.borrow_mut();

		let mut res = None;

		match event {}

		res
	}

	#[allow(clippy::too_many_lines)]
	fn view(&self, _state: &Self::State) -> iced_native::Element<Self::Event, Renderer> {
		let character = self.character.borrow();

		column![].into()
	}
}

impl<'a, Message, Renderer> From<EquipmentTab<Message>> for Element<'a, Message, Renderer>
where
	Message: 'a + Clone,
	Renderer: 'static + iced_native::text::Renderer,
	Renderer::Theme: iced::widget::pick_list::StyleSheet
		+ iced::widget::text_input::StyleSheet
		+ iced::widget::text::StyleSheet
		+ iced::widget::button::StyleSheet
		+ widget::dots::StyleSheet
		+ widget::track::StyleSheet,
{
	fn from(equipment_tab: EquipmentTab<Message>) -> Self {
		iced_lazy::component(equipment_tab)
	}
}
