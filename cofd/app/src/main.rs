use std::{cell::RefCell, rc::Rc};

use i18n_embed::fluent::FluentLanguageLoader;
use iced::{
	alignment::Vertical,
	executor,
	widget::{column, container, row, text, text_input},
	widget::{Column, Row},
	Alignment, Application, Command, Element, Length, Sandbox, Settings, Theme,
};
use iced_aw::pure::{TabLabel, Tabs};

use cofd::{
	character::{AttributeCategory, InfoTrait, TraitCategory},
	prelude::*,
};

mod i18n;
mod widget;

use i18n::fl;
use widget::SheetDots;

struct PlayerCompanionApp {
	active_tab: usize,
	character: Character,
	// lang_loader: FluentLanguageLoader,
}

#[derive(Debug, Clone)]
enum Message {
	TabSelected(usize),
	AttrChanged(u8, Attribute),
	InfoTraitChanged(String, InfoTrait),
}

impl PlayerCompanionApp {
	fn overview_tab(&self) -> Element<'static, Message> {
		column![self.info(), self.attributes()]
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
				col1 = col1.push(text(format!("{}", fl("attribute", Some(attr.name())))));

				let v = app.character.base_attributes().get(&attr) as u8;
				col2 = col2.push(SheetDots::new(v, 1, 5, |val| {
					Message::AttrChanged(val, attr)
				}));
			}

			row![col1, col2].spacing(5).into()
		}
		column![
			text(fl!("attributes")).size(25),
			row![
				mk_col(self, TraitCategory::Mental),
				mk_col(self, TraitCategory::Physical),
				mk_col(self, TraitCategory::Social)
			]
			.spacing(10)
		]
		.align_items(Alignment::Center)
		.into()
	}

	fn info(&self) -> Element<'static, Message> {
		fn mk(app: &PlayerCompanionApp, info: Vec<InfoTrait>) -> Element<'static, Message> {
			let mut col1 = Column::new().spacing(3);
			let mut col2 = Column::new().spacing(3).max_width(120);

			for _trait in info {
				let (msg, attribute) = match _trait {
					InfoTrait::VirtueAnchor => {
						if app.character.splat.virtue_anchor() == "virtue" {
							("virtue", None)
						} else {
							(app.character.splat.name(), Some(app.character.splat.virtue_anchor()))
						}
					},
					InfoTrait::ViceAnchor => {
						if app.character.splat.vice_anchor() == "vice" {
							("vice", None)
						} else {
							(app.character.splat.name(), Some(app.character.splat.vice_anchor()))
						}
					},
					_ => (_trait.name(), None)
				};

				col1 = col1.push(text(format!("{}:", fl(msg, attribute))));
				col2 = col2.push(text_input(
					"",
					app.character.info.get(&_trait),
					move |val| Message::InfoTraitChanged(val, _trait),
				))
			}

			row![col1, col2].spacing(5).into()
		}

		// column![
		row![
			mk(
				self,
				vec![InfoTrait::Name, InfoTrait::Player, InfoTrait::Chronicle]
			),
			mk(
				self,
				vec![InfoTrait::VirtueAnchor, InfoTrait::ViceAnchor, InfoTrait::Concept]
			),
			mk(
				self,
				vec![InfoTrait::Chronicle, InfoTrait::Name, InfoTrait::Name]
			)
		]
		.spacing(10)
		// ]
		// .align_items(Alignment::Center)
		.into()
	}
}

impl Application for PlayerCompanionApp {
	type Executor = executor::Default;
	type Flags = ();
	type Message = Message;
	type Theme = Theme;

	fn new(_flags: ()) -> (Self, Command<Self::Message>) {
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
			Message::InfoTraitChanged(val, _trait) => *self.character.info.get_mut(&_trait) = val,
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
	env_logger::init();
	i18n::setup();

	PlayerCompanionApp::run(Settings {
		..Default::default()
	})
}
