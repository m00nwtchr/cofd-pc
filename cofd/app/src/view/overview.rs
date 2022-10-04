use std::{cell::RefCell, rc::Rc};

use cofd::{
	character::{Trait, Wound},
	prelude::*,
	splat::{
		ability::{Ability, AbilityVal},
		changeling::Regalia,
		Splat, SplatType,
	},
};
use iced::{
	widget::{column, pick_list, row, text, text_input, Column},
	Alignment, Element, Length,
};
use iced_lazy::Component;

use crate::{
	component::{attributes::attribute_bar, info::info_bar, skills::skills_component},
	fl,
	// i18n::fl,
	widget::{self, dots::Shape, dots::SheetDots, track::HealthTrack},
	H2_SIZE,
	H3_SIZE,
};

pub struct OverviewTab<Message> {
	character: Rc<RefCell<Character>>,

	_c: Option<Message>,
}

pub fn overview_tab<Message>(character: Rc<RefCell<Character>>) -> OverviewTab<Message> {
	OverviewTab::new(character)
}

#[derive(Clone)]
pub enum Event {
	AttrChanged(u8, Attribute),
	SkillChanged(u8, Skill),
	TraitChanged(u8, Trait),
	// InfoTraitChanged(String, InfoTrait),
	// XSplatChanged(XSplat),
	// YSplatChanged(YSplat),
	AbilityChanged(Ability, AbilityVal),
	CustomAbilityChanged(Ability, String),
	HealthChanged(Wound),
	IntegrityDamage(SplatType, Wound),

	RegaliaChanged(Regalia),

	Msg,
}

impl<Message> OverviewTab<Message> {
	pub fn new(character: Rc<RefCell<Character>>) -> Self {
		Self {
			character,
			_c: None,
		}
	}

	fn abilities<Renderer>(&self, character: &Character) -> Element<Event, Renderer>
	where
		Renderer: iced_native::text::Renderer + 'static,
		Renderer::Theme: iced::widget::text::StyleSheet
			+ widget::dots::StyleSheet
			+ iced::widget::text_input::StyleSheet
			+ iced::widget::pick_list::StyleSheet,
	{
		let splat_name = character.splat.name();
		let mut col1 = Column::new().spacing(3).width(Length::Fill);
		let mut col2 = Column::new()
			.spacing(4)
			.width(Length::Fill)
			.align_items(Alignment::End);

		if character.splat.are_abilities_finite() {
			if let Some(abilities) = character.splat.all_abilities() {
				for ability in abilities {
					let val = match character.get_ability(&ability) {
						Some(val) => val.1,
						None => 0,
					};

					col1 = col1.push(text(fl(splat_name, Some(ability.name()))));
					col2 = col2.push(SheetDots::new(val, 0, 5, Shape::Dots, None, move |val| {
						Event::AbilityChanged(ability.clone(), AbilityVal(ability.clone(), val))
					}));
				}
			}
		} else {
			for ability in character.abilities.values() {
				if ability.0.is_custom() {
					// if let
					// 	Ability::Merit(Merit::_Custom(str))
					// 	| Ability::Discipline(Discipline::_Custom(str))
					// 	| Ability::MoonGift(MoonGift::_Custom(str)) = ability.0 {

					col1 = col1.push(text_input("", ability.0.name(), {
						let ab = ability.0.clone();
						move |val| Event::CustomAbilityChanged(ab.clone(), val)
					}));

				// }
				} else {
					let mut e: Vec<Ability> = character
						.splat
						.all_abilities()
						.unwrap()
						.iter()
						.filter(|e| !character.has_ability(e))
						.cloned()
						.collect();

					if let Some(ability) = character.splat.custom_ability("Custom".to_string()) {
						e.push(ability);
					}

					col1 = col1
						.push(
							pick_list(e, Some(ability.0.clone()), {
								let val = ability.clone();
								move |key| {
									Event::AbilityChanged(val.0.clone(), AbilityVal(key, val.1))
								}
							})
							.padding(1)
							.text_size(20),
						)
						.spacing(1);
				}

				col2 = col2.push(SheetDots::new(ability.1, 0, 5, Shape::Dots, None, {
					let key = ability.0.clone();
					move |val| Event::AbilityChanged(key.clone(), AbilityVal(key.clone(), val))
				}));
			}
		}

		let mut col = Column::new().align_items(Alignment::Center);
		if let Some(name) = character.splat.ability_name() {
			col = col
				.push(text(fl(splat_name, Some(name))).size(H3_SIZE))
				.push(column![row![col1, col2]]);
		}

		col.into()
	}
}

impl<Message, Renderer> Component<Message, Renderer> for OverviewTab<Message>
where
	Renderer: iced_native::text::Renderer + 'static,
	Renderer::Theme: iced::widget::pick_list::StyleSheet
		+ iced::widget::text_input::StyleSheet
		+ iced::widget::text::StyleSheet
		+ widget::dots::StyleSheet
		+ widget::track::StyleSheet,
{
	type State = ();

	type Event = Event;

	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		let mut character = self.character.borrow_mut();

		match event {
			Event::AttrChanged(val, attr) => *character.base_attributes_mut().get_mut(&attr) = val,
			Event::SkillChanged(val, skill) => *character.skills_mut().get_mut(&skill) = val,
			Event::AbilityChanged(ability, val) => {
				if character.has_ability(&ability) {
					character.remove_ability(&ability);
					character.add_ability(val);
				} else {
					character.add_ability(val);
				}
				// match character.get_ability_mut(&ability) {
				// 	Some(ability) => *ability = val,
				// 	None => character.add_ability(val),
				// }

				character.calc_mod_map();
				println!("{:?}", character.abilities);
			}
			Event::CustomAbilityChanged(ability, name) => {
				if let Some(mut val) = character.remove_ability(&ability) {
					*val.0.name_mut().unwrap() = name;
					character.add_ability(val);
				}
			}
			Event::TraitChanged(val, _trait) => match _trait {
				Trait::Willpower => character.willpower = val,
				Trait::Power => character.power = val,
				Trait::Fuel => character.fuel = val,
				Trait::Integrity => character.integrity = val,
				_ => {}
			},
			Event::HealthChanged(wound) => character.health_mut().poke(&wound),
			#[allow(clippy::single_match)]
			Event::IntegrityDamage(_type, wound) => match (_type, &mut character.splat) {
				(SplatType::Changeling, Splat::Changeling(_, _, _, data)) => {
					data.clarity.poke(&wound);
					if let Wound::Lethal = wound {
						data.clarity.poke(&Wound::Aggravated);
					}
				}
				_ => {}
			},
			Event::RegaliaChanged(regalia) => {
				if let Splat::Changeling(seeming, _, _, data) = &mut character.splat {
					// if !flag {
					data.regalia = Some(regalia);
					// } else if let Seeming::_Custom(_, _regalia) = seeming {
					// 	*_regalia = regalia;
					// }
				}
			}
			Event::Msg => {}
		}

		None
	}

	#[allow(clippy::too_many_lines)]
	fn view(&self, _state: &Self::State) -> iced_native::Element<Self::Event, Renderer> {
		let character = self.character.borrow();

		let health = {
			let track = HealthTrack::new(
				character.health().clone(),
				character.max_health() as usize,
				Event::HealthChanged,
			);

			let wp = character.wound_penalty();
			let mut label = fl!("health");

			if wp > 0 {
				label += &format!(" (-{wp})");
			}
			column![
				text(label).size(H3_SIZE),
				track // text(format!("{:?}", character.health_track))
			]
			.align_items(Alignment::Center)
		};

		let willpower = {
			let dots = SheetDots::new(
				character.willpower,
				0,
				character.max_willpower() as u8,
				Shape::Dots,
				None,
				|val| Event::TraitChanged(val, Trait::Willpower),
			);

			column![text(fl!("willpower")).size(H3_SIZE), dots].align_items(Alignment::Center)
		};

		let st = if let Some(st) = character.splat.supernatural_tolerance() {
			let dots = SheetDots::new(character.power, 1, 10, Shape::Dots, None, |val| {
				Event::TraitChanged(val, Trait::Power)
			});

			column![
				text(fl(character.splat.name(), Some(st))).size(H3_SIZE),
				dots
			]
			.align_items(Alignment::Center)
		} else {
			column![]
		};

		let fuel = if let Some(fuel) = character.splat.fuel() {
			let boxes = SheetDots::new(
				character.fuel,
				0,
				character.max_fuel(),
				Shape::Boxes,
				Some(10),
				|val| Event::TraitChanged(val, Trait::Fuel),
			);

			column![
				text(fl(character.splat.name(), Some(fuel))).size(H3_SIZE),
				boxes
			]
			.align_items(Alignment::Center)
		} else {
			column![]
		};

		let integrity = {
			let dots: Element<Event, Renderer> =
				if let Splat::Changeling(_, _, _, data) = &character.splat {
					HealthTrack::new(
						data.clarity.clone(),
						data.max_clarity(&character) as usize,
						|w| Event::IntegrityDamage(SplatType::Changeling, w),
					)
					.into()
				} else {
					SheetDots::new(character.integrity, 0, 10, Shape::Dots, None, |val| {
						Event::TraitChanged(val, Trait::Integrity)
					})
					.into()
				};

			let label = text(fl(
				character.splat.name(),
				Some(character.splat.integrity()),
			))
			.size(H3_SIZE);

			let mut col = Column::new().align_items(Alignment::Center);

			// match character.splat {
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

		match &character.splat {
			Splat::Mortal => {}
			Splat::Changeling(seeming, _, _, data) => {
				let sg = seeming.get_favored_regalia();
				let all_regalia: Vec<Regalia> = Regalia::all().to_vec();

				let seeming_regalia = text(fl(character.splat.name(), Some(sg.name())));
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
				// 		Event::RegaliaChanged(val, true)
				// 	})
				// 	.into()
				// } else {
				// text(fl(character.splat.name(), Some(sg.name()))).into()
				// };

				let regalia: Element<Event, Renderer> =
					if let Some(Regalia::_Custom(name)) = &data.regalia {
						text_input("", name, |val| Event::RegaliaChanged(Regalia::_Custom(val)))
							.width(Length::Fill)
							.into()
					} else {
						let reg: Vec<Regalia> = all_regalia
							.iter()
							.cloned()
							.filter(|reg| reg != sg)
							.collect();

						pick_list(reg, data.regalia.clone(), Event::RegaliaChanged)
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
				col1 = col1.push(self.abilities(&character));
			}
		}

		// let margin_col = || Column::new();

		// container(
		row![
			// (margin_col)(),
			column![
				column![
					info_bar(self.character.clone(), || Event::Msg),
					attribute_bar(character.base_attributes().clone(), Event::AttrChanged)
				]
				.align_items(Alignment::Center)
				.width(Length::Fill),
				row![
					skills_component(character.skills().clone(), Event::SkillChanged),
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
				// 	Event::LocaleChanged
				// )
			]
			.width(Length::Fill),
			// (margin_col)()
		]
		.width(Length::Fill)
		// )
		.padding(10)
		// .center_x()
		.into()
	}
}

impl<'a, Message, Renderer> From<OverviewTab<Message>> for Element<'a, Message, Renderer>
where
	Message: 'a,
	Renderer: 'static + iced_native::text::Renderer,
	Renderer::Theme: iced::widget::pick_list::StyleSheet
		+ iced::widget::text_input::StyleSheet
		+ iced::widget::text::StyleSheet
		+ widget::dots::StyleSheet
		+ widget::track::StyleSheet,
{
	fn from(overview_tab: OverviewTab<Message>) -> Self {
		iced_lazy::component(overview_tab)
	}
}
