use iced::{
	widget::{checkbox, column, row, text, Column, Row},
	Alignment, Length,
};
use iced_lazy::Component;
use iced_native::Element;

use cofd::{
	character::TraitCategory,
	prelude::{Skill, Skills},
	splat::{mage::Order, Splat},
};

use crate::{
	fl as flt,
	i18n::fl,
	widget::{
		self,
		dots::{Shape, SheetDots},
	},
	H2_SIZE, H3_SIZE,
};

pub struct SkillsComponent<Message> {
	skills: Skills,
	splat: Splat,
	on_change: Box<dyn Fn(u16, Skill) -> Message>,
	on_rote_change: Box<dyn Fn(Skill) -> Message>,
}

pub fn skills_component<Message>(
	skills: Skills,
	splat: Splat,
	on_change: impl Fn(u16, Skill) -> Message + 'static,
	on_rote_change: impl Fn(Skill) -> Message + 'static,
) -> SkillsComponent<Message> {
	SkillsComponent::new(skills, splat, on_change, on_rote_change)
}

#[derive(Clone)]
pub enum Event {
	SkillChanged(u16, Skill),
	RoteSkillChanged(Skill),
}

impl<Message> SkillsComponent<Message> {
	fn new(
		skills: Skills,
		splat: Splat,
		on_change: impl Fn(u16, Skill) -> Message + 'static,
		on_rote_change: impl Fn(Skill) -> Message + 'static,
	) -> Self {
		Self {
			skills,
			splat,
			on_change: Box::new(on_change),
			on_rote_change: Box::new(on_rote_change),
		}
	}

	fn mk_skill_col<Renderer>(&self, cat: &TraitCategory) -> Element<Event, Renderer>
	where
		Renderer: iced_native::text::Renderer + 'static,
		Renderer::Theme: iced::widget::text::StyleSheet
			+ widget::dots::StyleSheet
			+ iced::widget::checkbox::StyleSheet,
	{
		let mut col0 = Column::new().spacing(3);
		let mut col1 = Column::new().width(Length::Fill).spacing(3);
		let mut col2 = Column::new()
			.spacing(4)
			.width(Length::Fill)
			.align_items(Alignment::End);

		for skill in Skill::get(cat) {
			if let Splat::Mage(_, order, _, data) = &self.splat {
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

			col1 = col1.push(text(fl("skill", Some(skill.name())).unwrap()));

			let v = self.skills.get(&skill);
			col2 = col2.push(SheetDots::new(*v, 0, 5, Shape::Dots, None, move |val| {
				Event::SkillChanged(val, skill.clone())
			}));
		}

		column![
			text(fl(cat.name(), None).unwrap()).size(H3_SIZE),
			text(flt!("unskilled", num = cat.unskilled())).size(17),
			row![col0, col1, col2].spacing(5)
		]
		.align_items(Alignment::Center)
		.into()
	}
}

impl<Message, Renderer> Component<Message, Renderer> for SkillsComponent<Message>
where
	Renderer: iced_native::text::Renderer + 'static,
	Renderer::Theme: iced::widget::text::StyleSheet
		+ widget::dots::StyleSheet
		+ iced::widget::checkbox::StyleSheet,
{
	type State = ();
	type Event = Event;

	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		match event {
			Event::SkillChanged(val, skill) => Some((self.on_change)(val, skill)),
			Event::RoteSkillChanged(skill) => Some((self.on_rote_change)(skill)),
		}
	}

	fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
		column![
			text(flt!("skills").to_uppercase()).size(H2_SIZE),
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
}

impl<'a, Message, Renderer> From<SkillsComponent<Message>> for Element<'a, Message, Renderer>
where
	Message: 'a,
	Renderer: 'static + iced_native::text::Renderer,
	Renderer::Theme: iced::widget::text::StyleSheet
		+ widget::dots::StyleSheet
		+ iced::widget::checkbox::StyleSheet,
{
	fn from(info_bar: SkillsComponent<Message>) -> Self {
		iced_lazy::component(info_bar)
	}
}
