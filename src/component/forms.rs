use crate::{fl, i18n::Translate, Element, INPUT_PADDING};
use cofd::{
	character::modifier::ModifierTarget,
	prelude::*,
	splat::{
		werewolf::{get_form_trait, Form},
		Splat,
	},
	traits::DerivedTrait,
};
use iced::{
	alignment,
	widget::{button, column, row, text, text_input, Column, Row},
	Length,
};

#[derive(Debug, Clone)]
pub struct FormsComponent;

#[derive(Clone)]
pub enum Message {
	FormChanged(Form),

	Msg,
}

impl FormsComponent {
	pub fn new() -> Self {
		Self {}
	}

	pub fn update(&mut self, message: Message, character: &mut Character) {
		if let Splat::Werewolf(data) = &mut character.splat {
			match message {
				Message::FormChanged(form) => {
					data.form = form;
					character.calc_mod_map();
				}
				Message::Msg => {}
			}
		}
	}

	pub fn view(&self, character: &Character) -> Element<Message> {
		let mut row = Row::new();

		if let Splat::Werewolf(..) = &character.splat {
			for form in Form::all() {
				row = row.push(self.mk_col(form, &character));
			}
		}

		row.into()
	}

	fn mk_col(&self, form: Form, character: &Character) -> Element<Message> {
		let mut col = Column::new();

		let mut vec: Vec<ModifierTarget> = vec![
			Attribute::Strength.into(),
			Attribute::Dexterity.into(),
			Attribute::Stamina.into(),
			Attribute::Manipulation.into(),
		];

		vec.extend(vec![
			ModifierTarget::Trait(Trait::DerivedTrait(DerivedTrait::Size)),
			ModifierTarget::Trait(Trait::DerivedTrait(DerivedTrait::Defense)),
			ModifierTarget::Trait(Trait::DerivedTrait(DerivedTrait::Initiative)),
			ModifierTarget::Trait(Trait::DerivedTrait(DerivedTrait::Speed)),
			// ModifierTarget::Trait(Trait::Armor(None)),
			ModifierTarget::Trait(Trait::DerivedTrait(DerivedTrait::Initiative)),
		]);

		for target in vec {
			let name = match target {
				ModifierTarget::BaseAttribute(_)
				| ModifierTarget::BaseSkill(_)
				| ModifierTarget::Skill(_) => unreachable!(),
				ModifierTarget::Attribute(attr) => attr.translated(),
				ModifierTarget::Trait(trait_) => trait_.translated(),
			};

			let val = get_form_trait(character, &form, &target);

			col = col.push(row![
				text(format!("{name}: ")),
				text_input("", &val.to_string())
					.on_input(|_val| Message::Msg)
					.padding(INPUT_PADDING)
			]);
		}

		column![
			button(
				text(form.translated())
					.width(Length::Fill)
					.horizontal_alignment(alignment::Horizontal::Center)
			)
			.on_press(Message::FormChanged(form))
			.width(Length::Fill),
			col
		]
		.width(Length::Fill)
		.padding(4)
		.into()
	}
}
