use closure::closure;
use cofd::{
	character::Wound,
	prelude::*,
	splat::{
		ability::Ability,
		changeling::Regalia,
		werewolf::{
			Auspice, HuntersAspect, KuruthTrigger, KuruthTriggerSet, KuruthTriggers, Tribe,
		},
		Splat,
	},
	traits::DerivedTrait,
};
use iced::{
	theme,
	widget::{column, pick_list, row, text, text_input, Column, Row},
	Alignment, Element, Length,
};

use crate::{
	component::{
		attributes, info, info::InfoBar, integrity, list, merits, skills, traits, AttributeBar,
		IntegrityComponent, MeritComponent, SkillsComponent, TraitsComponent,
	},
	fl, i18n,
	i18n::{Translate, Translated},
	widget::{
		dots::{Shape, SheetDots},
		track::HealthTrack,
	},
	COMPONENT_SPACING, H2_SIZE, H3_SIZE, INPUT_PADDING, MAX_INPUT_WIDTH, TITLE_SPACING,
};

#[derive(Debug, Clone)]
pub struct OverviewTab {
	info_bar: InfoBar,
	attribute_bar: AttributeBar,
	skills_component: SkillsComponent,
	merit_component: MeritComponent,
	traits_component: TraitsComponent,
	integrity_component: IntegrityComponent,
}

#[derive(Clone)]
pub enum Message {
	// InfoTraitChanged(String, InfoTrait),
	// XSplatChanged(XSplat),
	// YSplatChanged(YSplat),
	AbilityValChanged(Ability, u16),
	AbilityChanged(Ability, Ability),
	NewAbility(Ability),

	// CustomAbilityChanged(Ability, String),
	HealthChanged(Wound),
	WillpowerChanged(u16),
	PowerChanged(u16),
	FuelChanged(u16),

	// IntegrityDamage(SplatType, Wound),
	// TouchstoneChanged(usize, String),
	ConditionChanged(usize, String),
	AspirationChanged(usize, String),
	SplatThingChanged(usize, String),

	RegaliaChanged(Regalia),

	KuruthTriggersChanged(KuruthTriggers),
	KuruthTriggerChanged(KuruthTrigger, String),
	HuntersAspectChanged(HuntersAspect),

	InfoBar(info::Message),
	AttributeBar(attributes::Message),
	SkillComponent(skills::Message),
	MeritComponent(merits::Message),
	TraitsComponent(traits::Message),
	IntegrityComponent(integrity::Message),
}

pub fn vec_changed<T: Default + Clone>(i: usize, val: T, vec: &mut Vec<T>) {
	if let Some(v) = vec.get_mut(i) {
		*v = val;
	} else {
		vec.resize_with(i + 1, Default::default);
		*vec.get_mut(i).unwrap() = val;
	}
}

impl OverviewTab {
	pub fn new() -> Self {
		Self {
			info_bar: InfoBar::new(),
			attribute_bar: AttributeBar::new(),
			skills_component: SkillsComponent::new(),
			merit_component: MeritComponent::new(),
			traits_component: TraitsComponent::new(),
			integrity_component: IntegrityComponent::new(),
		}
	}

	#[allow(clippy::too_many_lines)]
	pub fn update(&mut self, message: Message, character: &mut Character) {
		match message {
			Message::AbilityValChanged(ability, val) => {
				if let Some(val_) = character.get_ability_value_mut(&ability) {
					*val_ = val;
				} else {
					character.add_ability(ability, val);
				}

				character.calc_mod_map();
			}
			Message::AbilityChanged(ability, new) => {
				if character.has_ability(&ability) {
					let val = character.remove_ability(&ability).unwrap_or_default();
					character.add_ability(new, val);
				}
			}
			Message::NewAbility(ability) => {
				if !character.has_ability(&ability) {
					character.add_ability(ability, 0);
				}
			}

			Message::HealthChanged(wound) => character.health_mut().poke(&wound),
			Message::WillpowerChanged(willpower) => character.willpower = willpower,
			Message::PowerChanged(power) => character.power = power,
			Message::FuelChanged(fuel) => character.fuel = fuel,

			Message::ConditionChanged(i, val) => {
				if val.is_empty() {
					character.conditions.remove(i);
				} else {
					vec_changed(i, val, &mut character.conditions);
				}
			}
			Message::AspirationChanged(i, val) => {
				if val.is_empty() {
					character.aspirations.remove(i);
				} else {
					vec_changed(i, val, &mut character.aspirations);
				}
			}
			Message::SplatThingChanged(i, val) => match &mut character.splat {
				Splat::Changeling(.., data) => {
					if val.is_empty() {
						data.frailties.remove(i);
					} else {
						vec_changed(i, val, &mut data.frailties);
					}
				}
				Splat::Vampire(.., data) => {
					if val.is_empty() {
						data.banes.remove(i);
					} else {
						vec_changed(i, val, &mut data.banes);
					}
				}
				Splat::Mage(.., data) => {
					if val.is_empty() {
						data.obsessions.remove(i);
					} else {
						vec_changed(i, val, &mut data.obsessions);
					}
				}
				_ => (),
			},
			Message::RegaliaChanged(regalia) => {
				if let Splat::Changeling(data) = &mut character.splat {
					data.regalia = regalia;
				}
			}

			Message::KuruthTriggersChanged(triggers) => {
				if let Splat::Werewolf(.., data) = &mut character.splat {
					data.triggers = triggers;
				}
			}
			Message::KuruthTriggerChanged(trigger, val) => {
				if let Splat::Werewolf(.., data) = &mut character.splat {
					if let KuruthTriggers::_Custom(triggers) = &mut data.triggers {
						match trigger {
							KuruthTrigger::Passive => triggers.passive = val,
							KuruthTrigger::Common => triggers.common = val,
							KuruthTrigger::Specific => triggers.specific = val,
						}
					}
				}
			}
			Message::HuntersAspectChanged(val) => {
				if let Splat::Werewolf(data) = &mut character.splat {
					if let Some(Auspice::_Custom(.., hunters_aspect)) = &mut data.auspice {
						if let HuntersAspect::_Custom(_) = &val {
							*hunters_aspect = val;
						}
					}
				}
			}

			Message::InfoBar(message) => self.info_bar.update(message, character),
			Message::AttributeBar(message) => self.attribute_bar.update(message, character),
			Message::SkillComponent(message) => self.skills_component.update(message, character),
			Message::MeritComponent(message) => self.merit_component.update(message, character),
			Message::TraitsComponent(message) => self.traits_component.update(message, character),
			Message::IntegrityComponent(message) => {
				self.integrity_component.update(message, character);
			}
		}
	}

	#[allow(clippy::too_many_lines)]
	pub fn view(&self, character: &Character) -> Element<Message> {
		let health = {
			let track = HealthTrack::new(
				character.health().clone(),
				character.max_health() as usize,
				Message::HealthChanged,
			);

			let wp = character.wound_penalty();
			let mut label = fl!("health");

			if wp > 0 {
				label += &format!(" (-{wp})");
			}
			column![text(label).size(H3_SIZE), track]
				.spacing(TITLE_SPACING)
				.align_items(Alignment::Center)
		};

		let willpower = {
			let dots = SheetDots::new(
				character.willpower,
				0,
				character.max_willpower(),
				Shape::Dots,
				None,
				Message::WillpowerChanged,
			);

			column![text(fl!("willpower")).size(H3_SIZE), dots]
				.spacing(TITLE_SPACING)
				.align_items(Alignment::Center)
		};

		let st = if let Some(st) = character.splat.supernatural_tolerance() {
			let dots = SheetDots::new(
				character.power,
				1,
				10,
				Shape::Dots,
				None,
				Message::PowerChanged,
			);

			column![text(st.translated()).size(H3_SIZE), dots]
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
				Message::FuelChanged,
			);

			column![text(fuel.translated()).size(H3_SIZE), boxes]
				.spacing(TITLE_SPACING)
				.align_items(Alignment::Center)
		} else {
			column![]
		};

		let integrity = self
			.integrity_component
			.view(character)
			.map(Message::IntegrityComponent);

		let conditions = list(
			fl!("conditions"),
			Some(character.conditions.len() + 1),
			None,
			character.conditions.clone(),
			|i, val| {
				text_input("", &val.unwrap_or_default())
					.on_input(move |val| Message::ConditionChanged(i, val))
					.padding(INPUT_PADDING)
					.into()
			},
		);
		// .max_width(MAX_INPUT_WIDTH);

		let aspirations = list(
			fl!("aspirations"),
			Some(character.aspirations.len() + 1),
			Some(3),
			character.aspirations.clone(),
			|i, val| {
				text_input("", &val.unwrap_or_default())
					.on_input(move |val| Message::AspirationChanged(i, val))
					.padding(INPUT_PADDING)
					.into()
			},
		);
		// .max_width(MAX_INPUT_WIDTH);

		let obsessions = if let Splat::Mage(.., data) = &character.splat {
			column![list(
				fl!("obsessions"),
				Some(1),
				Some(match character.power {
					1..=2 => 1,
					3..=5 => 2,
					6..=8 => 3,
					9..=10 => 4,
					_ => 1,
				}),
				data.obsessions.clone(),
				|i, val| text_input("", &val.unwrap_or_default(),)
					.on_input(move |val| Message::SplatThingChanged(i, val))
					.padding(INPUT_PADDING)
					.into()
			)]
			.max_width(MAX_INPUT_WIDTH)
		} else {
			column![]
		};

		let kuruth_triggers = if let Splat::Werewolf(.., data) = &character.splat {
			let (passive, common, specific): (
				Element<Message>,
				Element<Message>,
				Element<Message>,
			) = if let KuruthTriggers::_Custom(KuruthTriggerSet {
				passive,
				common,
				specific,
			}) = &data.triggers
			{
				(
					text_input("", passive)
						.on_input(|passive| {
							Message::KuruthTriggerChanged(KuruthTrigger::Passive, passive)
						})
						.padding(INPUT_PADDING)
						.into(),
					text_input("", common)
						.on_input(|common| {
							Message::KuruthTriggerChanged(KuruthTrigger::Common, common)
						})
						.padding(INPUT_PADDING)
						.into(),
					text_input("", specific)
						.on_input(|specific| {
							Message::KuruthTriggerChanged(KuruthTrigger::Specific, specific)
						})
						.padding(INPUT_PADDING)
						.into(),
				)
			} else {
				let name = data.triggers.name().unwrap();

				let passive = i18n::LANGUAGE_LOADER.get_attr(name, "passive");
				let common = i18n::LANGUAGE_LOADER.get_attr(name, "common");
				let specific = i18n::LANGUAGE_LOADER.get_attr(name, "specific");

				(
					text(passive).into(),
					text(common).into(),
					text(specific).into(),
				)
			};

			let vec: Vec<Translated<KuruthTriggers>> =
				KuruthTriggers::all().into_iter().map(Into::into).collect();

			let txt = text(fl!("passive")).style(theme::Text::default());
			column![
				txt,
				text(fl!("kuruth-triggers")),
				column![
					pick_list(
						vec,
						Some::<Translated<KuruthTriggers>>(data.triggers.clone().into()),
						|val| Message::KuruthTriggersChanged(val.unwrap())
					)
					.width(Length::Fill)
					.padding(INPUT_PADDING),
					text(fl!("passive")).style(theme::Text::default()),
					passive,
					text(fl!("common")),
					common,
					text(fl!("specific")),
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

		let abilities = self.abilities(character);
		let merits = self
			.merit_component
			.view(character)
			.map(Message::MeritComponent);
		let traits = self
			.traits_component
			.view(character)
			.map(Message::TraitsComponent);

		let regalia = if let Splat::Changeling(data) = &character.splat {
			let favoured_regalia = data.seeming.get_favored_regalia();
			let all_regalia: Vec<Regalia> = Regalia::all().to_vec();

			let seeming_regalia = text(favoured_regalia.translated());

			let regalia: Element<Message> = if let Regalia::_Custom(name) = &data.regalia {
				text_input("", name)
					.on_input(|val| Message::RegaliaChanged(Regalia::_Custom(val)))
					.width(Length::Fill)
					.padding(INPUT_PADDING)
					.into()
			} else {
				let reg: Vec<Translated<Regalia>> = all_regalia
					.iter()
					.filter(|&regalia| regalia != favoured_regalia)
					.cloned()
					.map(Into::into)
					.collect();

				pick_list(
					reg,
					Some::<Translated<Regalia>>(data.regalia.clone().into()),
					|val| Message::RegaliaChanged(val.unwrap()),
				)
				.width(Length::Fill)
				.into()
			};

			column![
				text(fl!("favored-regalia")).size(H3_SIZE),
				column![seeming_regalia, regalia].width(Length::Fill)
			]
			.align_items(Alignment::Center)
			.width(Length::Fill)
		} else {
			column![]
		};

		let frailties: Element<Message> = if let Splat::Changeling(.., data) = &character.splat {
			list(
				fl!("frailties"),
				Some(3),
				Some(3),
				data.frailties.clone(),
				|i, val| {
					text_input("", &val.unwrap_or_default())
						.on_input(move |val| Message::SplatThingChanged(i, val))
						.padding(INPUT_PADDING)
						.into()
				},
			)
			.into()
		} else {
			column![].into()
		};

		let banes: Element<Message> = if let Splat::Vampire(.., data) = &character.splat {
			list(
				fl!("banes"),
				Some(3),
				Some(3),
				data.banes.clone(),
				|i, val| {
					text_input("", &val.unwrap_or_default())
						.on_input(move |val| Message::SplatThingChanged(i, val))
						.padding(INPUT_PADDING)
						.into()
				},
			)
			.into()
		} else {
			column![].into()
		};

		let hunters_aspect: Element<Message> = if let Splat::Werewolf(data) = &character.splat {
			let mut vec: Vec<Translated<HuntersAspect>> = if let Some(auspice) = &data.auspice {
				vec![auspice.get_hunters_aspect().clone().into()]
			} else if let Some(Tribe::Pure(tribe)) = &data.tribe {
				tribe
					.get_hunters_aspects()
					.iter()
					.cloned()
					.map(Into::into)
					.collect()
			} else {
				Vec::new()
			};

			vec.push(HuntersAspect::_Custom(fl!("custom")).into());

			let mut col = column![text(fl!("hunters-aspect")),]
				.align_items(Alignment::Center)
				.spacing(TITLE_SPACING);

			if let Some(HuntersAspect::_Custom(name)) = &data.hunters_aspect {
				col = col.push(
					text_input("", name)
						.on_input(|val| Message::HuntersAspectChanged(HuntersAspect::_Custom(val)))
						.padding(INPUT_PADDING),
				);
			} else {
				col = col.push(
					pick_list(
						vec,
						data.hunters_aspect
							.clone()
							.map(Into::<Translated<HuntersAspect>>::into),
						|val| Message::HuntersAspectChanged(val.unwrap()),
					)
					.width(Length::Fill)
					.padding(INPUT_PADDING),
				);
			}

			col.into()
		} else {
			column![].into()
		};

		let mut col1: Column<Message> = Column::new()
			.align_items(Alignment::Center)
			.width(Length::Fill)
			.spacing(COMPONENT_SPACING);

		let mut col2 = Column::new()
			.push(health)
			.push(willpower)
			.spacing(COMPONENT_SPACING)
			.align_items(Alignment::Center)
			.width(Length::Fill);

		match &character.splat {
			Splat::Mortal(..) => {}
			Splat::Bound(..) => {
				col2 = col2.push(fuel);
			}
			_ => {
				col2 = col2.push(st).push(fuel);
			}
		}

		col2 = col2.push(integrity);

		match &character.splat {
			Splat::Vampire(..) => {
				col1 = col1
					.push(abilities)
					.push(merits)
					.push(aspirations)
					.push(banes)
					.push(conditions);
				col2 = col2.push(traits);
			}
			Splat::Werewolf(..) => {
				col1 = col1
					.push(merits)
					.push(abilities)
					.push(aspirations)
					.push(hunters_aspect)
					.push(conditions)
					.push(traits);
				col2 = col2.push(kuruth_triggers);
			}
			Splat::Mage(..) => {
				col1 = col1.push(abilities).push(merits).push(traits);
				col2 = col2.push(conditions).push(aspirations).push(obsessions);
			}
			Splat::Changeling(..) => {
				col1 = col1
					.push(merits)
					.push(regalia)
					.push(frailties)
					.push(aspirations)
					.push(conditions);
				col2 = col2.push(traits);
			}
			Splat::Bound(..) => {
				col1 = col1
					.push(merits)
					// TODO: Keys
					.push(abilities);
				col2 = col2.push(aspirations);
			}
			_ => {
				col1 = col1.push(merits).push(traits);
				col2 = col2.push(conditions).push(aspirations);
			}
		}

		column![
			column![
				self.info_bar.view(character).map(Message::InfoBar),
				self.attribute_bar
					.view(character)
					.map(Message::AttributeBar)
			]
			.align_items(Alignment::Center)
			.width(Length::Fill),
			row![
				self.skills_component
					.view(character)
					.map(Message::SkillComponent),
				column![
					text("Other Traits".to_uppercase()).size(H2_SIZE),
					row![col1, col2].spacing(20)
				]
				.spacing(crate::TITLE_SPACING)
				.align_items(Alignment::Center)
				// .padding(11)
				.width(Length::FillPortion(2))
			]
			.spacing(20)
			.padding(20),
			// pick_list(
			// 	Vec::from(LANGS),
			// 	Some(self.locale.clone()),
			// 	Message::LocaleChanged
			// )
		]
		// .spacing(10)
		.width(Length::Fill)
		.into()
	}

	fn abilities(&self, character: &Character) -> Element<Message> {
		let mut col = Column::new().spacing(3);

		if let Some(abilities) = character.splat.all_abilities() {
			if character.splat.are_abilities_finite() {
				for ability in abilities {
					let val = character.get_ability_value(&ability).unwrap_or(&0);

					col = col.push(
						Row::new()
							.push(text(ability.translated()).width(Length::Fill))
							.push(SheetDots::new(*val, 0, 5, Shape::Dots, None, move |val| {
								Message::AbilityValChanged(ability.clone(), val)
							})),
					);
				}
			} else {
				let mut vec = abilities.clone();

				if let Some(ability) = character.splat.custom_ability(fl!("custom")) {
					vec.push(ability);
				}

				let vec: Vec<Translated<Ability>> = vec
					.iter()
					.filter(|e| !character.has_ability(e))
					.cloned()
					.map(Into::into)
					.collect();

				for (ability, val) in &character.abilities {
					let item: Element<Message> = if ability.is_custom() {
						text_input("", ability.name())
							.on_input(closure!(clone ability, |val| {
								let mut new = ability.clone();
								*new.name_mut().unwrap() = val;
								Message::AbilityChanged(ability.clone(), new)
							}))
							.width(Length::Fill)
							.padding(INPUT_PADDING)
							.into()
					} else {
						pick_list(
							vec.clone(),
							Some::<Translated<Ability>>(ability.clone().into()),
							closure!(clone ability, |val| Message::AbilityChanged(ability.clone(), val.unwrap())),
						)
						.width(Length::Fill)
						.padding(INPUT_PADDING)
						.text_size(20)
						.into()
					};

					let dots = SheetDots::new(
						*val,
						0,
						5,
						Shape::Dots,
						None,
						closure!(clone ability, |val| Message::AbilityValChanged(ability.clone(), val)),
					);

					col = col.push(row![item, dots]);
				}

				col = col.push(
					pick_list(vec, None::<Translated<Ability>>, |key| {
						Message::NewAbility(key.unwrap())
					})
					.width(Length::Fill)
					.padding(INPUT_PADDING)
					.text_size(20),
				);
			}
		}

		Column::new()
			.align_items(Alignment::Center)
			.spacing(TITLE_SPACING)
			.push(
				text(if let Some(name) = character.splat.ability_name() {
					i18n::LANGUAGE_LOADER.get(name)
				} else {
					String::new()
				})
				.size(H3_SIZE),
			)
			.push(col)
			.into()
	}
}
