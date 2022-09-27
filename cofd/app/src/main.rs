use std::{cell::RefCell, fmt::Display, rc::Rc};

use i18n_embed::{fluent::FluentLanguageLoader, LanguageRequester};
use iced::{
	alignment::Vertical,
	executor,
	widget::{column, container, pick_list, row, text, text_input},
	widget::{Column, Row},
	Alignment, Application, Command, Element, Length, Sandbox, Settings, Theme,
};
use iced_aw::pure::{TabLabel, Tabs};

use cofd::{
	character::{AttributeCategory, InfoTrait, TraitCategory},
	prelude::*,
	splat::{vampire::Clan, Splat},
};

mod i18n;
mod widget;

use i18n::{fl, Locale};
use unic_langid::langid;
use widget::SheetDots;

struct PlayerCompanionApp {
	active_tab: usize,
	character: Character,
	locale: Locale,
	language_requester: Box<dyn LanguageRequester<'static>>,
}

const LANGS: [Locale; 4] = [
	Locale::System,
	Locale::Lang(langid!("en-GB")),
	Locale::Lang(langid!("en-US")),
	Locale::Lang(langid!("pl-PL")),
];

#[derive(Debug, Clone)]
enum Message {
	TabSelected(usize),
	LocaleChanged(Locale),
	AttrChanged(u8, Attribute),
	InfoTraitChanged(String, InfoTrait),
}

impl PlayerCompanionApp {
	fn overview_tab(&self) -> Element<'static, Message> {
		column![
			self.info(),
			self.attributes(),
			pick_list(
				Vec::from(LANGS),
				Some(self.locale.clone()),
				Message::LocaleChanged
			)
		]
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
					InfoTrait::VirtueAnchor | InfoTrait::ViceAnchor => {
						if app.character.splat.virtue_anchor() == "virtue" {
							(_trait.name(), None)
						} else {
							match _trait {
								InfoTrait::VirtueAnchor => (
									app.character.splat.name(),
									Some(app.character.splat.virtue_anchor()),
								),
								InfoTrait::ViceAnchor => (
									app.character.splat.name(),
									Some(app.character.splat.vice_anchor()),
								),
								_ => unreachable!(),
							}
						}
					}
					_ => (_trait.name(), None),
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
				vec![
					InfoTrait::VirtueAnchor,
					InfoTrait::ViceAnchor,
					InfoTrait::Concept
				]
			),
			match self.character.splat {
				Splat::Mortal => mk(
					self,
					vec![InfoTrait::Age, InfoTrait::Faction, InfoTrait::GroupName]
				),
				_ => row![
					column![text(fl(
						self.character.splat.name(),
						Some(self.character.splat.xsplat_name())
					))]
					.spacing(3),
					column![].spacing(3)
				]
				.spacing(5)
				.into(),
			},
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
		let language_requester = i18n::setup();

		let character = Character::builder()
			// .with_splat(Splat::Vampire(Clan::Ventrue, None, None))
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
				locale: Default::default(), // lang_loader,
				language_requester,
			},
			Command::none(),
		)
	}

	fn title(&self) -> String {
		fl!("app-name")
	}

	fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
		match message {
			Message::TabSelected(tab) => self.active_tab = tab,
			Message::AttrChanged(val, attr) => {
				*self.character.base_attributes_mut().get_mut(&attr) = val as i8;
			}
			Message::InfoTraitChanged(val, _trait) => *self.character.info.get_mut(&_trait) = val,
			Message::LocaleChanged(locale) => {
				self.locale = locale;
				self.language_requester
					.set_language_override(match &self.locale {
						Locale::System => None,
						Locale::Lang(id) => Some(id.clone()),
					})
					.unwrap();
				self.language_requester.poll().unwrap();
				println!("{}, {}", fl!("attribute"), fl("attribute", None))
			}
		}

		Command::none()
	}

	fn view(&self) -> Element<'_, Self::Message> {
		self.overview_tab().into()
		// Tabs::new(self.active_tab, Message::TabSelected)
		// .push(
		// 	TabLabel::Text(String::from("Overview")),
		// 	self.overview_tab(),
		// )
		// .push(TabLabel::Text("UwU".to_string()), self.overview_tab())
		// .into()
	}
}

fn main() -> iced::Result {
	env_logger::init();

	PlayerCompanionApp::run(Settings {
		..Default::default()
	})
}
