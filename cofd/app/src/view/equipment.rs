use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use iced::widget::column;
use iced_lazy::Component;

use cofd::prelude::*;

use crate::Element;

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

	fn abilities(&self, character: &Character) -> Element<Event> {
		todo!()
	}
}

impl<Message> Component<Message, iced::Renderer> for EquipmentTab<Message>
where
	Message: Clone,
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
	fn view(&self, _state: &Self::State) -> Element<Self::Event> {
		let character = self.character.borrow();

		column![].into()
	}
}

impl<'a, Message> From<EquipmentTab<Message>> for Element<'a, Message>
where
	Message: 'a + Clone,
{
	fn from(equipment_tab: EquipmentTab<Message>) -> Self {
		iced_lazy::component(equipment_tab)
	}
}
