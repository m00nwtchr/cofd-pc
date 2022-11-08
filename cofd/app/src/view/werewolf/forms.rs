use std::{cell::RefCell, rc::Rc};

use iced::{
	alignment,
	widget::{button, column, text, text_input, Column, Row},
	Element, Length,
};
use iced_lazy::Component;
use iced_native::row;

use crate::{fl, INPUT_PADDING};
use cofd::{
	character::{ModifierTarget, Trait},
	prelude::*,
	splat::{werewolf::Form, Splat},
};

pub struct FormTab<Message> {
	character: Rc<RefCell<Character>>,
	on_change: Message,
}

pub fn form_tab<Message>(
	character: Rc<RefCell<Character>>,
	on_change: Message,
) -> FormTab<Message> {
	FormTab::new(character, on_change)
}

#[derive(Clone)]
pub enum Event {
	FormChanged(Form),

	Msg,
}

impl<Message> FormTab<Message> {
	pub fn new(character: Rc<RefCell<Character>>, on_change: Message) -> Self {
		Self {
			character,
			on_change,
		}
	}

	fn mk_col<Renderer>(
		&self,
		form: Form,
		character: &Character,
		current_form: &Form,
	) -> Element<Event, Renderer>
	where
		Renderer: iced_native::text::Renderer + 'static,
		Renderer::Theme: iced::widget::text::StyleSheet
			+ iced::widget::text_input::StyleSheet
			+ iced::widget::button::StyleSheet,
	{
		let attrs = character.attributes();

		let mut col = Column::new();

		let cur_mods = current_form.get_modifiers();
		let mods = form.get_modifiers();

		let mut vec: Vec<ModifierTarget> = vec![
			Attribute::Strength.into(),
			Attribute::Dexterity.into(),
			Attribute::Stamina.into(),
			Attribute::Manipulation.into(),
		];

		let iter: Vec<_> = mods
			.iter()
			.filter_map(|mod_| {
				if let ModifierTarget::Attribute(_) = mod_.target && !vec.contains(&mod_.target) {
				Some(mod_.target)
			} else {
				None
			}
			})
			.collect();

		vec.extend(iter);
		vec.extend(vec![
			ModifierTarget::Trait(Trait::Size),
			ModifierTarget::Trait(Trait::Defense),
			ModifierTarget::Trait(Trait::Initative),
			ModifierTarget::Trait(Trait::Speed),
			// ModifierTarget::Trait(Trait::Armor(None)),
			ModifierTarget::Trait(Trait::Perception),
		]);

		for target in vec {
			let (base, name) = match target {
				ModifierTarget::BaseAttribute(_)
				| ModifierTarget::BaseSkill(_)
				| ModifierTarget::Skill(_) => unreachable!(),
				ModifierTarget::Attribute(attr) => (
					*attrs.get(attr) as i16,
					fl("attribute", Some(attr.name())).unwrap(),
				),
				ModifierTarget::Trait(trait_) => (
					character.get_trait(trait_) as i16,
					fl(trait_.name().unwrap(), None).unwrap(),
				),
			};

			let val: i16 = if !form.eq(current_form) {
				let cur_mod_ = cur_mods
					.iter()
					.filter(|el| el.target.eq(&target))
					.find_map(|el| el.val())
					.unwrap_or(0);

				let mod_ = mods
					.iter()
					.filter(|el| el.target.eq(&target))
					.find_map(|el| el.val())
					.unwrap_or(0);

				base - cur_mod_ + mod_
			} else {
				base
			};

			col = col.push(row![
				text(format!("{}: ", name)),
				text_input("", &val.to_string(), |val| Event::Msg).padding(INPUT_PADDING)
			]);
		}

		column![
			button(
				text(fl("werewolf", Some(form.name())).unwrap())
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

impl<Message, Renderer> Component<Message, Renderer> for FormTab<Message>
where
	Message: Clone,
	Renderer: iced_native::text::Renderer + 'static,
	Renderer::Theme: iced::widget::text_input::StyleSheet
		+ iced::widget::text::StyleSheet
		+ iced::widget::button::StyleSheet,
{
	type State = ();

	type Event = Event;

	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		let mut character = self.character.borrow_mut();

		if let Splat::Werewolf(_, _, _, data) = &mut character.splat {
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

	fn view(&self, _state: &Self::State) -> iced_native::Element<Self::Event, Renderer> {
		let character = self.character.borrow();

		let mut row = Row::new();

		if let Splat::Werewolf(_, _, _, data) = &character.splat {
			for form in Form::all() {
				row = row.push(self.mk_col(form, &character, &data.form));
			}
		}

		row.into()
	}
}

impl<'a, Message, Renderer> From<FormTab<Message>> for Element<'a, Message, Renderer>
where
	Message: 'a + Clone,
	Renderer: 'static + iced_native::text::Renderer,
	Renderer::Theme: iced::widget::text_input::StyleSheet
		+ iced::widget::text::StyleSheet
		+ iced::widget::button::StyleSheet,
{
	fn from(form_tab: FormTab<Message>) -> Self {
		iced_lazy::component(form_tab)
	}
}
