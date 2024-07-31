use super::list;
use crate::i18n::Translate;
use crate::widget::{dots, track};
use crate::{
	fl,
	widget::{
		self,
		dots::{Shape, SheetDots},
		track::HealthTrack,
	},
	Element, COMPONENT_SPACING, H3_SIZE, INPUT_PADDING, MAX_INPUT_WIDTH, TITLE_SPACING,
};
use cofd::{character::Wound, prelude::*, splat::Splat};
use iced::{
	widget::{column, row, text, text_input, Column},
	Alignment, Length,
};

#[derive(Debug, Clone)]
pub struct IntegrityComponent;

#[derive(Clone)]
pub enum Message {
	IntegrityChanged(u16),
	IntegrityDamage(Wound),
	TouchstoneChanged(usize, String),
}

impl IntegrityComponent {
	pub fn new() -> Self {
		Self
	}

	pub fn update(&mut self, event: Message, character: &mut Character) {
		match event {
			Message::IntegrityChanged(val) => character.integrity = val,
			Message::IntegrityDamage(wound) => {
				if let Splat::Changeling(.., data) = &mut character.splat {
					data.clarity.poke(&wound);
					if let Wound::Lethal = wound {
						data.clarity.poke(&Wound::Aggravated);
					}
				}
			}
			Message::TouchstoneChanged(i, val) => {
				if let Some(touchstone) = character.touchstones.get_mut(i) {
					*touchstone = val;
				} else {
					character.touchstones.resize(i + 1, String::new());
					*character.touchstones.get_mut(i).unwrap() = val;
				}
			}
		}
	}

	#[allow(clippy::too_many_lines)]
	pub fn view(&self, character: &Character) -> Element<Message> {
		let mut col = Column::<Message>::new()
			.align_items(Alignment::Center)
			.spacing(COMPONENT_SPACING);

		let dots: Element<Message> = if let Splat::Changeling(.., data) = &character.splat {
			HealthTrack::new(
				data.clarity.clone(),
				data.max_clarity(&character.attributes()) as usize,
				Message::IntegrityDamage,
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
						.on_input(move |val| Message::TouchstoneChanged(i, val))
						.padding(INPUT_PADDING)]
						.max_width(
							MAX_INPUT_WIDTH - SheetDots::<Message>::DEFAULT_SIZE, // - SheetDots::<Event, Renderer>::DEFAULT_SPACING,
						),
					);
				}
			}

			row![
				column![
					SheetDots::new(character.integrity, 1, 10, Shape::Dots, None, |val| {
						Message::IntegrityChanged(val)
					})
					.axis(if flag {
						widget::dots::Axis::Vertical
					} else {
						widget::dots::Axis::Horizontal
					})
					.spacing(if flag {
						4f32
					} else {
						SheetDots::<Message>::DEFAULT_SPACING
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

		let label = text(character.splat.integrity().translated()).size(H3_SIZE);

		if let Splat::Werewolf(..) = character.splat {
			col = col.push(
				column![
					text(fl!("flesh-touchstone")).size(H3_SIZE),
					column![text_input(
						"",
						character.touchstones.first().unwrap_or(&String::new()),
					)
					.on_input(|str| Message::TouchstoneChanged(0, str),)
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
						text(fl!("spirit-touchstone")).size(H3_SIZE),
						column![text_input(
							"",
							character.touchstones.get(1).unwrap_or(&String::new()),
						)
						.on_input(|str| Message::TouchstoneChanged(1, str),)
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
						character.touchstones.clone(),
						|i, val| {
							text_input("", &val.unwrap_or_default())
								.on_input(move |val| Message::TouchstoneChanged(i, val))
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
