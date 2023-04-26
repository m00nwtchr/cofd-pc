use closure::closure;
use iced::{
	widget::{column, pick_list, row, text, text_input, Column},
	Alignment, Length,
};
use iced_lazy::Component;
use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use cofd::{
	character::Wound,
	prelude::*,
	splat::{
		ability::Ability,
		changeling::Regalia,
		mage::{Ministry, Order},
		werewolf::{HuntersAspect, KuruthTrigger, KuruthTriggerSet, KuruthTriggers, Tribe},
		Merit, Splat,
	},
};

use crate::{
	component::{
		attribute_bar, info_bar, integrity_component, list, merit_component, skills_component,
		traits_component,
	},
	fl,
	i18n::{flt, Translated},
	widget::{dots::Shape, dots::SheetDots, track::HealthTrack},
	Element, COMPONENT_SPACING, H2_SIZE, H3_SIZE, INPUT_PADDING, MAX_INPUT_WIDTH, TITLE_SPACING,
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
	SpecialtyChanged(Skill, usize, String),
	TraitChanged(u16, Trait),
	// InfoTraitChanged(String, InfoTrait),
	// XSplatChanged(XSplat),
	// YSplatChanged(YSplat),
	AbilityValChanged(Ability, u16),
	AbilityChanged(Ability, Ability),
	NewAbility(Ability),
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

	fn abilities(&self, character: &Character) -> Element<Event> {
		let splat_name = character.splat.name();

		let mut col = Column::new()
			.align_items(Alignment::Center)
			.spacing(TITLE_SPACING);

		if let Some(abilities) = character.splat.all_abilities() {
			let mut col1 = Column::new().spacing(3).width(Length::FillPortion(3));
			let mut col2 = Column::new()
				.spacing(4)
				.width(Length::FillPortion(2))
				.align_items(Alignment::End);

			let mut new = Column::new().width(Length::Fill);

			if character.splat.are_abilities_finite() {
				for ability in abilities {
					let val = character.get_ability_value(&ability).unwrap_or(&0);

					col1 = col1.push(text(flt(splat_name, Some(ability.name())).unwrap()));
					col2 = col2.push(SheetDots::new(*val, 0, 5, Shape::Dots, None, move |val| {
						Event::AbilityValChanged(ability.clone(), val)
					}));
				}
			} else {
				let mut vec = character.splat.all_abilities().unwrap();

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
					if ability.is_custom() {
						col1 = col1.push(
							text_input(
								"",
								ability.name(),
								closure!(clone ability, |val| {
									let mut new = ability.clone();
									*new.name_mut().unwrap() = val;
									Event::AbilityChanged(ability.clone(), new)
								}),
							)
							.padding(INPUT_PADDING),
						);
					} else {
						col1 = col1
							.push(
								pick_list(
									vec.clone(),
									Some(ability.clone().into()),
									closure!(clone ability, |val| Event::AbilityChanged(ability.clone(), val.unwrap())),
								)
								.width(Length::Fill)
								.padding(INPUT_PADDING)
								.text_size(20),
							)
							.spacing(1);
					}

					col2 = col2.push(SheetDots::new(
						*val,
						0,
						5,
						Shape::Dots,
						None,
						closure!(clone ability, |val| Event::AbilityValChanged(ability.clone(), val)),
					));
				}

				new = new.push(
					pick_list(vec, None, |key| Event::NewAbility(key.unwrap()))
						.width(Length::Fill)
						.padding(INPUT_PADDING)
						.text_size(20),
				);
			}

			if let Some(name) = character.splat.ability_name() {
				col = col
					.push(text(flt(splat_name, Some(name)).unwrap()).size(H3_SIZE))
					.push(column![row![col1, col2], new]);
			}
		}

		col.into()
	}
}

impl<Message> Component<Message, iced::Renderer> for OverviewTab<Message>
where
	Message: Clone,
{
	type State = ();

	type Event = Event;

	#[allow(clippy::too_many_lines)]
	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		let mut character = self.character.borrow_mut();

		let res = None;

		match event {
			Event::AttrChanged(val, attr) => *character.base_attributes_mut().get_mut(&attr) = val,
			Event::SkillChanged(val, skill) => *character.base_skills_mut().get_mut(skill) = val,
			Event::SpecialtyChanged(skill, i, val) => {
				if let Some(vec) = character.specialties.get_mut(&skill) {
					if val.is_empty() {
						vec.remove(i);
					} else {
						vec_changed(i, val, vec);
					}
				} else {
					character.specialties.insert(skill, vec![val]);
				}
			}
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
				}
			}
			Event::NewAbility(ability) => {
				if !character.has_ability(&ability) {
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

					if !ability.name().is_empty() {
						character.merits.insert(i, (ability, val));
					}
				}

				if flag {
					character.calc_mod_map();
				}
			}
			Event::TraitChanged(val, _trait) => match _trait {
				Trait::Size => {
					character.base_size = (val as i16 - character.get_modifier(Trait::Size)) as u16;
				}
				Trait::Willpower => character.willpower = val,
				Trait::Power => character.power = val,
				Trait::Fuel => character.fuel = val,
				Trait::Integrity => character.integrity = val,
				Trait::Beats => character.beats = val,
				Trait::AlternateBeats => character.alternate_beats = val,
				_ => {}
			},
			Event::HealthChanged(wound) => character.health_mut().poke(&wound),
			Event::ConditionChanged(i, val) => {
				if val.is_empty() {
					character.conditions.remove(i);
				} else {
					vec_changed(i, val, &mut character.conditions)
				}
			}
			Event::AspirationChanged(i, val) => {
				if val.is_empty() {
					character.aspirations.remove(i);
				} else {
					vec_changed(i, val, &mut character.aspirations)
				}
			}
			Event::SplatThingChanged(i, val) => match &mut character.splat {
				Splat::Changeling(.., data) => {
					if val.is_empty() {
						data.frailties.remove(i);
					} else {
						vec_changed(i, val, &mut data.frailties)
					}
				}
				Splat::Vampire(.., data) => {
					if val.is_empty() {
						data.banes.remove(i);
					} else {
						vec_changed(i, val, &mut data.banes)
					}
				}
				Splat::Mage(.., data) => {
					if val.is_empty() {
						data.obsessions.remove(i);
					} else {
						vec_changed(i, val, &mut data.obsessions)
					}
				}
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
				if let Splat::Changeling(_seeming, .., data) = &mut character.splat {
					data.regalia = Some(regalia);
				}
			}

			Event::Msg => {}
			Event::KuruthTriggersChanged(triggers) => {
				if let Splat::Werewolf(.., data) = &mut character.splat {
					data.triggers = triggers;
				}
			}
			Event::KuruthTriggerChanged(trigger, val) => {
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
			Event::HuntersAspectChanged(val) => {
				if let Splat::Werewolf(.., data) = &mut character.splat {
					if let HuntersAspect::_Custom(name) = &val && name.eq("") {
						data.hunters_aspect = None;
					} else {
						data.hunters_aspect = Some(val);
					}
				}
			}
		}

		res
	}

	#[allow(clippy::too_many_lines)]
	fn view(&self, _state: &Self::State) -> iced_native::Element<Self::Event, iced::Renderer> {
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
				|val| Event::TraitChanged(val, Trait::Willpower),
			);

			column![text(fl!("willpower")).size(H3_SIZE), dots]
				.spacing(TITLE_SPACING)
				.align_items(Alignment::Center)
		};

		let st = if let Some(st) = character.splat.supernatural_tolerance() {
			let dots = SheetDots::new(character.power, 1, 10, Shape::Dots, None, |val| {
				Event::TraitChanged(val, Trait::Power)
			});

			column![
				text(flt(character.splat.name(), Some(st)).unwrap()).size(H3_SIZE),
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
				|val| Event::TraitChanged(val, Trait::Fuel),
			);

			column![
				text(flt(character.splat.name(), Some(fuel)).unwrap()).size(H3_SIZE),
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
			Some(character.conditions.len() + 1),
			None,
			character.conditions.clone(),
			|i, val| {
				text_input("", &val.unwrap_or_default(), move |val| {
					Event::ConditionChanged(i, val)
				})
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
				text_input("", &val.unwrap_or_default(), move |val| {
					Event::AspirationChanged(i, val)
				})
				.padding(INPUT_PADDING)
				.into()
			},
		);
		// .max_width(MAX_INPUT_WIDTH);

		let obsessions = if let Splat::Mage(.., data) = &character.splat {
			column![list(
				flt("mage", Some("obsessions")).unwrap(),
				Some(1),
				Some(match character.power {
					1..=2 => 1,
					3..=5 => 2,
					6..=8 => 3,
					9..=10 => 4,
					_ => 1,
				}),
				data.obsessions.clone(),
				|i, val| text_input("", &val.unwrap_or_default(), move |val| {
					Event::SplatThingChanged(i, val)
				})
				.padding(INPUT_PADDING)
				.into()
			)]
			.max_width(MAX_INPUT_WIDTH)
		} else {
			column![]
		};

		let kuruth_triggers = if let Splat::Werewolf(.., data) = &character.splat {
			let (passive, common, specific): (Element<Event>, Element<Event>, Element<Event>) =
				if let KuruthTriggers::_Custom(KuruthTriggerSet {
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

					let passive = flt("kuruth-triggers", Some(&format!("{name}-passive"))).unwrap();
					let common = flt("kuruth-triggers", Some(&format!("{name}-common"))).unwrap();
					let specific =
						flt("kuruth-triggers", Some(&format!("{name}-specific"))).unwrap();

					(
						text(passive).into(),
						text(common).into(),
						text(specific).into(),
					)
				};

			let vec: Vec<Translated<KuruthTriggers>> =
				KuruthTriggers::all().into_iter().map(Into::into).collect();

			column![
				text(fl!("kuruth-triggers")),
				column![
					pick_list(vec, Some(data.triggers.clone().into()), |val| {
						Event::KuruthTriggersChanged(val.unwrap())
					})
					.width(Length::Fill)
					.padding(INPUT_PADDING),
					text(fl!("werewolf", "passive")),
					passive,
					text(fl!("werewolf", "common")),
					common,
					text(fl!("werewolf", "specific")),
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

		let abilities = self.abilities(&character);
		let merits = merit_component(self.character.clone(), Event::MeritChanged);
		let traits = traits_component(&character, Event::TraitChanged);

		let regalia = if let Splat::Changeling(seeming, .., data) = &character.splat {
			let sg = seeming.get_favored_regalia();
			let all_regalia: Vec<Regalia> = Regalia::all().to_vec();

			let seeming_regalia = text(flt(character.splat.name(), Some(sg.name())).unwrap());

			let regalia: Element<Event> = if let Some(Regalia::_Custom(name)) = &data.regalia {
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

			column![
				text(fl!("favored-regalia")).size(H3_SIZE),
				column![seeming_regalia, regalia].width(Length::Fill)
			]
			.align_items(Alignment::Center)
			.width(Length::Fill)
		} else {
			column![]
		};

		let frailties: Element<Self::Event> = if let Splat::Changeling(.., data) = &character.splat
		{
			list(
				fl!("changeling", "frailties"),
				Some(3),
				Some(3),
				data.frailties.clone(),
				|i, val| {
					text_input("", &val.unwrap_or_default(), move |val| {
						Event::SplatThingChanged(i, val)
					})
					.padding(INPUT_PADDING)
					.into()
				},
			)
			.into()
		} else {
			column![].into()
		};

		let banes: Element<Self::Event> = if let Splat::Vampire(.., data) = &character.splat {
			list(
				fl!("vampire", "banes"),
				Some(3),
				Some(3),
				data.banes.clone(),
				|i, val| {
					text_input("", &val.unwrap_or_default(), move |val| {
						Event::SplatThingChanged(i, val)
					})
					.padding(INPUT_PADDING)
					.into()
				},
			)
			.into()
		} else {
			column![].into()
		};

		let hunters_aspect: Element<Self::Event> =
			if let Splat::Werewolf(auspice, tribe, _, data) = &character.splat {
				let mut vec: Vec<Translated<HuntersAspect>> = if let Some(auspice) = auspice {
					vec![auspice.get_hunters_aspect().clone().into()]
				} else if let Some(Tribe::Pure(tribe)) = tribe {
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

				let mut col = column![text(fl!("werewolf", "hunters_aspect")),]
					.align_items(Alignment::Center)
					.spacing(TITLE_SPACING);

				if let Some(HuntersAspect::_Custom(name)) = &data.hunters_aspect {
					col = col.push(
						text_input("", name, |val| {
							Event::HuntersAspectChanged(HuntersAspect::_Custom(val))
						})
						.padding(INPUT_PADDING),
					);
				} else {
					col = col.push(
						pick_list(vec, data.hunters_aspect.clone().map(Into::into), |val| {
							Event::HuntersAspectChanged(val.unwrap())
						})
						.width(Length::Fill)
						.padding(INPUT_PADDING),
					);
				}

				col.into()
			} else {
				column![].into()
			};

		let mut col1 = Column::new()
			.align_items(Alignment::Center)
			.width(Length::Fill)
			.spacing(COMPONENT_SPACING);

		let mut col2 = Column::new()
			.push(health)
			.push(willpower)
			// .push(st)
			// .push(fuel)
			// .push(integrity)
			.spacing(COMPONENT_SPACING)
			.align_items(Alignment::Center)
			.width(Length::Fill);

		match &character.splat {
			Splat::Mortal => {}
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
					// Keys
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
				info_bar(self.character.clone(), || Event::Msg),
				attribute_bar(self.character.clone(), Event::AttrChanged)
			]
			.align_items(Alignment::Center)
			.width(Length::Fill),
			row![
				skills_component(
					self.character.clone(),
					Event::SkillChanged,
					Event::RoteSkillChanged,
					Event::SpecialtyChanged,
				),
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
			// 	Event::LocaleChanged
			// )
		]
		// .spacing(10)
		.width(Length::Fill)
		.into()
	}
}

impl<'a, Message> From<OverviewTab<Message>> for Element<'a, Message>
where
	Message: 'a + Clone,
{
	fn from(overview_tab: OverviewTab<Message>) -> Self {
		iced_lazy::component(overview_tab)
	}
}
