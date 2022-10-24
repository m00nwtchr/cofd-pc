use std::{cell::RefCell, rc::Rc};

use iced::{
	theme::{self},
	widget::{button, checkbox, column, row, text, text_input, Column},
	Alignment, Length,
};
use iced_lazy::Component;

use cofd::{
	character::{ModifierTarget, TraitCategory},
	prelude::{Character, Skill},
	splat::Splat,
};

use crate::{
	fl as flt,
	i18n::fl,
	widget::dots::{Shape, SheetDots},
	Element, H2_SIZE, H3_SIZE, TITLE_SPACING,
};

use super::list;

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
	SkillChanged(u16, Skill),
	RoteSkillChanged(Skill),
	SpecialtyChanged(Skill, usize, String),
	SpecialtySkillChanged(Skill),
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

	fn mk_skill_col(
		&self,
		state: &State,
		character: &Character,
		cat: &TraitCategory,
	) -> Element<Event> {
		let mut col = Column::new();

		let mut col0 = Column::new().spacing(3);
		let mut col1 = Column::new().width(Length::Fill).spacing(3);
		let mut col2 = Column::new()
			.spacing(4)
			.width(Length::Fill)
			.align_items(Alignment::End);

		for skill in Skill::get(cat) {
			if let Splat::Mage(_, order, _, _) = &character.splat {
				let flag = if let Some(order) = order {
					order.get_rote_skills().contains(&skill)
				} else {
					false
				};

				col0 = col0.push(
					checkbox("", flag, {
						let skill = skill.clone();
						move |_| Event::RoteSkillChanged(skill.clone())
					})
					.spacing(0),
				);
			}

			col1 = col1.push(
				button(text(fl("skill", Some(skill.name())).unwrap()))
					.padding(0)
					.style(theme::Button::Text)
					.on_press(Event::SpecialtySkillChanged(skill)),
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
				move |val| Event::SkillChanged(val - mod_, skill.clone()),
			));

			if let Some(specialty_skill) = state.specialty_skill && skill.eq(&specialty_skill) {
				let specialties = character.specialties.get(&skill).cloned().unwrap_or(vec![]);

				col = col.push(row![col0, col1, col2].spacing(5))
					.push(list("".to_string(), specialties.len()+1, specialties, {
						let skill = skill.clone();
						move |i, val| text_input("", &val.unwrap_or_default(), move |val| Event::SpecialtyChanged(skill, i, val)).padding(0).into()
					}));

				col0 = Column::new().spacing(3);
				col1 = Column::new().width(Length::Fill).spacing(3);
				col2 = Column::new()
					.spacing(4)
					.width(Length::Fill)
					.align_items(Alignment::End);
			}
		}

		col = col.push(row![col0, col1, col2].spacing(5));

		column![
			text(fl(cat.name(), None).unwrap()).size(H3_SIZE),
			text(flt!("unskilled", num = cat.unskilled())).size(17),
			col
		]
		.spacing(TITLE_SPACING)
		.align_items(Alignment::Center)
		.into()
	}
}

impl<Message> Component<Message, iced::Renderer> for SkillsComponent<Message> {
	type State = State;
	type Event = Event;

	fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
		match event {
			Event::SkillChanged(val, skill) => Some((self.on_change)(val, skill)),
			Event::RoteSkillChanged(skill) => Some((self.on_rote_change)(skill)),
			Event::SpecialtyChanged(skill, i, val) =>  Some((self.on_specialty_change)(skill, i, val)),
			Event::SpecialtySkillChanged(skill) => if let Some(cur) = state.specialty_skill && cur == skill {
				state.specialty_skill = None;
				None
			} else {
				state.specialty_skill = Some(skill);
				None
			}
		}
	}

	fn view(&self, state: &Self::State) -> Element<Self::Event> {
		let character = self.character.borrow();

		column![
			text(flt!("skills").to_uppercase()).size(H2_SIZE),
			self.mk_skill_col(state, &character, &TraitCategory::Mental),
			self.mk_skill_col(state, &character, &TraitCategory::Physical),
			self.mk_skill_col(state, &character, &TraitCategory::Social),
		]
		.spacing(10)
		// .padding(15)
		.align_items(Alignment::Center)
		.width(Length::Fill)
		.into()
	}
}

impl<'a, Message> From<SkillsComponent<Message>> for Element<'a, Message>
where
	Message: 'a,
{
	fn from(info_bar: SkillsComponent<Message>) -> Self {
		iced_lazy::component(info_bar)
	}
}
