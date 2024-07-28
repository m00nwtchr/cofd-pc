use iced::{
	widget::{column, row, text, text_input, Column},
	Alignment, Length,
};
use std::{cell::RefCell, rc::Rc};

use super::list;
use crate::widget::{dots, track};
use crate::{
	fl,
	i18n::flt,
	widget::{
		self,
		dots::{Shape, SheetDots},
		track::HealthTrack,
	},
	Element, COMPONENT_SPACING, H3_SIZE, INPUT_PADDING, MAX_INPUT_WIDTH, TITLE_SPACING,
};
use cofd::{character::Wound, prelude::*, splat::Splat};
use iced::widget::{component, Component};

pub struct IntegrityComponent {
	character: Rc<RefCell<Character>>,
}

pub fn integrity_component(character: Rc<RefCell<Character>>) -> IntegrityComponent {
	IntegrityComponent::new(character)
}

#[derive(Clone)]
pub enum Event {
	IntegrityChanged(u16),
	IntegrityDamage(Wound),
	TouchstoneChanged(usize, String),
}

impl IntegrityComponent {
	fn new(character: Rc<RefCell<Character>>) -> Self {
		Self { character }
	}
}

impl<Message, Theme> Component<Message, Theme> for IntegrityComponent
where
	Theme:
		text::StyleSheet + text_input::StyleSheet + dots::StyleSheet + track::StyleSheet + 'static,
{
	type State = ();
	type Event = Event;

	fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Message> {
		let mut character = self.character.borrow_mut();

		match event {
			Event::IntegrityChanged(val) => character.integrity = val,
			Event::IntegrityDamage(wound) => if let Splat::Changeling(.., data) = &mut character.splat {
   					data.clarity.poke(&wound);
   					if let Wound::Lethal = wound {
   						data.clarity.poke(&Wound::Aggravated);
   					}
   				},
			Event::TouchstoneChanged(i, val) => {
				if let Some(touchstone) = character.touchstones.get_mut(i) {
					*touchstone = val;
				} else {
					character.touchstones.resize(i + 1, String::new());
					*character.touchstones.get_mut(i).unwrap() = val;
				}
			}
		}
		None
	}

	#[allow(clippy::too_many_lines)]
	fn view(&self, _state: &Self::State) -> Element<Event, Theme> {
		let character = self.character.borrow();

		let mut col = Column::<Event, Theme>::new()
			.align_items(Alignment::Center)
			.spacing(COMPONENT_SPACING);

		let dots: Element<Event, Theme> = if let Splat::Changeling(.., data) = &character.splat {
			HealthTrack::new(
				data.clarity.clone(),
				data.max_clarity(&character) as usize,
				Event::IntegrityDamage,
			)
			.into()
		} else {
			let mut coll = Column::new();

			let mut flag = false;

			if let Splat::Vampire(..) | Splat::Bound(..) = &character.splat {
				flag = true;

				coll = coll.width(Length::FillPortion(4)).spacing(1);

				for i in 0..10 {
					coll = coll.push(
						column![text_input(
							"",
							character.touchstones.get(i).unwrap_or(&String::new()),
						)
						.on_input(move |val| Event::TouchstoneChanged(i, val))
						.padding(INPUT_PADDING)]
						.max_width(
							MAX_INPUT_WIDTH - SheetDots::<Event, Theme>::DEFAULT_SIZE, // - SheetDots::<Event, Renderer>::DEFAULT_SPACING,
						),
					);
				}
			}

			row![
				column![
					SheetDots::new(character.integrity, 1, 10, Shape::Dots, None, |val| {
						Event::IntegrityChanged(val)
					})
					.axis(if flag {
						widget::dots::Axis::Vertical
					} else {
						widget::dots::Axis::Horizontal
					})
					.spacing(if flag {
						4f32
					} else {
						SheetDots::<Event, Theme>::DEFAULT_SPACING
					}),
				]
				.align_items(if flag {
					Alignment::End
				} else {
					Alignment::Center
				})
				.width(Length::Fill),
				coll
			]
			.align_items(Alignment::Center)
			.spacing(5)
			.into()
		};

		let label = text(flt(character.splat.name(), Some(character.splat.integrity())).unwrap())
			.size(H3_SIZE);

		if let Splat::Werewolf(..) = character.splat {
			col = col.push(
				column![
					text(fl!("werewolf", "flesh-touchstone")).size(H3_SIZE),
					column![text_input(
						"",
						character.touchstones.first().unwrap_or(&String::new()),
					)
					.on_input(|str| Event::TouchstoneChanged(0, str),)
					.padding(INPUT_PADDING)]
					.max_width(MAX_INPUT_WIDTH),
				]
				.align_items(Alignment::Center)
				.spacing(TITLE_SPACING),
			);
		}

		col = col.push(
			column![label, dots]
				.align_items(Alignment::Center)
				.spacing(TITLE_SPACING),
		);

		match character.splat {
			Splat::Werewolf(..) => {
				col = col.push(
					column![
						text(fl!("werewolf", "spirit-touchstone")).size(H3_SIZE),
						column![text_input(
							"",
							character.touchstones.get(1).unwrap_or(&String::new()),
						)
						.on_input(|str| Event::TouchstoneChanged(1, str),)
						.padding(INPUT_PADDING)]
						.max_width(MAX_INPUT_WIDTH),
					]
					.align_items(Alignment::Center)
					.spacing(TITLE_SPACING),
				);
			}
			Splat::Changeling(..) => {
				col = col.push(
					list(
						fl!("touchstones"),
						Some(10),
						Some(10),
						character.touchstones.clone() as Vec<String>,
						|i, val| {
							text_input("", &val.unwrap_or_default())
								.on_input(move |val| Event::TouchstoneChanged(i, val))
								.padding(INPUT_PADDING)
								.into()
						},
					)
					.max_width(MAX_INPUT_WIDTH),
				);
			}
			_ => (),
		}

		col.into()
	}
}

impl<'a, Message, Theme> From<IntegrityComponent> for Element<'a, Message, Theme>
where
	Message: 'a,
	Theme:
		text::StyleSheet + text_input::StyleSheet + dots::StyleSheet + track::StyleSheet + 'static,
{
	fn from(integrity: IntegrityComponent) -> Self {
		component(integrity)
	}
}
