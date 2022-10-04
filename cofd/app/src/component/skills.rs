use iced::{
	widget::{column, row, text, Column},
	Alignment, Length,
};
use iced_lazy::Component;
use iced_native::Element;

use cofd::{
	character::TraitCategory,
	prelude::{Skill, Skills},
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
	on_change: Box<dyn Fn(u8, Skill) -> Message>,
}

pub fn skills_component<Message>(
	skills: Skills,
	on_change: impl Fn(u8, Skill) -> Message + 'static,
) -> SkillsComponent<Message> {
	SkillsComponent::new(skills, on_change)
}

#[derive(Clone)]
pub struct Event(u8, Skill);

impl<Message> SkillsComponent<Message> {
	fn new(skills: Skills, on_change: impl Fn(u8, Skill) -> Message + 'static) -> Self {
		Self {
			skills,
			on_change: Box::new(on_change),
		}
	}

	fn mk_skill_col<Renderer>(&self, cat: &TraitCategory) -> Element<Event, Renderer>
	where
		Renderer: iced_native::text::Renderer + 'static,
		Renderer::Theme: iced::widget::text::StyleSheet + widget::dots::StyleSheet,
	{
		let mut col1 = Column::new().spacing(3);
		let mut col2 = Column::new()
			.spacing(4)
			.width(Length::Fill)
			.align_items(Alignment::End);

		for skill in Skill::get(cat) {
			col1 = col1.push(text(fl("skill", Some(skill.name()))));

			let v = self.skills.get(&skill);
			col2 = col2.push(SheetDots::new(*v, 0, 5, Shape::Dots, None, move |val| {
				Event(val, skill.clone())
			}));
		}

		column![
			text(fl(cat.name(), None)).size(H3_SIZE),
			text(flt!("unskilled", num = cat.unskilled())).size(17),
			row![col1, col2].spacing(5)
		]
		.align_items(Alignment::Center)
		.into()
	}
}

impl<Message, Renderer> Component<Message, Renderer> for SkillsComponent<Message>
where
	Renderer: iced_native::text::Renderer + 'static,
	Renderer::Theme: iced::widget::text::StyleSheet + widget::dots::StyleSheet,
{
	type State = ();
	type Event = Event;

	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		Some((self.on_change)(event.0, event.1))
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
	Renderer::Theme: iced::widget::text::StyleSheet + widget::dots::StyleSheet,
{
	fn from(info_bar: SkillsComponent<Message>) -> Self {
		iced_lazy::component(info_bar)
	}
}
