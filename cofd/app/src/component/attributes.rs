use iced::{
	widget::{column, row, text, Column},
	Alignment, Length,
};
use iced_lazy::Component;
use iced_native::Element;

use cofd::{
	character::{AttributeCategory, TraitCategory},
	prelude::{Attribute, Attributes},
};

use crate::{
	fl as flt,
	i18n::fl,
	widget::{
		self,
		dots::{Shape, SheetDots},
	},
	H2_SIZE,
};

pub struct AttributeBar<Message> {
	attributes: Attributes,
	on_change: Box<dyn Fn(u8, Attribute) -> Message>,
}

pub fn attribute_bar<Message>(
	attributes: Attributes,
	on_change: impl Fn(u8, Attribute) -> Message + 'static,
) -> AttributeBar<Message> {
	AttributeBar::new(attributes, on_change)
}

#[derive(Clone)]
#[allow(clippy::enum_variant_names)]
pub struct Event(u8, Attribute);

impl<Message> AttributeBar<Message> {
	fn new(attributes: Attributes, on_change: impl Fn(u8, Attribute) -> Message + 'static) -> Self {
		Self {
			attributes,
			on_change: Box::new(on_change),
		}
	}

	fn mk_attr_col<Renderer>(&self, cat: TraitCategory) -> Element<Event, Renderer>
	where
		Renderer: iced_native::text::Renderer + 'static,
		Renderer::Theme: iced::widget::text::StyleSheet + widget::dots::StyleSheet,
	{
		let mut col1 = Column::new().spacing(3);
		let mut col2 = Column::new()
			.spacing(5)
			.width(Length::Fill)
			.align_items(Alignment::End);

		for attr in Attribute::get(AttributeCategory::Trait(cat)) {
			let v = self.attributes.get(&attr) as u8;

			col1 = col1.push(text(fl("attribute", Some(attr.name()))));
			col2 = col2.push(SheetDots::new(v, 1, 5, Shape::Dots, None, move |val| {
				Event(val, attr)
			}));
		}

		row![col1, col2]
			.width(Length::FillPortion(2))
			.spacing(5)
			.into()
	}
}

impl<Message, Renderer> Component<Message, Renderer> for AttributeBar<Message>
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
			text(flt!("attributes")).size(H2_SIZE),
			row![
				column![
					text(fl("attribute", Some("power"))),
					text(fl("attribute", Some("finesse"))),
					text(fl("attribute", Some("resistance")))
				]
				.spacing(3)
				.width(Length::Fill)
				.align_items(Alignment::End),
				self.mk_attr_col(TraitCategory::Mental),
				self.mk_attr_col(TraitCategory::Physical),
				self.mk_attr_col(TraitCategory::Social),
				column![].width(Length::Fill)
			]
			.spacing(10)
		]
		.align_items(Alignment::Center)
		.into()
	}
}

impl<'a, Message, Renderer> From<AttributeBar<Message>> for Element<'a, Message, Renderer>
where
	Message: 'a,
	Renderer: 'static + iced_native::text::Renderer,
	Renderer::Theme: iced::widget::text::StyleSheet + widget::dots::StyleSheet,
{
	fn from(info_bar: AttributeBar<Message>) -> Self {
		iced_lazy::component(info_bar)
	}
}
