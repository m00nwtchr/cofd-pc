use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use iced::{
	widget::{column, pick_list, row, text, text_input, Column},
	Alignment, Element, Length,
};
use iced_lazy::Component;

use cofd::{
	character::{ModifierTarget, Trait, Wound},
	prelude::*,
	splat::{
		ability::Ability,
		changeling::Regalia,
		mage::{Ministry, Order},
		Merit, Splat, SplatType,
	},
};

use crate::{
	component::{
		attributes::attribute_bar, info::info_bar, merits::merit_component,
		skills::skills_component, traits::traits_component,
	},
	fl,
	// i18n::fl,
	widget::{self, dots::Shape, dots::SheetDots, track::HealthTrack},
	H2_SIZE,
	H3_SIZE,
	MAX_INPUT_WIDTH,
};

pub struct OverviewTab<Message> {
	character: Rc<RefCell<Character>>,
	phantom: PhantomData<Message>,
}

pub fn overview_tab<Message>(character: Rc<RefCell<Character>>) -> OverviewTab<Message> {
	OverviewTab::new(character)
}

#[derive(Clone)]
pub enum Event {
	AttrChanged(u16, Attribute),
	SkillChanged(u16, Skill),
	TraitChanged(u16, Trait),
	// InfoTraitChanged(String, InfoTrait),
	// XSplatChanged(XSplat),
	// YSplatChanged(YSplat),
	AbilityValChanged(Ability, u16),
	AbilityChanged(Ability, Ability),
	MeritChanged(usize, Merit, u16),
	// CustomAbilityChanged(Ability, String),
	HealthChanged(Wound),
	IntegrityDamage(SplatType, Wound),
	TouchstoneChanged(usize, String),

	BaneChanged(usize, String),

	RegaliaChanged(Regalia),
	FrailtyChanged(usize, String),

	RoteSkillChanged(Skill),

	Msg,
}

impl<Message> OverviewTab<Message> {
	pub fn new(character: Rc<RefCell<Character>>) -> Self {
		Self {
			character,
			phantom: PhantomData,
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

		let mut new = Column::new().width(Length::Fill);

		if character.splat.are_abilities_finite() {
			if let Some(abilities) = character.splat.all_abilities() {
				for ability in abilities {
					let val = character.get_ability_value(&ability).unwrap_or(&0);

					col1 = col1.push(text(fl(splat_name, Some(ability.name())).unwrap()));
					col2 = col2.push(SheetDots::new(
						val.clone(),
						0,
						5,
						Shape::Dots,
						None,
						move |val| Event::AbilityValChanged(ability.clone(), val),
					));
				}
			}
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

			for (ability, val) in &character.abilities {
				if ability.is_custom() {
					col1 = col1.push(text_input("", ability.name(), {
						let ab = ability.clone();
						move |val| {
							let mut new = ab.clone();
							*new.name_mut().unwrap() = val;
							Event::AbilityChanged(ab.clone(), new)
						}
					}));
				} else {
					col1 = col1
						.push(
							pick_list(e.clone(), Some(ability.clone()), {
								let ability = ability.clone();
								move |val| Event::AbilityChanged(ability.clone(), val)
							})
							.width(Length::Fill)
							.padding(1)
							.text_size(20),
						)
						.spacing(1);
				}

				col2 = col2.push(SheetDots::new(val.clone(), 0, 5, Shape::Dots, None, {
					let ability = ability.clone();
					move |val| Event::AbilityValChanged(ability.clone(), val)
				}));
			}

			new = new.push(
				pick_list(e, None, |key| Event::AbilityValChanged(key, 0))
					.width(Length::Fill)
					.padding(1)
					.text_size(20),
			);
		}

		let mut col = Column::new().align_items(Alignment::Center);
		if let Some(name) = character.splat.ability_name() {
			col = col
				.push(text(fl(splat_name, Some(name)).unwrap()).size(H3_SIZE))
				.push(column![row![col1, col2], new]);
		}

		col.into()
	}
}

impl<Message, Renderer> Component<Message, Renderer> for OverviewTab<Message>
where
	Message: Clone,
	Renderer: iced_native::text::Renderer + 'static,
	Renderer::Theme: iced::widget::pick_list::StyleSheet
		+ iced::widget::text_input::StyleSheet
		+ iced::widget::text::StyleSheet
		+ iced::widget::button::StyleSheet
		+ widget::dots::StyleSheet
		+ widget::track::StyleSheet
		+ iced::widget::checkbox::StyleSheet,
{
	type State = ();

	type Event = Event;

	#[allow(clippy::too_many_lines)]
	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		let mut character = self.character.borrow_mut();

		let mut res = None;

		match event {
			Event::AttrChanged(val, attr) => *character.base_attributes_mut().get_mut(attr) = val,
			Event::SkillChanged(val, skill) => *character.base_skills_mut().get_mut(skill) = val,
			Event::AbilityValChanged(ability, val) => {
				if let Some(val_) = character.get_ability_value_mut(&ability) {
					*val_ = val;
				}

				character.calc_mod_map();
			}
			Event::AbilityChanged(ability, new) => {
				if character.has_ability(&ability) {
					let val = character.remove_ability(&ability).unwrap_or_default();
					character.add_ability(new, val);
				} else {
					character.add_ability(ability, 0);
				}
			}
			Event::MeritChanged(i, ability, val) => {
				let mut flag = false;

				if character.merits.len() == i {
					if !ability.get_modifiers(val).is_empty() {
						flag = true;
					}
					character.merits.push((ability, val));
				} else {
					let old = character.merits.remove(i);
					if old.0.get_modifiers(old.1) != ability.get_modifiers(val) {
						flag = true;
					}

					character.merits.insert(i, (ability, val));
				}

				if flag {
					character.calc_mod_map();
				}
			}
			Event::TraitChanged(val, _trait) => match _trait {
				Trait::Size => {
					character.base_size =
						(val as i16 - character._mod(ModifierTarget::Trait(Trait::Size))) as u16;
				}
				Trait::Willpower => character.willpower = val as u16,
				Trait::Power => character.power = val as u16,
				Trait::Fuel => character.fuel = val as u16,
				Trait::Integrity => character.integrity = val as u16,
				Trait::Beats => character.beats = val,
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
				if let Splat::Changeling(_seeming, _, _, data) = &mut character.splat {
					// if !flag {
					data.regalia = Some(regalia);
					// } else if let Seeming::_Custom(_, _regalia) = seeming {
					// 	*_regalia = regalia;
					// }
				}
			}
			Event::Msg => {}
			Event::TouchstoneChanged(i, str) => {
				if let Some(touchstone) = character.touchstones.get_mut(i) {
					*touchstone = str;
				} else {
					character.touchstones.resize(i + 1, String::new());
					*character.touchstones.get_mut(i).unwrap() = str;
				}
			}
			Event::FrailtyChanged(i, str) => {
				if let Splat::Changeling(_, _, _, data) = &mut character.splat {
					if let Some(frailty) = data.frailties.get_mut(i) {
						*frailty = str;
					} else {
						data.frailties.resize(i + 1, String::new());
						*data.frailties.get_mut(i).unwrap() = str;
					}
				}
			}
			Event::BaneChanged(i, str) => {
				if let Splat::Vampire(_, _, _, data) = &mut character.splat {
					if let Some(bane) = data.banes.get_mut(i) {
						*bane = str;
					} else {
						data.banes.resize(i + 1, String::new());
						*data.banes.get_mut(i).unwrap() = str;
					}
				}
			}
			Event::RoteSkillChanged(skill) => {
				if let Splat::Mage(
					_,
					Some(
						Order::_Custom(_, rote_skills)
						| Order::SeersOfTheThrone(Some(Ministry::_Custom(_, rote_skills))),
					),
					_,
					_,
				) = &mut character.splat
				{
					if !rote_skills.contains(&skill) {
						rote_skills.rotate_left(1);
						rote_skills[2] = skill;
					}
				}
			}
		}

		res
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
				character.max_willpower() as u16,
				Shape::Dots,
				None,
				|val| Event::TraitChanged(val as u16, Trait::Willpower),
			);

			column![text(fl!("willpower")).size(H3_SIZE), dots].align_items(Alignment::Center)
		};

		let st = if let Some(st) = character.splat.supernatural_tolerance() {
			let dots = SheetDots::new(character.power, 1, 10, Shape::Dots, None, |val| {
				Event::TraitChanged(val as u16, Trait::Power)
			});

			column![
				text(fl(character.splat.name(), Some(st)).unwrap()).size(H3_SIZE),
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
				|val| Event::TraitChanged(val as u16, Trait::Fuel),
			);

			column![
				text(fl(character.splat.name(), Some(fuel)).unwrap()).size(H3_SIZE),
				boxes
			]
			.align_items(Alignment::Center)
		} else {
			column![]
		};

		let integrity = {
			let dots: Element<Event, Renderer> = if let Splat::Changeling(_, _, _, data) =
				&character.splat
			{
				HealthTrack::new(
					data.clarity.clone(),
					data.max_clarity(&character) as usize,
					|w| Event::IntegrityDamage(SplatType::Changeling, w),
				)
				.into()
			} else {
				let mut coll = Column::new();

				let mut flag = false;

				if let Splat::Vampire(_, _, _, _) = &character.splat {
					flag = true;

					coll = coll.width(Length::FillPortion(4)).spacing(1);

					for i in 0..10 {
						coll = coll.push(
							column![text_input(
								"",
								character.touchstones.get(i).unwrap_or(&String::new()),
								move |val| Event::TouchstoneChanged(i, val),
							)]
							.max_width(MAX_INPUT_WIDTH),
						);
					}
				}

				row![
					column![
						SheetDots::new(character.integrity, 1, 10, Shape::Dots, None, |val| {
							Event::TraitChanged(val as u16, Trait::Integrity)
						})
						.axis(if flag {
							widget::dots::Axis::Vertical
						} else {
							widget::dots::Axis::Horizontal
						}),
					]
					.align_items(if flag {
						Alignment::End
					} else {
						Alignment::Center
					})
					.width(Length::Fill),
					coll
				]
				.align_items(Alignment::Center)
				.spacing(5)
				.into()
			};

			let label =
				text(fl(character.splat.name(), Some(character.splat.integrity())).unwrap())
					.size(H3_SIZE);

			let mut col = Column::new().align_items(Alignment::Center);

			if let Splat::Werewolf(_, _, _, _) = character.splat {
				col = col
					.push(
						text(fl(character.splat.name(), Some("flesh-touchstone")).unwrap())
							.size(H3_SIZE),
					)
					.push(
						column![text_input(
							"",
							character.touchstones.get(0).unwrap_or(&String::new()),
							|str| Event::TouchstoneChanged(0, str),
						)]
						.max_width(MAX_INPUT_WIDTH),
					);
			}

			col = col.push(label).push(dots);

			match character.splat {
				Splat::Werewolf(_, _, _, _) => {
					col = col
						.push(
							text(fl(character.splat.name(), Some("spirit-touchstone")).unwrap())
								.size(H3_SIZE),
						)
						.push(
							column![text_input(
								"",
								character.touchstones.get(1).unwrap_or(&String::new()),
								|str| Event::TouchstoneChanged(1, str),
							)]
							.max_width(MAX_INPUT_WIDTH),
						);
				}
				Splat::Changeling(_, _, _, _) => {
					col = col.push(text(fl!("touchstones")).size(H3_SIZE));
					for i in 0..10 {
						col = col.push(
							column![text_input(
								"",
								character.touchstones.get(i).unwrap_or(&String::new()),
								move |val| Event::TouchstoneChanged(i, val),
							)]
							.max_width(MAX_INPUT_WIDTH),
						);
					}
				}
				_ => (),
			}

			col
		};

		let mut col1 = Column::new()
			.align_items(Alignment::Center)
			.width(Length::Fill);

		match &character.splat {
			Splat::Mortal => {}
			Splat::Changeling(_, _, _, _) => {}
			_ => {
				col1 = col1.push(self.abilities(&character));
			}
		}

		col1 = col1.push(merit_component(
			character.splat._type(),
			character.merits.clone(),
			Event::MeritChanged,
		));

		match &character.splat {
			Splat::Changeling(seeming, _, _, data) => {
				let sg = seeming.get_favored_regalia();
				let all_regalia: Vec<Regalia> = Regalia::all().to_vec();

				let seeming_regalia = text(fl(character.splat.name(), Some(sg.name())).unwrap());
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

				let mut frailties = Column::new().width(Length::Fill).spacing(1);

				for i in 0..4 {
					frailties = frailties.push(column![text_input(
						"",
						data.frailties.get(i).unwrap_or(&String::new()),
						move |val| Event::FrailtyChanged(i, val),
					)]);
				}

				col1 = col1
					.push(
						column![
							text(fl!("favored-regalia")).size(H3_SIZE),
							column![seeming_regalia, regalia].width(Length::Fill)
						]
						.align_items(Alignment::Center)
						.width(Length::Fill),
					)
					.push(
						column![
							text(fl("changeling", Some("frailties")).unwrap()).size(H3_SIZE),
							frailties
						]
						.align_items(Alignment::Center)
						.width(Length::Fill),
					);
			}
			Splat::Vampire(_, _, _, data) => {
				let mut banes = Column::new().width(Length::Fill).spacing(1);

				for i in 0..3 {
					banes = banes.push(column![text_input(
						"",
						data.banes.get(i).unwrap_or(&String::new()),
						move |val| Event::BaneChanged(i, val),
					)]);
				}

				col1 = col1.push(
					column![
						text(fl("vampire", Some("banes")).unwrap()).size(H3_SIZE),
						banes
					]
					.align_items(Alignment::Center)
					.width(Length::Fill),
				);
			}
			_ => {}
		}

		col1 = col1.push(traits_component(&character, Event::TraitChanged));

		// let margin_col = || Column::new();

		// row![
		// (margin_col)(),
		column![
			column![
				info_bar(self.character.clone(), || Event::Msg),
				attribute_bar(self.character.clone(), Event::AttrChanged)
			]
			.align_items(Alignment::Center)
			.width(Length::Fill),
			row![
				skills_component(
					self.character.clone(),
					Event::SkillChanged,
					Event::RoteSkillChanged
				),
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
				.width(Length::FillPortion(2))
			],
			// pick_list(
			// 	Vec::from(LANGS),
			// 	Some(self.locale.clone()),
			// 	Event::LocaleChanged
			// )
		]
		.width(Length::Fill)
		// (margin_col)()
		// ]
		// .width(Length::Fill)
		// .padding(10)
		.into()
	}
}

impl<'a, Message, Renderer> From<OverviewTab<Message>> for Element<'a, Message, Renderer>
where
	Message: 'a + Clone,
	Renderer: 'static + iced_native::text::Renderer,
	Renderer::Theme: iced::widget::pick_list::StyleSheet
		+ iced::widget::text_input::StyleSheet
		+ iced::widget::text::StyleSheet
		+ iced::widget::button::StyleSheet
		+ widget::dots::StyleSheet
		+ widget::track::StyleSheet
		+ iced::widget::checkbox::StyleSheet,
{
	fn from(overview_tab: OverviewTab<Message>) -> Self {
		iced_lazy::component(overview_tab)
	}
}
