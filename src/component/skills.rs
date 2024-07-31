use cofd::{
	character::modifier::ModifierTarget,
	prelude::*,
	splat::{
		mage::{Mage, Ministry, Order},
		Splat,
	},
};
use iced::{
	theme::{self},
	widget::{button, checkbox, column, row, text, text_input, Column},
	Alignment, Color, Element, Length,
};

use super::list;
use crate::{
	fl,
	i18n::Translate,
	view::overview::vec_changed,
	widget::dots::{Shape, SheetDots},
	H2_SIZE, H3_SIZE, TITLE_SPACING,
};

#[derive(Debug, Clone)]
pub struct SkillsComponent {
	specialty_skill: Option<Skill>,
}

#[derive(Clone)]
pub enum Message {
	Skill(u16, Skill),
	RoteSkill(Skill),
	Specialty(Skill, usize, String),
	SpecialtySkill(Skill),
}

impl SkillsComponent {
	pub fn new() -> Self {
		Self {
			specialty_skill: None,
		}
	}

	pub fn update(&mut self, message: Message, character: &mut Character) {
		match message {
			Message::Skill(val, skill) => *character.base_skills_mut().get_mut(skill) = val,
			Message::RoteSkill(skill) => {
				if let Splat::Mage(Mage {
					order:
						Some(
							Order::_Custom(_, rote_skills)
							| Order::SeersOfTheThrone(Some(Ministry::_Custom(_, rote_skills))),
						),
					..
				}) = &mut character.splat
				{
					if !rote_skills.contains(&skill) {
						rote_skills.rotate_left(1);
						rote_skills[2] = skill;
					}
				}
			}
			Message::Specialty(skill, i, val) => {
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
			Message::SpecialtySkill(skill) => {
				if let Some(cur) = self.specialty_skill
					&& cur == skill
				{
					self.specialty_skill = None;
				} else {
					self.specialty_skill = Some(skill);
				}
			}
		}
	}

	pub fn view(&self, character: &Character) -> Element<Message> {
		column![
			text(fl!("skills").to_uppercase()).size(H2_SIZE),
			self.mk_skill_col(character, TraitCategory::Mental),
			self.mk_skill_col(character, TraitCategory::Physical),
			self.mk_skill_col(character, TraitCategory::Social),
		]
		.spacing(10)
		// .padding(15)
		.align_items(Alignment::Center)
		.width(Length::Fill)
		.into()
	}

	fn mk_skill_col(&self, character: &Character, category: TraitCategory) -> Element<Message> {
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
						.on_toggle(move |_| Message::RoteSkill(skill))
						.spacing(0),
				);
			}

			let specialties = character
				.specialties
				.get(&skill)
				.cloned()
				.unwrap_or_default();

			col1 = col1.push(
				button(text(skill.translated()).style(if specialties.is_empty() {
					theme::Text::Default
				} else {
					theme::Text::Color(Color::from_rgb(0.0, 0.7, 0.0))
				}))
				.padding(0)
				.style(theme::Button::Text)
				.on_press(Message::SpecialtySkill(skill)),
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
				move |val| Message::Skill(val - mod_, skill),
			));

			if let Some(specialty_skill) = self.specialty_skill {
				if skill.eq(&specialty_skill) {
					col = col.push(row![col0, col1, col2].spacing(5)).push(list(
						String::new(),
						Some(specialties.len() + 1),
						None,
						specialties,
						move |i, val| {
							text_input("", &val.unwrap_or_default())
								.on_input(move |val| Message::Specialty(skill, i, val))
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
