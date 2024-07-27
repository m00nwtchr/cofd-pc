use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use super::overview::vec_changed;
use crate::{Element, H2_SIZE, TITLE_SPACING};
use closure::closure;
use cofd::{character::Weapon, prelude::*};
use iced::widget::{component, Component};
use iced::{
	widget::{column, row, text, text_input},
	Alignment, Length,
};

pub struct EquipmentTab<Message> {
	character: Rc<RefCell<Character>>,
	phantom: PhantomData<Message>,
}

pub fn equipment_tab<Message>(character: Rc<RefCell<Character>>) -> EquipmentTab<Message> {
	EquipmentTab::new(character)
}

#[derive(Clone)]
pub enum Event {
	WeaponChanged(usize, Weapon),
}

impl<Message> EquipmentTab<Message> {
	pub fn new(character: Rc<RefCell<Character>>) -> Self {
		Self {
			character,
			phantom: PhantomData,
		}
	}

	fn abilities(&self, _character: &Character) -> Element<Event> {
		todo!()
	}
}

impl<Message, Theme> Component<Message, Theme> for EquipmentTab<Message>
where
	Message: Clone,
	Theme: text::StyleSheet + text_input::StyleSheet + 'static,
{
	type State = ();

	type Event = Event;

	fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Message> {
		let mut character = self.character.borrow_mut();

		match event {
			Event::WeaponChanged(i, weapon) => {
				if weapon == Default::default() {
					character.weapons.remove(i);
				} else {
					vec_changed(i, weapon, &mut character.weapons);
				}

				None
			}
		}
	}

	#[allow(clippy::too_many_lines)]
	fn view(&self, _state: &Self::State) -> Element<Event, Theme> {
		let character = self.character.borrow();

		let weapons = {
			let mut name = column![text("Weapon/Attack")]
				.width(Length::FillPortion(3))
				.align_items(Alignment::Center)
				.spacing(3);
			let mut pool = column![text("Dice Pool")]
				.width(Length::Fill)
				.align_items(Alignment::Center)
				.spacing(3);
			let mut damage = column![text("Damage")]
				.width(Length::Fill)
				.align_items(Alignment::Center)
				.spacing(3);
			let mut range = column![text("Range")]
				.width(Length::Fill)
				.align_items(Alignment::Center)
				.spacing(3);
			let mut initative = column![text("Initative")]
				.width(Length::Fill)
				.align_items(Alignment::Center)
				.spacing(3);
			let mut size = column![text("Size")]
				.width(Length::Fill)
				.align_items(Alignment::Center)
				.spacing(3);

			let mut vec = character.weapons.clone();
			vec.push(Default::default());

			for (i, weapon) in vec.into_iter().enumerate() {
				name = name.push(text_input("", &weapon.name).on_input(
					closure!(clone weapon, |val| {
						let mut weapon = weapon.clone();
						weapon.name = val;
						Event::WeaponChanged(i, weapon)
					}),
				));
				pool = pool.push(text_input("", &weapon.dice_pool).on_input(
					closure!(clone weapon, |val| {
						let mut weapon = weapon.clone();
						weapon.dice_pool = val;
						Event::WeaponChanged(i, weapon)
					}),
				));
				damage = damage.push(text_input("", &weapon.damage).on_input(
					closure!(clone weapon, |val| {
						let mut weapon = weapon.clone();
						weapon.damage = val;
						Event::WeaponChanged(i, weapon)
					}),
				));
				range = range.push(text_input("", &weapon.range).on_input(
					closure!(clone weapon, |val| {
						let mut weapon = weapon.clone();
						weapon.range = val;
						Event::WeaponChanged(i, weapon)
					}),
				));
				initative = initative.push(text_input("", &weapon.initative.to_string()).on_input(
					closure!(clone weapon, |val| {
						let mut weapon = weapon.clone();
						if let Ok(val) = val.parse() {
							weapon.initative = val;
						}
						Event::WeaponChanged(i, weapon)
					}),
				));
				size = size.push(text_input("", &weapon.size.to_string()).on_input(
					closure!(clone weapon, |val| {
						let mut weapon = weapon.clone();
						if let Ok(val) = val.parse() {
							weapon.size = val;
						}
						Event::WeaponChanged(i, weapon)
					}),
				));
			}

			column![
				text("Combat").size(H2_SIZE),
				row![name, pool, damage, range, initative, size]
					.spacing(5)
					.padding(5)
			]
			.align_items(Alignment::Center)
			.spacing(TITLE_SPACING)
		};

		column![weapons].align_items(Alignment::Center).into()
	}
}

impl<'a, Message> From<EquipmentTab<Message>> for Element<'a, Message>
where
	Message: 'a + Clone,
{
	fn from(equipment_tab: EquipmentTab<Message>) -> Self {
		component(equipment_tab)
	}
}
