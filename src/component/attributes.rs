use iced::{
	widget::{column, row, text, Column},
	Alignment, Length,
};

use cofd::{character::modifier::ModifierTarget, prelude::TraitCategory, prelude::*};

use crate::i18n::Translate;
use crate::{
	fl,
	widget::dots::{Shape, SheetDots},
	Element, H2_SIZE, TITLE_SPACING,
};

#[derive(Debug, Clone)]
pub struct AttributeBar;

#[derive(Clone)]
pub struct Message(u16, Attribute);

impl AttributeBar {
	pub fn new() -> Self {
		Self
	}

	pub fn update(&mut self, message: Message, character: &mut Character) {
		let Message(val, attr) = message;
		*character.base_attributes_mut().get_mut(&attr) = val;
	}

	pub fn view(&self, character: &Character) -> Element<Message> {
		column![
			text(fl!("attributes")).size(H2_SIZE),
			row![
				column![
					text(fl!("power")),
					text(fl!("finesse")),
					text(fl!("resistance"))
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

	fn mk_attr_col(&self, character: &Character, category: TraitCategory) -> Element<Message> {
		let mut col1 = Column::new().spacing(3);
		let mut col2 = Column::new()
			.spacing(5)
			.width(Length::Fill)
			.align_items(Alignment::End);

		let base_attributes = character.base_attributes();
		for attr in Attribute::get_by_category(category) {
			let v = base_attributes.get(&attr);
			let val = character._modified(ModifierTarget::BaseAttribute(attr));
			let mod_ = val - v;

			col1 = col1.push(text(attr.translated()));
			col2 = col2.push(SheetDots::new(
				val,
				1 + mod_,
				5,
				Shape::Dots,
				None,
				move |val| Message(val - mod_, attr),
			));
		}

		row![col1, col2]
			.width(Length::FillPortion(2))
			.spacing(5)
			.into()
	}
}
