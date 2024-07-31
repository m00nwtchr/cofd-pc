use std::cell::{Ref, RefMut};

use super::*;
use cofd::character::Character;
use iced::widget::{button, column, row, Column, Component};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub enum Tab {
	Overview(overview::OverviewTab),
	Equipment(equipment::EquipmentTab),
	SplatExtras(splat_extras::SplatExtrasTab),
}

#[derive(Debug, Clone)]
pub struct SheetView {
	tab: Tab,
}

#[derive(Clone)]
pub enum Message {
	OverviewTab(overview::Message),
	EquipmentTab(equipment::Message),
	SplatExtras(splat_extras::Message),

	Back,
	Save,

	SelectOverview,
	SelectEquipment,
	SelectSplatExtras,
}

impl SheetView {
	pub fn new() -> Self {
		Self {
			tab: Tab::Overview(overview::OverviewTab::new()),
		}
	}

	pub fn update(&mut self, message: Message, character: &mut Character) {
		match message {
			Message::OverviewTab(message) => {
				if let Tab::Overview(view) = &mut self.tab {
					view.update(message, character);
				}
			}
			Message::EquipmentTab(message) => {
				if let Tab::Equipment(view) = &mut self.tab {
					view.update(message, character);
				}
			}
			Message::SplatExtras(message) => {
				if let Tab::SplatExtras(view) = &mut self.tab {
					view.update(message, character);
				}
			}

			Message::SelectOverview => self.tab = Tab::Overview(overview::OverviewTab::new()),
			Message::SelectEquipment => self.tab = Tab::Equipment(equipment::EquipmentTab::new()),
			Message::SelectSplatExtras => {
				self.tab = Tab::SplatExtras(splat_extras::SplatExtrasTab::new());
			}
			_ => {}
		}
	}

	#[allow(clippy::too_many_lines)]
	pub fn view(&self, character: &Character) -> Element<Message> {
		let tab: Element<Message> = match &self.tab {
			Tab::Overview(view) => view.view(character).map(Message::OverviewTab),
			Tab::Equipment(view) => view.view(character).map(Message::EquipmentTab),
			Tab::SplatExtras(view) => view.view(character).map(Message::SplatExtras),
		};

		column![
			row![
				button("Back").on_press(Message::Back),
				button("Save").on_press(Message::Save),
				button("Home").on_press(Message::SelectOverview),
				button("Equipment").on_press(Message::SelectEquipment),
				button("Splat").on_press(Message::SelectSplatExtras),
			],
			tab
		]
		.width(Length::Fill)
		.spacing(1)
		.into()
	}
}
