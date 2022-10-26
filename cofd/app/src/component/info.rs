use std::{cell::RefCell, rc::Rc};

use iced::{
	widget::{column, pick_list, row, text, text_input, Column, Row},
	Alignment, Length,
};
use iced_lazy::Component;
use iced_native::Element;

use cofd::{
	character::InfoTrait,
	prelude::Character,
	splat::{Splat, XSplat, YSplat, ZSplat},
};

use crate::{i18n::fl, widget};

pub struct InfoBar<Message> {
	character: Rc<RefCell<Character>>,
	on_change: Box<dyn Fn() -> Message>,
}

pub fn info_bar<Message>(
	character: Rc<RefCell<Character>>,
	on_change: impl Fn() -> Message + 'static,
) -> InfoBar<Message> {
	InfoBar::new(character, on_change)
}

#[derive(Clone)]
#[allow(clippy::enum_variant_names)]
pub enum Event {
	InfoTraitChanged(String, InfoTrait),
	XSplatChanged(XSplat),
	YSplatChanged(YSplat),
	ZSplatChanged(ZSplat),
}

impl<Message> InfoBar<Message> {
	fn new(character: Rc<RefCell<Character>>, on_change: impl Fn() -> Message + 'static) -> Self {
		Self {
			character,
			on_change: Box::new(on_change),
		}
	}

	fn mk_info_col<Renderer>(
		&self,
		info: Vec<InfoTrait>,
		character: &Character,
	) -> Row<Event, Renderer>
	where
		Renderer: iced_native::text::Renderer + 'static,
		Renderer::Theme: iced::widget::pick_list::StyleSheet
			+ iced::widget::text_input::StyleSheet
			+ iced::widget::text::StyleSheet,
	{
		let mut col1 = Column::new().spacing(3);
		let mut col2 = Column::new()
			.spacing(3)
			.width(Length::Fill)
			.align_items(Alignment::End);

		for _trait in info {
			let (msg, attribute) = match _trait {
				InfoTrait::VirtueAnchor | InfoTrait::ViceAnchor => {
					if character.splat.virtue_anchor() == "virtue" {
						(_trait.name(), None)
					} else {
						match _trait {
							InfoTrait::VirtueAnchor => (
								character.splat.name(),
								Some(character.splat.virtue_anchor()),
							),
							InfoTrait::ViceAnchor => {
								(character.splat.name(), Some(character.splat.vice_anchor()))
							}
							_ => unreachable!(),
						}
					}
				}
				_ => (_trait.name(), None),
			};

			col1 = col1.push(text(format!("{}:", fl(msg, attribute).unwrap())));
			col2 = col2.push(text_input("", character.info.get(&_trait), move |val| {
				Event::InfoTraitChanged(val, _trait)
			}));
		}

		row![col1, col2].width(Length::Fill).spacing(5)
	}
}

impl<Message, Renderer> Component<Message, Renderer> for InfoBar<Message>
where
	Renderer: iced_native::text::Renderer + 'static,
	Renderer::Theme: iced::widget::pick_list::StyleSheet
		+ iced::widget::text_input::StyleSheet
		+ iced::widget::text::StyleSheet,
{
	type State = ();
	type Event = Event;

	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		let mut character = self.character.borrow_mut();

		match event {
			Event::InfoTraitChanged(val, _trait) => *character.info.get_mut(&_trait) = val,
			Event::XSplatChanged(xsplat) => {
				if xsplat.name().eq("") {
					character.splat.set_xsplat(None);
				} else {
					character.splat.set_xsplat(Some(xsplat));
				}
			}
			Event::YSplatChanged(ysplat) => {
				if ysplat.name().eq("") {
					character.splat.set_ysplat(None);
				} else {
					character.splat.set_ysplat(Some(ysplat));
				}
			}
			Event::ZSplatChanged(zsplat) => {
				if zsplat.name().eq("") {
					character.splat.set_zsplat(None);
				} else {
					character.splat.set_zsplat(Some(zsplat));
				}
			}
		}

		Some((self.on_change)())
	}

	#[allow(
		clippy::similar_names,
		clippy::single_match_else,
		clippy::too_many_lines
	)]
	fn view(&self, _state: &Self::State) -> Element<Self::Event, Renderer> {
		let character = self.character.borrow();

		let col3: Element<Self::Event, Renderer> = match character.splat {
			Splat::Mortal => self
				.mk_info_col(
					vec![InfoTrait::Age, InfoTrait::Faction, InfoTrait::GroupName],
					&character,
				)
				.into(),
			_ => {
				let mut xsplats = XSplat::all(&character.splat._type());
				let mut ysplats = YSplat::all(&character.splat._type());
				let mut zsplats = ZSplat::all(&character.splat._type());

				if let Some(xsplat) = character.splat.custom_xsplat(String::from("Custom")) {
					xsplats.push(xsplat);
				}
				if let Some(ysplat) = character.splat.custom_ysplat(String::from("Custom")) {
					ysplats.push(ysplat);
				}
				if let Some(zsplat) = character.splat.custom_zsplat(String::from("Custom")) {
					zsplats.push(zsplat);
				}

				let xsplat = character.splat.xsplat();
				let ysplat = character.splat.ysplat();
				let zsplat = character.splat.zsplat();

				let xsplat: Element<Self::Event, Renderer> = if let Some(xsplat) = xsplat.clone() && xsplat.is_custom() {
					text_input("", xsplat.name(), {
						let xsplat = xsplat.clone();
						move |val| {
							let mut xsplat = xsplat.clone();
							*xsplat.name_mut().unwrap() = val;
							Event::XSplatChanged(xsplat)
						}
					})
					.into()
				} else {
					pick_list(xsplats, xsplat, Event::XSplatChanged)
					.padding(1)
					.width(Length::Fill).into()
				};

				let ysplat: Element<Self::Event, Renderer> = if let Some(ysplat) = ysplat.clone() && ysplat.is_custom() {
					text_input("", ysplat.name(), {
						let ysplat = ysplat.clone();
						move |val| {
							let mut ysplat = ysplat.clone();
							*ysplat.name_mut().unwrap() = val;
							Event::YSplatChanged(ysplat)
						}
					})
					.into()
				} else {
					pick_list(ysplats, ysplat, Event::YSplatChanged)
					.padding(1)
					.width(Length::Fill).into()
				};

				let zsplat: Element<Self::Event, Renderer> = if let Some(zsplat) = zsplat.clone() && zsplat.is_custom() {
					text_input("", zsplat.name(), {
						let zsplat = zsplat.clone();
						move |val| {
							let mut zsplat = zsplat.clone();
							*zsplat.name_mut().unwrap() = val;
							Event::ZSplatChanged(zsplat)
						}
					})
					.into()
				} else {
					pick_list(zsplats, zsplat, Event::ZSplatChanged)
						.padding(1)
						.width(Length::Fill)
						.into()
				};

				row![
					column![
						text(format!(
							"{}:",
							fl(character.splat.name(), Some(character.splat.xsplat_name()))
								.unwrap()
						)),
						text(format!(
							"{}:",
							fl(character.splat.name(), Some(character.splat.ysplat_name()))
								.unwrap()
						)),
						text(format!(
							"{}:",
							fl(character.splat.name(), Some(character.splat.zsplat_name()))
								.unwrap()
						))
					]
					.spacing(3),
					column![xsplat, ysplat, zsplat]
						.spacing(1)
						.width(Length::Fill)
				]
				.width(Length::Fill)
				.spacing(5)
				.into()
			}
		};

		row![
			self.mk_info_col(
				vec![InfoTrait::Name, InfoTrait::Player, InfoTrait::Chronicle],
				&character
			),
			self.mk_info_col(
				vec![
					InfoTrait::VirtueAnchor,
					InfoTrait::ViceAnchor,
					InfoTrait::Concept
				],
				&character
			),
			col3,
		]
		.spacing(10)
		.into()
	}
}

impl<'a, Message, Renderer> From<InfoBar<Message>> for Element<'a, Message, Renderer>
where
	Message: 'a,
	Renderer: 'static + iced_native::text::Renderer,
	Renderer::Theme: iced::widget::pick_list::StyleSheet
		+ iced::widget::text_input::StyleSheet
		+ iced::widget::text::StyleSheet
		+ widget::dots::StyleSheet,
{
	fn from(info_bar: InfoBar<Message>) -> Self {
		iced_lazy::component(info_bar)
	}
}
