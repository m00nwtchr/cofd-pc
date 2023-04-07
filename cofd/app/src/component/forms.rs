use iced::{
	alignment,
	widget::{button, column, row, text, text_input, Column, Row},
	Length,
};
use iced_lazy::Component;
use std::{cell::RefCell, rc::Rc};

use crate::{i18n::flt, Element, INPUT_PADDING};
use cofd::{
	character::{
		modifier::{Modifier, ModifierTarget},
		traits::Trait,
	},
	prelude::*,
	splat::{
		werewolf::{get_form_trait, Form},
		Splat,
	},
};

pub struct FormsComponent<Message> {
	character: Rc<RefCell<Character>>,
	// on_change: Box<dyn Fn(u16, Trait) -> Message>,
	on_change: Message,
}

pub fn forms_component<Message>(
	character: Rc<RefCell<Character>>,
	// on_change: impl Fn(u16, Trait) -> Message + 'static,
	on_change: Message,
) -> FormsComponent<Message> {
	FormsComponent::new(character, on_change)
}

#[derive(Clone)]
pub enum Event {
	FormChanged(Form),

	Msg,
}

impl<Message> FormsComponent<Message> {
	fn new(character: Rc<RefCell<Character>>, on_change: Message) -> Self {
		Self {
			character,
			on_change,
		}
	}

	fn mk_col(&self, form: Form, character: &Character, active_form: &Form) -> Element<Event> {
		let attrs = character.attributes();

		let mut col = Column::new();

		let cur_mods = active_form.get_modifiers();
		let mods = form.get_modifiers();

		let mut vec: Vec<ModifierTarget> = vec![
			Attribute::Strength.into(),
			Attribute::Dexterity.into(),
			Attribute::Stamina.into(),
			Attribute::Manipulation.into(),
		];

		vec.extend(vec![
			ModifierTarget::Trait(Trait::Size),
			ModifierTarget::Trait(Trait::Defense),
			ModifierTarget::Trait(Trait::Initative),
			ModifierTarget::Trait(Trait::Speed),
			// ModifierTarget::Trait(Trait::Armor(None)),
			ModifierTarget::Trait(Trait::Perception),
		]);

		for target in vec {
			let name = match target {
				ModifierTarget::BaseAttribute(_)
				| ModifierTarget::BaseSkill(_)
				| ModifierTarget::Skill(_)
				| ModifierTarget::WerewolfForm(..) => unreachable!(),
				ModifierTarget::Attribute(attr) => flt("attribute", Some(attr.name())).unwrap(),
				ModifierTarget::Trait(trait_) => flt(trait_.name().unwrap(), None).unwrap(),
			};

			let val = get_form_trait(character, &form, &target);

			col = col.push(row![
				text(format!("{name}: ")),
				text_input("", &val.to_string(), |_val| Event::Msg).padding(INPUT_PADDING)
			]);
		}

		column![
			button(
				text(flt("werewolf", Some(form.name())).unwrap())
					.width(Length::Fill)
					.horizontal_alignment(alignment::Horizontal::Center)
			)
			.on_press(Event::FormChanged(form))
			.width(Length::Fill),
			col
		]
		.width(Length::Fill)
		.padding(4)
		.into()
	}
}

impl<Message: Clone> Component<Message, iced::Renderer> for FormsComponent<Message> {
	type State = ();
	type Event = Event;

	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		let mut character = self.character.borrow_mut();

		if let Splat::Werewolf(.., data) = &mut character.splat {
			match event {
				Event::FormChanged(form) => {
					data.form = form;
					character.calc_mod_map();
					Some(self.on_change.clone())
				}
				Event::Msg => None,
			}
		} else {
			None
		}
	}

	fn view(&self, _state: &Self::State) -> Element<Self::Event> {
		let character = self.character.borrow();

		let mut row = Row::new();

		if let Splat::Werewolf(.., data) = &character.splat {
			for form in Form::all() {
				row = row.push(self.mk_col(form, &character, &data.form));
			}
		}

		row.into()
	}
}

impl<'a, Message> From<FormsComponent<Message>> for Element<'a, Message>
where
	Message: 'a + Clone,
{
	fn from(forms_component: FormsComponent<Message>) -> Self {
		iced_lazy::component(forms_component)
	}
}
