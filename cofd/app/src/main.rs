use std::{cell::RefCell, rc::Rc};

use i18n_embed::fluent::FluentLanguageLoader;
use iced::{
	alignment::Vertical,
	executor,
	widget::{column, container, row, text},
	widget::{Column, Row},
	Alignment, Application, Command, Element, Sandbox, Settings, Theme, Length,
};
use iced_aw::pure::{TabLabel, Tabs};

use i18n_embed_fl::fl;

use cofd::{
	character::{AttributeCategory, TraitCategory},
	prelude::*,
};

mod i18n;
mod widget;

use i18n::LANGUAGE_LOADER;
use widget::SheetDots;

struct PlayerCompanionApp {
	active_tab: usize,
	character: Character,
	// lang_loader: FluentLanguageLoader,
}

#[derive(Debug, Clone, Copy)]
enum Message {
	TabSelected(usize),
	AttrChanged(u8, Attribute),
}

pub fn fl(loader: &FluentLanguageLoader, message_id: &str, attribute: &str) -> String {
	let message = Rc::new(RefCell::new(None));
	let message_clone = message.clone();
	loader.with_bundles_mut(|bundle| {
		let pattern = bundle
			.get_message(message_id)
			.unwrap()
			.get_attribute(attribute)
			.unwrap()
			.value();

		message.replace(Some(
			bundle
				.format_pattern(pattern, None, &mut vec![])
				.to_string(),
		));
	});
	message_clone.take().unwrap()
}

impl PlayerCompanionApp {
	fn overview_tab(&self) -> Element<'static, Message> {
		column![self.attributes()]
			.padding(10)
			.width(Length::Fill)
			.align_items(Alignment::Center)
			// .align_y(Vertical::Center)
			.into()
	}

	fn attributes(&self) -> Element<'static, Message> {
		fn mk_col(app: &PlayerCompanionApp, cat: TraitCategory) -> Element<'static, Message> {
			let mut col1 = Column::new().spacing(3);
			let mut col2 = Column::new().spacing(3);
	
			for attr in Attribute::get(AttributeCategory::Trait(cat)) {
				col1 = col1.push(text(format!(
					"{}",
					fl(&LANGUAGE_LOADER, "attribute", &attr.name())
				)));
	
				let v = app.character.base_attributes().get(&attr) as u8;
				col2 = col2.push(SheetDots::new(v, 1, 5, |val| Message::AttrChanged(val, attr)));
			}
	
			row![col1, col2].spacing(5).into()
		}

		row![
			mk_col(self, TraitCategory::Mental),
			mk_col(self, TraitCategory::Physical),
			mk_col(self, TraitCategory::Social)
		].spacing(10)
		.into()
	}
}

impl Application for PlayerCompanionApp {
	type Executor = executor::Default;
	type Flags = ();
	type Message = Message;
	type Theme = Theme;

	fn new(_flags: ()) -> (Self, Command<Self::Message>) {
		// let lang_loader = i18n::setup();

		// println!("{}", fl!(lang_loader, "attribute.wits"));

		let character = Character::builder()
			.with_attributes(Attributes {
				intelligence: 1,
				wits: 3,
				resolve: 2,
				//
				strength: 3,
				dexterity: 2,
				stamina: 3,
				//
				presence: 3,
				manipulation: 1,
				composure: 3,
			})
			.build();

		(
			Self {
				active_tab: 0,
				character,
				// lang_loader,
			},
			Command::none(),
		)
	}

	fn title(&self) -> String {
		"Chronicles of Darkness Player Companion".to_string()
	}

	fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
		match message {
			Message::TabSelected(tab) => self.active_tab = tab,
			Message::AttrChanged(val, attr) => {
				*self.character.base_attributes_mut().get_mut(&attr) = val as i8;
			}
		}

		Command::none()
	}

	fn view(&self) -> Element<'_, Self::Message> {
		self.overview_tab().into()
		// Tabs::new(self.active_tab, Message::TabSelected)
		// 	.push(
		// 		TabLabel::Text(String::from("Overview")),
		// 		self.overview_tab(),
		// 	)
		// 	.push(TabLabel::Text("UwU".to_string()), self.overview_tab())
		// 	.into()
	}
}

fn main() -> iced::Result {
	i18n::setup();

	PlayerCompanionApp::run(Settings {
		..Default::default()
	})
}
