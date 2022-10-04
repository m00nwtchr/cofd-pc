#![feature(is_some_with)]
#![deny(clippy::pedantic)]
#![allow(
	clippy::must_use_candidate,
	clippy::used_underscore_binding,
	clippy::unused_self,
	clippy::match_wildcard_for_single_variants,
	clippy::module_name_repetitions,
	clippy::wildcard_imports,
	clippy::match_same_arms,
	clippy::default_trait_access
)]

use i18n_embed::LanguageRequester;

use iced::{
	executor,
	widget::Column,
	widget::{column, container, pick_list, row, text, text_input},
	Alignment, Application, Command, Element, Length, Settings, Theme,
};
use iced_lazy::responsive;
// use iced_aw::pure::{TabLabel, Tabs};

use cofd::{
	character::{AttributeCategory, InfoTrait, Trait, TraitCategory, Wound},
	prelude::*,
	splat::{
		ability::{Ability, AbilityVal},
		changeling::{Court, Regalia, Seeming},
		mage::{Order, Path},
		vampire::{Bloodline, Clan, Covenant, Discipline},
		Splat, SplatType, XSplat, YSplat,
	},
};

mod i18n;
mod widget;

use i18n::{fl, Locale};
use widget::{HealthTrack, SheetDots};

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
	YSplatChanged(YSplat),
	AbilityChanged(Ability, AbilityVal),
	CustomAbilityChanged(Ability, String),
	HealthChanged(Wound),
	IntegrityDamage(SplatType, Wound),

	RegaliaChanged(Regalia),
}

impl PlayerCompanionApp {
	#[allow(clippy::too_many_lines)]
	fn overview_tab(&self) -> Element<'static, Message> {
		let health = {
			let track = HealthTrack::new(
				self.character.health().clone(),
				self.character.max_health() as usize,
				Message::HealthChanged,
			);

			let wp = self.character.wound_penalty();
			let mut label = fl!("health");

			if wp > 0 {
				label += &format!(" (-{wp})");
			}
			column![
				text(label).size(H3_SIZE),
				track // text(format!("{:?}", self.character.health_track))
			]
			.align_items(Alignment::Center)
		};

		let willpower = {
			let dots = SheetDots::new(
				self.character.willpower,
				0,
				self.character.max_willpower() as u8,
				widget::Shape::Dots,
				None,
				|val| Message::TraitChanged(val, Trait::Willpower),
			);

			column![text(fl!("willpower")).size(H3_SIZE), dots].align_items(Alignment::Center)
		};

		let st = if let Some(st) = self.character.splat.supernatural_tolerance() {
			let dots = SheetDots::new(
				self.character.power,
				1,
				10,
				widget::Shape::Dots,
				None,
				|val| Message::TraitChanged(val, Trait::Power),
			);

			column![
				text(fl(self.character.splat.name(), Some(st))).size(H3_SIZE),
				dots
			]
			.align_items(Alignment::Center)
		} else {
			column![]
		};

		let fuel = if let Some(fuel) = self.character.splat.fuel() {
			let boxes = SheetDots::new(
				self.character.fuel,
				0,
				self.character.max_fuel(),
				widget::Shape::Boxes,
				Some(10),
				|val| Message::TraitChanged(val, Trait::Fuel),
			);

			column![
				text(fl(self.character.splat.name(), Some(fuel))).size(H3_SIZE),
				boxes
			]
			.align_items(Alignment::Center)
		} else {
			column![]
		};

		let integrity = {
			let dots: Element<'static, Message> =
				if let Splat::Changeling(_, _, _, data) = &self.character.splat {
					HealthTrack::new(
						data.clarity.clone(),
						data.max_clarity(&self.character) as usize,
						|w| Message::IntegrityDamage(SplatType::Changeling, w),
					)
					.into()
				} else {
					SheetDots::new(
						self.character.integrity,
						0,
						10,
						widget::Shape::Dots,
						None,
						|val| Message::TraitChanged(val, Trait::Integrity),
					)
					.into()
				};

			let label = text(fl(
				self.character.splat.name(),
				Some(self.character.splat.integrity()),
			))
			.size(H3_SIZE);

			let mut col = Column::new().align_items(Alignment::Center);

			// match self.character.splat {
			// 	// Splat::Vampire(_, _, _) => todo!(),
			// 	Splat::Werewolf(_, _, _, _) => todo!(),
			// 	_ => ,
			// }

			col = col.push(label).push(dots);

			col
		};

		let mut col1 = Column::new()
			.align_items(Alignment::Center)
			.width(Length::Fill);

		match &self.character.splat {
			Splat::Mortal => {}
			Splat::Changeling(seeming, _, _, data) => {
				let sg = seeming.get_favored_regalia();
				let all_regalia: Vec<Regalia> = Regalia::all().to_vec();

				let seeming_regalia = text(fl(self.character.splat.name(), Some(sg.name())));
				// if let Seeming::_Custom(_, sg) = seeming {
				// 	let reg: Vec<Regalia> = all_regalia
				// 		.iter()
				// 		.cloned()
				// 		.filter(|reg| {
				// 			if let Some(regalia) = &data.regalia {
				// 				*reg != *regalia
				// 			} else {
				// 				true
				// 			}
				// 		})
				// 		.collect();

				// 	pick_list(reg, Some(sg.clone()), |val| {
				// 		Message::RegaliaChanged(val, true)
				// 	})
				// 	.into()
				// } else {
				// text(fl(self.character.splat.name(), Some(sg.name()))).into()
				// };

				let regalia: Element<'static, Message> =
					if let Some(Regalia::_Custom(name)) = &data.regalia {
						text_input("", name, |val| {
							Message::RegaliaChanged(Regalia::_Custom(val))
						})
						.width(Length::Fill)
						.into()
					} else {
						let reg: Vec<Regalia> = all_regalia
							.iter()
							.cloned()
							.filter(|reg| reg != sg)
							.collect();

						pick_list(reg, data.regalia.clone(), Message::RegaliaChanged)
							.width(Length::Fill)
							.into()
					};

				col1 = col1.push(
					column![
						text(fl!("favored-regalia")).size(H3_SIZE),
						column![seeming_regalia, regalia].width(Length::Fill)
					]
					.align_items(Alignment::Center)
					.width(Length::Fill),
				);
			}
			_ => {
				col1 = col1.push(self.abilities());
			}
		}

		// let margin_col = || Column::new();

		container(
			row![
				// (margin_col)(),
				column![
					column![self.info(), self.attributes(),]
						.align_items(Alignment::Center)
						.width(Length::Fill),
					row![
						self.skills(),
						column![
							text("Other Traits".to_uppercase()).size(H2_SIZE),
							row![
								col1,
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
				.width(Length::Fill),
				// (margin_col)()
			]
			.width(Length::Fill),
		)
		.padding(10)
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
			));
		}

		row![col1, col2].width(Length::Fill).spacing(5).into()
	}

	#[allow(clippy::single_match_else, clippy::similar_names)]
	fn info(&self) -> Element<'static, Message> {
		let col3 = match self.character.splat {
			Splat::Mortal => self.mk_info_col(vec![
				InfoTrait::Age,
				InfoTrait::Faction,
				InfoTrait::GroupName,
			]),
			_ => {
				let mut xsplats = XSplat::all(&self.character.splat._type());
				let mut ysplats = YSplat::all(&self.character.splat._type());

				xsplats.extend(self.custom_xsplats.iter().filter_map(|xsplat| {
					match (xsplat, &self.character.splat) {
						(XSplat::Vampire(_), Splat::Vampire(_, _, _))
						| (XSplat::Werewolf(_), Splat::Werewolf(_, _, _, _))
						| (XSplat::Mage(_), Splat::Mage(_, _, _))
						| (XSplat::Changeling(_), Splat::Changeling(_, _, _, _)) => Some(xsplat.clone()),
						_ => None,
					}
				}));

				row![
					column![
						text(format!(
							"{}:",
							fl(
								self.character.splat.name(),
								Some(self.character.splat.xsplat_name())
							)
						)),
						text(format!(
							"{}:",
							fl(
								self.character.splat.name(),
								Some(self.character.splat.ysplat_name())
							)
						))
					]
					.spacing(3),
					column![
						pick_list(
							xsplats,
							self.character.splat.xsplat(),
							Message::XSplatChanged
						)
						.padding(1)
						.width(Length::Fill),
						pick_list(
							ysplats,
							self.character.splat.ysplat(),
							Message::YSplatChanged
						)
						.padding(1)
						.width(Length::Fill)
					]
					.spacing(1)
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
			col2 = col2.push(SheetDots::new(
				v,
				1,
				5,
				widget::Shape::Dots,
				None,
				move |val| Message::AttrChanged(val, attr),
			));
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

	fn mk_skill_col(&self, cat: &TraitCategory) -> Element<'static, Message> {
		let mut col1 = Column::new().spacing(3);
		let mut col2 = Column::new()
			.spacing(4)
			.width(Length::Fill)
			.align_items(Alignment::End);

		for skill in Skill::get(cat) {
			col1 = col1.push(text(fl("skill", Some(skill.name()))));

			let v = self.character.skills().get(&skill);
			col2 = col2.push(SheetDots::new(
				*v,
				0,
				5,
				widget::Shape::Dots,
				None,
				move |val| Message::SkillChanged(val, skill.clone()),
			));
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
			self.mk_skill_col(&TraitCategory::Mental),
			self.mk_skill_col(&TraitCategory::Physical),
			self.mk_skill_col(&TraitCategory::Social),
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
					col2 = col2.push(SheetDots::new(
						val,
						0,
						5,
						widget::Shape::Dots,
						None,
						move |val| {
							Message::AbilityChanged(
								ability.clone(),
								AbilityVal(ability.clone(), val),
							)
						},
					));
				}
			}
		} else {
			for ability in self.character.abilities.values() {
				if ability.0.is_custom() {
					// if let
					// 	Ability::Merit(Merit::_Custom(str))
					// 	| Ability::Discipline(Discipline::_Custom(str))
					// 	| Ability::MoonGift(MoonGift::_Custom(str)) = ability.0 {

					col1 = col1.push(text_input("", ability.0.name(), {
						let ab = ability.0.clone();
						move |val| Message::CustomAbilityChanged(ab.clone(), val)
					}));

				// }
				} else {
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

					col1 = col1
						.push(
							pick_list(e, Some(ability.0.clone()), {
								let val = ability.clone();
								move |key| {
									Message::AbilityChanged(val.0.clone(), AbilityVal(key, val.1))
								}
							})
							.padding(1)
							.text_size(20),
						)
						.spacing(1);
				}

				col2 = col2.push(SheetDots::new(
					ability.1,
					0,
					5,
					widget::Shape::Dots,
					None,
					{
						let key = ability.0.clone();
						move |val| {
							Message::AbilityChanged(key.clone(), AbilityVal(key.clone(), val))
						}
					},
				));
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
			.with_st(3)
			.with_splat(Splat::Changeling(
				Seeming::Wizened,
				// Seeming::_Custom("bler".to_string(), Regalia::Jewels),
				Some(Court::Autumn),
				None,
				Default::default(),
			))
			.with_splat(Splat::Vampire(
				Clan::Ventrue,
				Some(Covenant::OrdoDracul),
				Some(Bloodline::_Custom(
					"Dragolescu".to_string(),
					[
						Discipline::Animalism,
						Discipline::Dominate,
						Discipline::Resilience,
						Discipline::Auspex,
					],
				)),
			))
			// .with_splat(Splat::Mage(Path::Mastigos, Some(Order::Mysterium), None))
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
				*self.character.base_attributes_mut().get_mut(&attr) = val;
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
				println!("{}, {}", fl!("attribute"), fl("attribute", None));
			}
			Message::XSplatChanged(xsplat) => self.character.splat.set_xsplat(Some(xsplat)),
			Message::YSplatChanged(ysplat) => self.character.splat.set_ysplat(Some(ysplat)),
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
				Trait::Willpower => self.character.willpower = val,
				Trait::Power => self.character.power = val,
				Trait::Fuel => self.character.fuel = val,
				Trait::Integrity => self.character.integrity = val,
				_ => {}
			},
			Message::HealthChanged(wound) => self.character.health_mut().poke(&wound),
			#[allow(clippy::single_match)]
			Message::IntegrityDamage(_type, wound) => match (_type, &mut self.character.splat) {
				(SplatType::Changeling, Splat::Changeling(_, _, _, data)) => {
					data.clarity.poke(&wound);
					if let Wound::Lethal = wound {
						data.clarity.poke(&Wound::Aggravated);
					}
				}
				_ => {}
			},
			Message::RegaliaChanged(regalia) => {
				if let Splat::Changeling(seeming, _, _, data) = &mut self.character.splat {
					// if !flag {
					data.regalia = Some(regalia);
					// } else if let Seeming::_Custom(_, _regalia) = seeming {
					// 	*_regalia = regalia;
					// }
				}
			}
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
