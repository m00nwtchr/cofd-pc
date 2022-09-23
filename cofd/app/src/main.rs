use iced::{
	pure::{
		widget::{Column, Row, Text},
		Element, Sandbox,
	},
	Alignment, Settings,
};
use iced_aw::pure::Tabs;

use i18n_embed_fl::fl;

use cofd::{prelude::*, character::{TraitCategory, AttributeCategory}};

mod i18n;

struct PlayerCompanionApp {
	active_tab: usize,
	character: Character,
	// locale: Locale
}

#[derive(Debug, Clone, Copy)]
enum Message {
	TabSelected(usize),
}

impl Sandbox for PlayerCompanionApp {
	type Message = Message;

	fn new() -> Self {
		Self {
			active_tab: 0,
			character: Default::default(),
			// locale: Locale::En_us
		}
	}

	fn title(&self) -> String {
		"Chronicles of Darkness Player Companion".to_string()
	}

	fn update(&mut self, message: Self::Message) {
		match message {
			Message::TabSelected(tab) => self.active_tab = tab,
		}
	}

	fn view(&self) -> Element<'_, Self::Message> {
		Tabs::new(self.active_tab, Message::TabSelected)
			.push(
				iced_aw::TabLabel::Text(String::from("Overview")),
				overview_tab(),
			)
			.push(iced_aw::TabLabel::Text("UwU".to_string()), overview_tab())
			.into()
	}
}

fn overview_tab() -> Element<'static, Message> {
	Column::new().push(attributes()).into()
}

fn attributes() -> Element<'static, Message> {
	fn mk_col(cat: TraitCategory) -> Column<'static, Message> {
		let col = Column::new();

		for attr in Attribute::get(AttributeCategory::Trait(cat)) {

		}

		col
	}

	Row::new()
		.push(Column::new())
		.push(Column::new())
		.push(Column::new())
		.into()
}

fn main() -> iced::Result {
	let lang_loader = i18n::load();

	println!("{}", fl!(lang_loader, "hello"));

	PlayerCompanionApp::run(Settings {
		..Default::default()
	})
}
