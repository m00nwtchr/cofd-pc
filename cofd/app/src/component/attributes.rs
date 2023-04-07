use iced::{
	widget::{column, row, text, Column},
	Alignment, Length,
};
use iced_lazy::Component;
use std::{cell::RefCell, rc::Rc};

use cofd::{
	character::{
		modifier::ModifierTarget,
		traits::{AttributeCategory, TraitCategory},
	},
	prelude::*,
};

use crate::{
	fl,
	i18n::flt,
	widget::dots::{Shape, SheetDots},
	Element, H2_SIZE, TITLE_SPACING,
};

pub struct AttributeBar<Message> {
	// attributes: Attributes,
	character: Rc<RefCell<Character>>,
	on_change: Box<dyn Fn(u16, Attribute) -> Message>,
}

pub fn attribute_bar<Message>(
	// attributes: Attributes,
	character: Rc<RefCell<Character>>,
	on_change: impl Fn(u16, Attribute) -> Message + 'static,
) -> AttributeBar<Message> {
	AttributeBar::new(character, on_change)
}

#[derive(Clone)]
pub struct Event(u16, Attribute);

impl<Message> AttributeBar<Message> {
	fn new(
		// attributes: Attributes,
		character: Rc<RefCell<Character>>,
		on_change: impl Fn(u16, Attribute) -> Message + 'static,
	) -> Self {
		Self {
			// attributes,
			character,
			on_change: Box::new(on_change),
		}
	}

	fn mk_attr_col(&self, character: &Character, cat: TraitCategory) -> Element<Event> {
		let mut col1 = Column::new().spacing(3);
		let mut col2 = Column::new()
			.spacing(5)
			.width(Length::Fill)
			.align_items(Alignment::End);

		let base_attributes = character.base_attributes();
		for attr in Attribute::get(AttributeCategory::Trait(cat)) {
			let v = base_attributes.get(&attr);
			let val = character._modified(ModifierTarget::BaseAttribute(attr));
			let mod_ = val - v;

			col1 = col1.push(text(flt("attribute", Some(attr.name())).unwrap()));
			col2 = col2.push(SheetDots::new(
				val,
				1 + mod_,
				5,
				Shape::Dots,
				None,
				move |val| Event(val - mod_, attr),
			));
		}

		row![col1, col2]
			.width(Length::FillPortion(2))
			.spacing(5)
			.into()
	}
}

impl<Message> Component<Message, iced::Renderer> for AttributeBar<Message> {
	type State = ();
	type Event = Event;

	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		Some((self.on_change)(event.0, event.1))
	}

	fn view(&self, _state: &Self::State) -> Element<Self::Event> {
		let character = self.character.borrow();

		column![
			text(fl!("attributes")).size(H2_SIZE),
			row![
				column![
					text(fl!("attribute", "power")),
					text(fl!("attribute", "finesse")),
					text(fl!("attribute", "resistance"))
				]
				.spacing(3)
				.width(Length::Fill)
				.align_items(Alignment::End),
				self.mk_attr_col(&character, TraitCategory::Mental),
				self.mk_attr_col(&character, TraitCategory::Physical),
				self.mk_attr_col(&character, TraitCategory::Social),
				column![].width(Length::Fill)
			]
			.spacing(10)
		]
		.spacing(TITLE_SPACING)
		.align_items(Alignment::Center)
		.into()
	}
}

impl<'a, Message> From<AttributeBar<Message>> for Element<'a, Message>
where
	Message: 'a,
{
	fn from(info_bar: AttributeBar<Message>) -> Self {
		iced_lazy::component(info_bar)
	}
}
