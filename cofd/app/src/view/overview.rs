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
		werewolf::{
			HuntersAspect, KuruthTrigger, KuruthTriggerSet, KuruthTriggers, Tribe, WerewolfData,
		},
		Merit, NameKey, Splat, SplatType,
	},
};

use crate::{
	component::{
		attribute_bar, info_bar, integrity_component, list, merit_component, skills_component,
		traits_component,
	},
	fl,
	i18n::Translated,
	// i18n::fl,
	widget::{self, dots::Shape, dots::SheetDots, track::HealthTrack},
	COMPONENT_SPACING,
	H2_SIZE,
	H3_SIZE,
	INPUT_PADDING,
	MAX_INPUT_WIDTH,
	TITLE_SPACING,
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
	// IntegrityDamage(SplatType, Wound),
	// TouchstoneChanged(usize, String),
	ConditionChanged(usize, String),
	AspirationChanged(usize, String),
	SplatThingChanged(usize, String),

	RegaliaChanged(Regalia),

	KuruthTriggersChanged(KuruthTriggers),
	KuruthTriggerChanged(KuruthTrigger, String),
	HuntersAspectChanged(HuntersAspect),

	RoteSkillChanged(Skill),

	Msg,
}

pub fn vec_changed<T: Default + Clone>(i: usize, val: T, vec: &mut Vec<T>) {
	if let Some(v) = vec.get_mut(i) {
		*v = val;
	} else {
		vec.resize_with(i + 1, Default::default);
		*vec.get_mut(i).unwrap() = val;
	}
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

					col1 = col1.push(text(fl(splat_name, Some(&ability.name())).unwrap()));
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

			if let Some(ability) = character.splat.custom_ability(fl!("custom")) {
				e.push(ability);
			}

			let e: Vec<Translated<Ability>> = e.into_iter().map(Into::into).collect();

			for (ability, val) in &character.abilities {
				if ability.is_custom() {
					col1 = col1.push(
						text_input("", ability.name(), {
							let ab = ability.clone();
							move |val| {
								let mut new = ab.clone();
								*new.name_mut().unwrap() = val;
								Event::AbilityChanged(ab.clone(), new)
							}
						})
						.padding(INPUT_PADDING),
					);
				} else {
					col1 = col1
						.push(
							pick_list(e.clone(), Some(ability.clone().into()), {
								let ability = ability.clone();
								move |val| Event::AbilityChanged(ability.clone(), val.unwrap())
							})
							.width(Length::Fill)
							.padding(INPUT_PADDING)
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
				pick_list(e, None, |key| Event::AbilityValChanged(key.unwrap(), 0))
					.width(Length::Fill)
					.padding(INPUT_PADDING)
					.text_size(20),
			);
		}

		let mut col = Column::new()
			.align_items(Alignment::Center)
			.spacing(TITLE_SPACING);
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
			Event::ConditionChanged(i, val) => vec_changed(i, val, &mut character.conditions),
			Event::AspirationChanged(i, val) => vec_changed(i, val, &mut character.aspirations),
			Event::SplatThingChanged(i, val) => match &mut character.splat {
				Splat::Changeling(_, _, _, data) => vec_changed(i, val, &mut data.frailties),
				Splat::Vampire(_, _, _, data) => vec_changed(i, val, &mut data.banes),
				Splat::Mage(_, _, _, data) => vec_changed(i, val, &mut data.obsessions),
				_ => (),
			},
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
			Event::RegaliaChanged(regalia) => {
				if let Splat::Changeling(_seeming, _, _, data) = &mut character.splat {
					data.regalia = Some(regalia);
				}
			}

			Event::Msg => {}
			Event::KuruthTriggersChanged(triggers) => {
				if let Splat::Werewolf(_, _, _, data) = &mut character.splat {
					data.triggers = triggers;
				}
			}
			Event::KuruthTriggerChanged(trigger, val) => {
				if let Splat::Werewolf(
					..,
					WerewolfData {
						triggers:
							KuruthTriggers::_Custom(KuruthTriggerSet {
								passive,
								common,
								specific,
							}),
						..
					},
				) = &mut character.splat
				{
					match trigger {
						KuruthTrigger::Passive => *passive = val,
						KuruthTrigger::Common => *common = val,
						KuruthTrigger::Specific => *specific = val,
					}
				}
			}
			Event::HuntersAspectChanged(val) => {
				if let Splat::Werewolf(_, _, _, data) = &mut character.splat {
					data.hunters_aspect = Some(val);
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
			.spacing(TITLE_SPACING)
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

			column![text(fl!("willpower")).size(H3_SIZE), dots]
				.spacing(TITLE_SPACING)
				.align_items(Alignment::Center)
		};

		let st = if let Some(st) = character.splat.supernatural_tolerance() {
			let dots = SheetDots::new(character.power, 1, 10, Shape::Dots, None, |val| {
				Event::TraitChanged(val as u16, Trait::Power)
			});

			column![
				text(fl(character.splat.name(), Some(st)).unwrap()).size(H3_SIZE),
				dots
			]
			.spacing(TITLE_SPACING)
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
			.spacing(TITLE_SPACING)
			.align_items(Alignment::Center)
		} else {
			column![]
		};

		let integrity = integrity_component(self.character.clone());

		let conditions = list(
			fl!("conditions"),
			character.conditions.len() + 1,
			character.conditions.clone(),
			|i, val| {
				text_input("", &val, move |val| Event::ConditionChanged(i, val))
					.padding(INPUT_PADDING)
					.into()
			},
		)
		.max_width(MAX_INPUT_WIDTH);

		let aspirations = list(
			fl!("aspirations"),
			character.aspirations.len() + 1,
			character.aspirations.clone(),
			|i, val| {
				text_input("", &val, move |val| Event::AspirationChanged(i, val))
					.padding(INPUT_PADDING)
					.into()
			},
		)
		.max_width(MAX_INPUT_WIDTH);

		let obsessions = if let Splat::Mage(_, _, _, data) = &character.splat {
			column![list(
				fl("mage", Some("obsessions")).unwrap(),
				5,
				// match character.power {
				// 	1..=2 => 1,
				// 	3..=5 => 2,
				// 	6..=8 => 3,
				// 	9..=10 => 4,
				// 	_ => 1,
				// },
				data.obsessions.clone(),
				|i, val| text_input("", &val, move |val| Event::SplatThingChanged(i, val))
					.padding(INPUT_PADDING)
					.into()
			)]
			.max_width(MAX_INPUT_WIDTH)
		} else {
			column![]
		};

		let kuruth_triggers = if let Splat::Werewolf(_, _, _, data) = &character.splat {
			let (passive, common, specific): (
				Element<Event, Renderer>,
				Element<Event, Renderer>,
				Element<Event, Renderer>,
			) = if let KuruthTriggers::_Custom(KuruthTriggerSet {
				passive,
				common,
				specific,
			}) = &data.triggers
			{
				(
					text_input("", passive, |passive| {
						Event::KuruthTriggerChanged(KuruthTrigger::Passive, passive)
					})
					.padding(INPUT_PADDING)
					.into(),
					text_input("", common, |common| {
						Event::KuruthTriggerChanged(KuruthTrigger::Common, common)
					})
					.padding(INPUT_PADDING)
					.into(),
					text_input("", specific, |specific| {
						Event::KuruthTriggerChanged(KuruthTrigger::Specific, specific)
					})
					.padding(INPUT_PADDING)
					.into(),
				)
			} else {
				let name = data.triggers.name().unwrap();

				let passive = fl("kuruth-triggers", Some(&format!("{}-passive", name))).unwrap();
				let common = fl("kuruth-triggers", Some(&format!("{}-common", name))).unwrap();
				let specific = fl("kuruth-triggers", Some(&format!("{}-specific", name))).unwrap();

				(
					text(passive).into(),
					text(common).into(),
					text(specific).into(),
				)
			};

			column![
				text(fl!("kuruth-triggers")),
				column![
					pick_list(
						KuruthTriggers::all().to_vec(),
						Some(data.triggers.clone()),
						Event::KuruthTriggersChanged,
					)
					.width(Length::Fill)
					.padding(INPUT_PADDING),
					text(fl("werewolf", Some("passive")).unwrap()),
					passive,
					text(fl("werewolf", Some("common")).unwrap()),
					common,
					text(fl("werewolf", Some("specific")).unwrap()),
					specific
				]
				.align_items(Alignment::Center)
			]
			.align_items(Alignment::Center)
			.spacing(TITLE_SPACING)
			.max_width(MAX_INPUT_WIDTH)
		} else {
			column![]
		};

		let mut col1 = Column::new()
			.align_items(Alignment::Center)
			.width(Length::Fill)
			.spacing(COMPONENT_SPACING);

		match &character.splat {
			Splat::Mortal => {}
			Splat::Changeling(_, _, _, _) => {}
			_ => {
				col1 = col1.push(self.abilities(&character));
			}
		}

		col1 = col1.push(merit_component(self.character.clone(), Event::MeritChanged));

		match &character.splat {
			Splat::Changeling(seeming, _, _, data) => {
				let sg = seeming.get_favored_regalia();
				let all_regalia: Vec<Regalia> = Regalia::all().to_vec();

				let seeming_regalia = text(fl(character.splat.name(), Some(&sg.name())).unwrap());

				let regalia: Element<Event, Renderer> =
					if let Some(Regalia::_Custom(name)) = &data.regalia {
						text_input("", name, |val| Event::RegaliaChanged(Regalia::_Custom(val)))
							.width(Length::Fill)
							.padding(INPUT_PADDING)
							.into()
					} else {
						let reg: Vec<Translated<Regalia>> = all_regalia
							.iter()
							.cloned()
							.filter(|reg| reg != sg)
							.map(Into::into)
							.collect();

						pick_list(reg, data.regalia.clone().map(Into::into), |val| {
							Event::RegaliaChanged(val.unwrap())
						})
						.width(Length::Fill)
						.into()
					};

				let frailties = list(
					fl("changeling", Some("frailties")).unwrap(),
					3,
					data.frailties.clone(),
					|i, val| {
						text_input("", &val, move |val| Event::SplatThingChanged(i, val))
							.padding(INPUT_PADDING)
							.into()
					},
				);

				col1 = col1
					.push(
						column![
							text(fl!("favored-regalia")).size(H3_SIZE),
							column![seeming_regalia, regalia].width(Length::Fill)
						]
						.align_items(Alignment::Center)
						.width(Length::Fill),
					)
					.push(frailties);
			}
			Splat::Vampire(_, _, _, data) => {
				col1 = col1.push(list(
					fl("vampire", Some("banes")).unwrap(),
					3,
					data.banes.clone(),
					|i, val| {
						text_input("", &val, move |val| Event::SplatThingChanged(i, val))
							.padding(INPUT_PADDING)
							.into()
					},
				));
			}
			Splat::Werewolf(auspice, tribe, _, data) => {
				let mut vec = Vec::new();

				if let Some(auspice) = auspice {
					vec.push(auspice.get_hunters_aspect().clone());
				} else if let Some(Tribe::Pure(tribe)) = tribe {
					vec.extend(tribe.get_hunters_aspects().clone());
				}

				vec.push(HuntersAspect::_Custom(fl!("custom")));

				col1 = col1.push(
					column![
						text(fl("werewolf", Some("hunters_aspect")).unwrap()),
						pick_list(vec, data.hunters_aspect.clone(), |val| {
							Event::HuntersAspectChanged(val)
						})
						.width(Length::Fill)
						.padding(INPUT_PADDING)
					]
					.align_items(Alignment::Center)
					.spacing(TITLE_SPACING),
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
						column![
							health,
							willpower,
							st,
							fuel,
							integrity,
							conditions,
							aspirations,
							obsessions,
							kuruth_triggers
						]
						.spacing(COMPONENT_SPACING)
						.align_items(Alignment::Center)
						.width(Length::Fill)
					]
				]
				.spacing(crate::TITLE_SPACING)
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
