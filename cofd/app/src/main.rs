#![feature(is_some_with)]

use i18n_embed::LanguageRequester;

use iced::{
	executor,
	widget::Column,
	widget::{column, container, pick_list, row, text, text_input},
	Alignment, Application, Command, Element, Length, Settings, Theme,
};
// use iced_aw::pure::{TabLabel, Tabs};

use cofd::{
	character::{AttributeCategory, InfoTrait, Trait, TraitCategory},
	prelude::*,
	splat::{
		ability::{Ability, AbilityVal},
		mage::{Order, Path},
		vampire::Discipline,
		Splat, XSplat,
	},
};

mod i18n;
mod widget;

use i18n::{fl, Locale};
use widget::SheetDots;

struct PlayerCompanionApp {
	active_tab: usize,
	character: Character,
	custom_xsplats: Vec<XSplat>,
	locale: Locale,
	language_requester: Box<dyn LanguageRequester<'static>>,
}

const H2_SIZE: u16 = 25;
const H3_SIZE: u16 = 20;

// const LANGS: [Locale; 4] = [
// 	Locale::System,
// 	Locale::Lang(langid!("en-GB")),
// 	Locale::Lang(langid!("en-US")),
// 	Locale::Lang(langid!("pl-PL")),
// ];

#[derive(Debug, Clone)]
enum Message {
	TabSelected(usize),
	LocaleChanged(Locale),
	AttrChanged(u8, Attribute),
	SkillChanged(u8, Skill),
	InfoTraitChanged(String, InfoTrait),
	TraitChanged(u8, Trait),
	XSplatChanged(XSplat),
	AbilityChanged(Ability, AbilityVal),
	CustomAbilityChanged(Ability, String),
}

impl PlayerCompanionApp {
	fn overview_tab(&self) -> Element<'static, Message> {
		let health = {
			// let boxes = SheetBoxes::new();

			column![
				text(fl!("health")).size(H3_SIZE),
				text(format!("{:?}", self.character.health_track))
			]
			.align_items(Alignment::Center)
		};

		let willpower = {
			let dots = SheetDots::new(
				self.character.willpower,
				0,
				self.character.max_willpower() as u8,
				|val| Message::TraitChanged(val, Trait::Integrity),
			);

			column![text(fl!("willpower")).size(H3_SIZE), dots].align_items(Alignment::Center)
		};

		let st = if let Some(st) = self.character.splat.supernatural_tolerance() {
			let dots = SheetDots::new(self.character.power, 0, 10, |val| {
				Message::TraitChanged(val, Trait::Power)
			});

			column![
				text(fl(self.character.splat.name(), Some(st))).size(H3_SIZE),
				dots
			]
			.align_items(Alignment::Center)
		} else {
			column![]
		};

		let fuel = if let Some(fuel) = self.character.splat.fuel() {
			column![text(fl(self.character.splat.name(), Some(fuel))).size(H3_SIZE)]
				.align_items(Alignment::Center)
		} else {
			column![]
		};

		let integrity = {
			let dots = SheetDots::new(self.character.integrity, 0, 10, |val| {
				Message::TraitChanged(val, Trait::Integrity)
			});

			match self.character.splat {
				Splat::Vampire(_, _, _) => todo!(),
				Splat::Werewolf(_, _, _, _) => todo!(),
				Splat::Changeling(_, _, _) => todo!(),
				_ => column![
					text(fl(
						self.character.splat.name(),
						Some(self.character.splat.integrity())
					))
					.size(H3_SIZE),
					dots
				]
				.align_items(Alignment::Center),
			}
		};

		container(
			column![
				column![self.info(), self.attributes(),]
					.align_items(Alignment::Center)
					.width(Length::Fill),
				row![
					self.skills(),
					column![
						text("Other Traits".to_uppercase()).size(H2_SIZE),
						row![
							column![self.abilities()]
								.align_items(Alignment::Center)
								.width(Length::Fill),
							column![health, willpower, st, fuel, integrity]
								.align_items(Alignment::Center)
								.width(Length::Fill)
						]
					]
					.align_items(Alignment::Center)
					.padding(15)
					.width(Length::FillPortion(3))
				],
				// pick_list(
				// 	Vec::from(LANGS),
				// 	Some(self.locale.clone()),
				// 	Message::LocaleChanged
				// )
			]
			.padding(10), // .width(Length::Fill), // .align_items(Alignment::Center)
			              // .align_y(Vertical::Center)
		)
		.center_x()
		.into()
	}

	fn mk_info_col(&self, info: Vec<InfoTrait>) -> Element<'static, Message> {
		let mut col1 = Column::new().spacing(3);
		let mut col2 = Column::new()
			.spacing(3)
			.width(Length::Fill)
			.align_items(Alignment::End);

		for _trait in info {
			let (msg, attribute) = match _trait {
				InfoTrait::VirtueAnchor | InfoTrait::ViceAnchor => {
					if self.character.splat.virtue_anchor() == "virtue" {
						(_trait.name(), None)
					} else {
						match _trait {
							InfoTrait::VirtueAnchor => (
								self.character.splat.name(),
								Some(self.character.splat.virtue_anchor()),
							),
							InfoTrait::ViceAnchor => (
								self.character.splat.name(),
								Some(self.character.splat.vice_anchor()),
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
				self.character.info.get(&_trait),
				move |val| Message::InfoTraitChanged(val, _trait),
			))
		}

		row![col1, col2].width(Length::Fill).spacing(5).into()
	}

	fn info(&self) -> Element<'static, Message> {
		let col3 = match self.character.splat {
			Splat::Mortal => self.mk_info_col(vec![
				InfoTrait::Age,
				InfoTrait::Faction,
				InfoTrait::GroupName,
			]),
			_ => {
				let mut all = XSplat::all(self.character.splat._type());

				all.extend(self.custom_xsplats.iter().filter_map(|xsplat| {
					match (xsplat, &self.character.splat) {
						(XSplat::Vampire(_), Splat::Vampire(_, _, _))
						| (XSplat::Werewolf(_), Splat::Werewolf(_, _, _, _))
						| (XSplat::Mage(_), Splat::Mage(_, _, _))
						| (XSplat::Changeling(_), Splat::Changeling(_, _, _)) => Some(xsplat.clone()),
						_ => None,
					}
				}));

				row![
					column![text(format!(
						"{}:",
						fl(
							self.character.splat.name(),
							Some(self.character.splat.xsplat_name())
						)
					))]
					.spacing(3),
					column![
						pick_list(all, self.character.splat.xsplat(), Message::XSplatChanged)
							.width(Length::Fill)
					]
					.width(Length::Fill)
				]
				.width(Length::Fill)
				.spacing(5)
				.into()
			}
		};

		row![
			self.mk_info_col(vec![
				InfoTrait::Name,
				InfoTrait::Player,
				InfoTrait::Chronicle
			]),
			self.mk_info_col(vec![
				InfoTrait::VirtueAnchor,
				InfoTrait::ViceAnchor,
				InfoTrait::Concept
			]),
			col3,
		]
		.spacing(10)
		.into()
	}

	fn mk_attr_col(&self, cat: TraitCategory) -> Element<'static, Message> {
		let mut col1 = Column::new().spacing(3);
		let mut col2 = Column::new()
			.spacing(5)
			.width(Length::Fill)
			.align_items(Alignment::End);

		for attr in Attribute::get(AttributeCategory::Trait(cat)) {
			let v = self.character.base_attributes().get(&attr) as u8;

			col1 = col1.push(text(fl("attribute", Some(attr.name()))));
			col2 = col2.push(SheetDots::new(v, 1, 5, |val| {
				Message::AttrChanged(val, attr)
			}));
		}

		row![col1, col2]
			.width(Length::FillPortion(2))
			.spacing(5)
			.into()
	}

	fn attributes(&self) -> Element<'static, Message> {
		column![
			text(fl!("attributes")).size(H2_SIZE),
			row![
				column![
					text(fl("attribute", Some("power"))),
					text(fl("attribute", Some("finesse"))),
					text(fl("attribute", Some("resistance")))
				]
				.spacing(3)
				.width(Length::Fill)
				.align_items(Alignment::End),
				self.mk_attr_col(TraitCategory::Mental),
				self.mk_attr_col(TraitCategory::Physical),
				self.mk_attr_col(TraitCategory::Social),
				column![].width(Length::Fill)
			]
			.spacing(10)
		]
		.align_items(Alignment::Center)
		.into()
	}

	fn mk_skill_col(&self, cat: TraitCategory) -> Element<'static, Message> {
		let mut col1 = Column::new().spacing(3);
		let mut col2 = Column::new()
			.spacing(4)
			.width(Length::Fill)
			.align_items(Alignment::End);

		for skill in Skill::get(&cat) {
			col1 = col1.push(text(fl("skill", Some(skill.name()))));

			let v = self.character.skills().get(&skill);
			col2 = col2.push(SheetDots::new(*v, 0, 5, |val| {
				Message::SkillChanged(val, skill.clone())
			}));
		}

		column![
			text(fl(cat.name(), None)).size(H3_SIZE),
			text(fl!("unskilled", num = cat.unskilled())).size(17),
			row![col1, col2].spacing(5)
		]
		.align_items(Alignment::Center)
		.into()
	}

	fn skills(&self) -> Element<'static, Message> {
		column![
			text(fl!("skills").to_uppercase()).size(H2_SIZE),
			self.mk_skill_col(TraitCategory::Mental),
			self.mk_skill_col(TraitCategory::Physical),
			self.mk_skill_col(TraitCategory::Social),
		]
		.spacing(10)
		.padding(15)
		.align_items(Alignment::Center)
		.width(Length::Fill)
		.into()
	}

	fn abilities(&self) -> Element<'static, Message> {
		let splat_name = self.character.splat.name();
		let mut col1 = Column::new().spacing(3).width(Length::Fill);
		let mut col2 = Column::new()
			.spacing(4)
			.width(Length::Fill)
			.align_items(Alignment::End);

		if self.character.splat.are_abilities_finite() {
			if let Some(abilities) = self.character.splat.all_abilities() {
				for ability in abilities {
					let val = match self.character.get_ability(&ability) {
						Some(val) => val.1,
						None => 0,
					};

					col1 = col1.push(text(fl(splat_name, Some(ability.name()))));
					col2 = col2.push(SheetDots::new(val, 0, 5, |val| {
						Message::AbilityChanged(ability.clone(), AbilityVal(ability.clone(), val))
					}));
				}
			}
		} else {
			for ability in self.character.abilities.values() {
				if !ability.0.is_custom() {
					let mut e: Vec<Ability> = self
						.character
						.splat
						.all_abilities()
						.unwrap()
						.iter()
						.filter(|e| !self.character.has_ability(e))
						.cloned()
						.collect();

					if let Some(ability) = self.character.splat.custom_ability("Custom".to_string())
					{
						e.push(ability);
					}

					col1 = col1.push(
						pick_list(e, Some(ability.0.clone()), {
							let val = ability.clone();
							move |key| {
								Message::AbilityChanged(val.0.clone(), AbilityVal(key, val.1))
							}
						})
						.text_size(20),
					);
				} else {
					// if let
					// 	Ability::Merit(Merit::_Custom(str))
					// 	| Ability::Discipline(Discipline::_Custom(str))
					// 	| Ability::MoonGift(MoonGift::_Custom(str)) = ability.0 {

					col1 = col1.push(text_input("", ability.0.name(), {
						let ab = ability.0.clone();
						move |val| Message::CustomAbilityChanged(ab.clone(), val)
					}));

					// }
				}

				col2 = col2.push(SheetDots::new(ability.1, 0, 5, |val| {
					Message::AbilityChanged(ability.0.clone(), AbilityVal(ability.0.clone(), val))
				}));
			}
		}

		let mut col = Column::new().align_items(Alignment::Center);
		if let Some(name) = self.character.splat.ability_name() {
			col = col
				.push(text(fl(splat_name, Some(name))).size(H3_SIZE))
				.push(column![row![col1, col2]]);
		}

		col.into()
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
			// .with_splat(Splat::Vampire(
			// 	Clan::Ventrue,
			// 	Some(Covenant::OrdoDracul),
			// 	Some(Bloodline::_Custom(
			// 		"Dragolescu".to_string(),
			// 		[
			// 			Discipline::Animalism,
			// 			Discipline::Dominate,
			// 			Discipline::Resilience,
			// 			Discipline::Auspex,
			// 		],
			// 	)),
			// ))
			.with_splat(Splat::Mage(Path::Mastigos, Some(Order::Mysterium), None))
			// .with_splat(Splat::Werewolf(
			// 	Some(Auspice::Rahu),
			// 	Some(Tribe::BloodTalons),
			// 	None,
			// 	Default::default(),
			// ))
			.with_attributes(Attributes {
				intelligence: 3,
				wits: 3,
				resolve: 2,
				strength: 1,
				dexterity: 3,
				stamina: 2,
				presence: 3,
				manipulation: 2,
				composure: 3,
			})
			.with_skills(Skills {
				investigation: 2,
				occult: 3,
				politics: 2,
				larceny: 3,
				stealth: 1,
				animal_ken: 1,
				expression: 3,
				intimidation: 1,
				streetwise: 2,
				subterfuge: 4,
				..Default::default()
			})
			.with_abilities([
				AbilityVal(Ability::Discipline(Discipline::Animalism), 1),
				AbilityVal(Ability::Discipline(Discipline::Dominate), 2),
				AbilityVal(
					Ability::Discipline(Discipline::_Custom("Coil of the Voivode".to_string())),
					2,
				),
			])
			.build();

		(
			Self {
				active_tab: 0,
				character,
				locale: Default::default(), // lang_loader,
				language_requester,
				custom_xsplats: vec![
					// My OC (Original Clan) (Do Not Steal)
					// XSplat::Vampire(Clan::_Custom(
					// 	"Blorbo".to_owned(),
					// 	[
					// 		Discipline::Majesty,
					// 		Discipline::Dominate,
					// 		Discipline::Auspex,
					// 	],
					// 	[Attribute::Intelligence, Attribute::Presence],
					// )),
				],
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
			Message::SkillChanged(val, skill) => *self.character.skills_mut().get_mut(&skill) = val,
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
			Message::XSplatChanged(xsplat) => self.character.splat.set_xsplat(Some(xsplat)),
			Message::AbilityChanged(ability, val) => {
				if self.character.has_ability(&ability) {
					self.character.remove_ability(&ability);
					self.character.add_ability(val);
				} else {
					self.character.add_ability(val);
				}
				// match self.character.get_ability_mut(&ability) {
				// 	Some(ability) => *ability = val,
				// 	None => self.character.add_ability(val),
				// }

				self.character.calc_mod_map();
				println!("{:?}", self.character.abilities);
			}
			Message::CustomAbilityChanged(ability, name) => {
				if let Some(mut val) = self.character.remove_ability(&ability) {
					*val.0.name_mut().unwrap() = name;
					self.character.add_ability(val);
				}
			}
			Message::TraitChanged(val, _trait) => match _trait {
				// Trait::Willpower => self.character.w,
				Trait::Power => self.character.power = val,
				// Trait::Fuel => self.character.fu,
				Trait::Integrity => self.character.integrity = val,
				_ => {}
			},
		}

		Command::none()
	}

	fn view(&self) -> Element<'_, Self::Message> {
		self.overview_tab()
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
	#[cfg(not(target_arch = "wasm32"))]
	env_logger::init();
	#[cfg(target_arch = "wasm32")]
	console_log::init_with_level(Level::Info);

	PlayerCompanionApp::run(Settings {
		..Default::default()
	})
}
