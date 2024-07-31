use closure::closure;
use cofd::{character::Weapon, prelude::*};
use iced::{
	widget::{column, row, text, text_input},
	Alignment, Length,
};

use super::overview::vec_changed;
use crate::{fl, Element, H2_SIZE, TITLE_SPACING};

#[derive(Debug, Clone)]
pub struct EquipmentTab;

#[derive(Debug, Clone)]
pub enum Message {
	WeaponChanged(usize, Weapon),
}

impl EquipmentTab {
	pub fn new() -> Self {
		Self
	}

	pub fn update(&mut self, event: Message, character: &mut Character) {
		match event {
			Message::WeaponChanged(i, weapon) => {
				if weapon == Default::default() {
					character.weapons.remove(i);
				} else {
					vec_changed(i, weapon, &mut character.weapons);
				}
			}
		}
	}

	#[allow(clippy::too_many_lines)]
	pub fn view(&self, character: &Character) -> Element<Message> {
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
			let mut initative = column![text(fl!("initiative"))]
				.width(Length::Fill)
				.align_items(Alignment::Center)
				.spacing(3);
			let mut size = column![text(fl!("size"))]
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
						Message::WeaponChanged(i, weapon)
					}),
				));
				pool = pool.push(text_input("", &weapon.dice_pool).on_input(
					closure!(clone weapon, |val| {
						let mut weapon = weapon.clone();
						weapon.dice_pool = val;
						Message::WeaponChanged(i, weapon)
					}),
				));
				damage = damage.push(text_input("", &weapon.damage).on_input(
					closure!(clone weapon, |val| {
						let mut weapon = weapon.clone();
						weapon.damage = val;
						Message::WeaponChanged(i, weapon)
					}),
				));
				range = range.push(text_input("", &weapon.range).on_input(
					closure!(clone weapon, |val| {
						let mut weapon = weapon.clone();
						weapon.range = val;
						Message::WeaponChanged(i, weapon)
					}),
				));
				initative = initative.push(text_input("", &weapon.initative.to_string()).on_input(
					closure!(clone weapon, |val| {
						let mut weapon = weapon.clone();
						if let Ok(val) = val.parse() {
							weapon.initative = val;
						}
						Message::WeaponChanged(i, weapon)
					}),
				));
				size = size.push(text_input("", &weapon.size.to_string()).on_input(
					closure!(clone weapon, |val| {
						let mut weapon = weapon.clone();
						if let Ok(val) = val.parse() {
							weapon.size = val;
						}
						Message::WeaponChanged(i, weapon)
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
