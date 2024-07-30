use std::{cell::RefCell, rc::Rc};

use cofd::splat::mage::Mage;
use cofd::{character::modifier::ModifierTarget, prelude::*, splat::Splat};
use iced::widget::{component, Component};
use iced::{
	theme::{self},
	widget::{button, checkbox, column, row, text, text_input, Column},
	Alignment, Color, Element, Length,
};

use super::list;
use crate::{
	fl,
	widget::dots::{self, Shape, SheetDots},
	H2_SIZE, H3_SIZE, TITLE_SPACING,
};
use crate::i18n::Translate;

pub struct SkillsComponent<Message> {
	character: Rc<RefCell<Character>>,
	on_change: Box<dyn Fn(u16, Skill) -> Message>,
	on_rote_change: Box<dyn Fn(Skill) -> Message>,
	on_specialty_change: Box<dyn Fn(Skill, usize, String) -> Message>,
}

pub fn skills_component<Message>(
	character: Rc<RefCell<Character>>,
	on_change: impl Fn(u16, Skill) -> Message + 'static,
	on_rote_change: impl Fn(Skill) -> Message + 'static,
	on_specialty_change: impl Fn(Skill, usize, String) -> Message + 'static,
) -> SkillsComponent<Message> {
	SkillsComponent::new(character, on_change, on_rote_change, on_specialty_change)
}

#[derive(Clone)]
pub enum Event {
	Skill(u16, Skill),
	RoteSkill(Skill),
	Specialty(Skill, usize, String),
	SpecialtySkill(Skill),
}

#[derive(Default, Clone)]
pub struct State {
	specialty_skill: Option<Skill>,
}

impl<Message> SkillsComponent<Message> {
	fn new(
		character: Rc<RefCell<Character>>,
		on_change: impl Fn(u16, Skill) -> Message + 'static,
		on_rote_change: impl Fn(Skill) -> Message + 'static,
		on_specialty_change: impl Fn(Skill, usize, String) -> Message + 'static,
	) -> Self {
		Self {
			character,
			on_change: Box::new(on_change),
			on_rote_change: Box::new(on_rote_change),
			on_specialty_change: Box::new(on_specialty_change),
		}
	}

	fn mk_skill_col<Theme>(
		&self,
		state: &State,
		character: &Character,
		category: TraitCategory,
	) -> Element<Event, Theme>
	where
		Theme: 'static
			+ button::StyleSheet
			+ dots::StyleSheet
			+ text::StyleSheet
			+ text_input::StyleSheet
			+ checkbox::StyleSheet,
		<Theme as text::StyleSheet>::Style: From<theme::Text>,
		<Theme as button::StyleSheet>::Style: From<theme::Button>,
	{
		let mut col = Column::new();

		let mut col0 = Column::new().spacing(3);
		let mut col1 = Column::new().width(Length::Fill).spacing(3);
		let mut col2 = Column::new()
			.spacing(4)
			.width(Length::Fill)
			.align_items(Alignment::End);

		for skill in Skill::get_by_category(category) {
			if let Splat::Mage(Mage { order, .. }) = &character.splat {
				let flag = if let Some(order) = order {
					order.get_rote_skills().contains(&skill)
				} else {
					false
				};

				col0 = col0.push(
					checkbox("", flag)
						.on_toggle(move |_| Event::RoteSkill(skill))
						.spacing(0),
				);
			}

			let specialties = if let Some(specialties) = character.specialties.get(&skill) {
				specialties
			} else {
				lazy_static::lazy_static! {
					static ref DEFAULT: Vec<String> = vec![];
				}
				&DEFAULT
			};

			col1 = col1.push(
				button(text(skill.translated()).style(
					if specialties.is_empty() {
						theme::Text::Default
					} else {
						theme::Text::Color(Color::from_rgb(0.0, 0.7, 0.0))
					},
				))
				.padding(0)
				.style(theme::Button::Text)
				.on_press(Event::SpecialtySkill(skill)),
			);

			let v = character.base_skills().get(skill);
			let val = character._modified(ModifierTarget::BaseSkill(skill));
			let mod_ = val - v;

			col2 = col2.push(SheetDots::new(
				val,
				mod_,
				5,
				Shape::Dots,
				None,
				move |val| Event::Skill(val - mod_, skill),
			));

			if let Some(specialty_skill) = state.specialty_skill {
				if skill.eq(&specialty_skill) {
					col = col.push(row![col0, col1, col2].spacing(5)).push(list(
						String::new(),
						Some(specialties.len() + 1),
						None,
						specialties.clone(),
						move |i, val| {
							text_input("", &val.unwrap_or_default())
								.on_input(move |val| Event::Specialty(skill, i, val))
								.padding(0)
								.into()
						},
					));

					col0 = Column::new().spacing(3);
					col1 = Column::new().width(Length::Fill).spacing(3);
					col2 = Column::new()
						.spacing(4)
						.width(Length::Fill)
						.align_items(Alignment::End);
				}
			}
		}

		col = col.push(row![col0, col1, col2].spacing(5));

		column![
			text(category.translated()).size(H3_SIZE),
			text(fl!("unskilled", num = category.unskilled())).size(13),
			col
		]
		.spacing(TITLE_SPACING)
		.align_items(Alignment::Center)
		.into()
	}
}

impl<Message, Theme> Component<Message, Theme> for SkillsComponent<Message>
where
	Theme: 'static
		+ button::StyleSheet
		+ dots::StyleSheet
		+ text::StyleSheet
		+ text_input::StyleSheet
		+ checkbox::StyleSheet,
	<Theme as text::StyleSheet>::Style: From<theme::Text>,
	<Theme as button::StyleSheet>::Style: From<theme::Button>,
{
	type State = State;
	type Event = Event;

	fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
		match event {
			Event::Skill(val, skill) => Some((self.on_change)(val, skill)),
			Event::RoteSkill(skill) => Some((self.on_rote_change)(skill)),
			Event::Specialty(skill, i, val) => Some((self.on_specialty_change)(skill, i, val)),
			Event::SpecialtySkill(skill) => {
				if let Some(cur) = state.specialty_skill
					&& cur == skill
				{
					state.specialty_skill = None;
					None
				} else {
					state.specialty_skill = Some(skill);
					None
				}
			}
		}
	}

	fn view(&self, state: &Self::State) -> Element<Event, Theme> {
		let character = self.character.borrow();

		column![
			text(fl!("skills").to_uppercase()).size(H2_SIZE),
			self.mk_skill_col(state, &character, TraitCategory::Mental),
			self.mk_skill_col(state, &character, TraitCategory::Physical),
			self.mk_skill_col(state, &character, TraitCategory::Social),
		]
		.spacing(10)
		// .padding(15)
		.align_items(Alignment::Center)
		.width(Length::Fill)
		.into()
	}
}

impl<'a, Message, Theme> From<SkillsComponent<Message>> for Element<'a, Message, Theme>
where
	Message: 'a,
	Theme: 'static
		+ button::StyleSheet
		+ dots::StyleSheet
		+ text::StyleSheet
		+ text_input::StyleSheet
		+ checkbox::StyleSheet,
	<Theme as text::StyleSheet>::Style: From<theme::Text>,
	<Theme as button::StyleSheet>::Style: From<theme::Button>,
{
	fn from(info_bar: SkillsComponent<Message>) -> Self {
		component(info_bar)
	}
}
