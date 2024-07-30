use iced::widget::{component, container, scrollable, Component};
use iced::{
	widget::{column, pick_list, row, text, text_input, Column},
	Alignment, Length,
};
use std::{cell::RefCell, rc::Rc};

use crate::i18n::{Translate, Translated};
use crate::{fl, i18n, Element, INPUT_PADDING};
use cofd::{
	character::InfoTrait,
	prelude::*,
	splat::{Splat, SplatTrait, XSplat, YSplat, ZSplat},
};
use iced::overlay::menu;

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

	fn mk_info_col<Theme>(
		&self,
		info: Vec<InfoTrait>,
		character: &Character,
	) -> Element<Event, Theme>
	where
		Theme: text_input::StyleSheet + text::StyleSheet + 'static,
	{
		let mut col1 = Column::new().spacing(3);
		let mut col2 = Column::new()
			.spacing(3)
			.width(Length::Fill)
			.align_items(Alignment::End);

		for _trait in info {
			let str = match _trait {
				InfoTrait::VirtueAnchor => character.splat.virtue_anchor().translated(),
				InfoTrait::ViceAnchor => character.splat.vice_anchor().translated(),
				_ => _trait.translated(),
			};

			col1 = col1.push(text(format!("{}:", str)));
			col2 = col2.push(
				text_input("", character.info.get(_trait))
					.on_input(move |val| Event::InfoTraitChanged(val, _trait))
					.padding(INPUT_PADDING),
			);
		}

		row![col1, col2].width(Length::Fill).spacing(5).into()
	}
}

impl<Message, Theme> Component<Message, Theme> for InfoBar<Message>
where
	Theme: text_input::StyleSheet + text::StyleSheet,
	Theme: pick_list::StyleSheet
		+ scrollable::StyleSheet
		+ menu::StyleSheet
		+ container::StyleSheet
		+ 'static,
	<Theme as menu::StyleSheet>::Style: From<<Theme as pick_list::StyleSheet>::Style>,
{
	type State = ();
	type Event = Event;

	fn update(&mut self, _state: &mut Self::State, event: Event) -> Option<Message> {
		let mut character = self.character.borrow_mut();

		match event {
			Event::InfoTraitChanged(val, _trait) => *character.info.get_mut(_trait) = val,
			Event::XSplatChanged(xsplat) => {
				if xsplat.name().eq("") {
					character.splat.set_xsplat(None);
				} else {
					character.splat.set_xsplat(Some(xsplat));
				}
				character.calc_mod_map();
			}
			Event::YSplatChanged(ysplat) => {
				if ysplat.name().eq("") {
					character.splat.set_ysplat(None);
				} else {
					character.splat.set_ysplat(Some(ysplat));
				}
				//character.calc_mod_map();
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
	fn view(&self, _state: &Self::State) -> Element<Event, Theme> {
		let character = self.character.borrow();

		let col3: Element<Event, Theme> = match character.splat {
			Splat::Mortal(..) => self.mk_info_col(
				vec![InfoTrait::Age, InfoTrait::Faction, InfoTrait::GroupName],
				&character,
			),
			_ => {
				let mut xsplats = character.splat.xsplats();
				let mut ysplats = character.splat.ysplats();
				let mut zsplats = character.splat.zsplats();

				if let Some(xsplat) = character.splat.custom_xsplat(fl!("custom")) {
					xsplats.push(xsplat);
				}
				if let Some(ysplat) = character.splat.custom_ysplat(fl!("custom")) {
					ysplats.push(ysplat);
				}
				if let Some(zsplat) = character.splat.custom_zsplat(fl!("custom")) {
					zsplats.push(zsplat);
				}

				let xsplats: Vec<Translated<XSplat>> =
					xsplats.into_iter().map(Into::into).collect();
				let ysplats: Vec<Translated<YSplat>> =
					ysplats.into_iter().map(Into::into).collect();
				let zsplats: Vec<Translated<ZSplat>> =
					zsplats.into_iter().map(Into::into).collect();

				let xsplat = character.splat.xsplat();
				let ysplat = character.splat.ysplat();
				let zsplat = character.splat.zsplat();

				let xsplat: Element<Event, Theme> = if let Some(xsplat) = xsplat.clone()
					&& xsplat.is_custom()
				{
					text_input("", xsplat.name())
						.on_input({
							let xsplat = xsplat.clone();
							move |val| {
								let mut xsplat = xsplat.clone();
								*xsplat.name_mut().unwrap() = val;
								Event::XSplatChanged(xsplat)
							}
						})
						.padding(INPUT_PADDING)
						.into()
				} else {
					pick_list(
						xsplats,
						xsplat.map(Into::<Translated<XSplat>>::into),
						|val| Event::XSplatChanged(val.unwrap()),
					)
					.padding(INPUT_PADDING)
					.width(Length::Fill)
					.into()
				};

				let ysplat: Element<Event, Theme> = if let Some(ysplat) = ysplat.clone()
					&& ysplat.is_custom()
				{
					text_input("", ysplat.name())
						.on_input({
							let ysplat = ysplat.clone();
							move |val| {
								let mut ysplat = ysplat.clone();
								*ysplat.name_mut().unwrap() = val;
								Event::YSplatChanged(ysplat)
							}
						})
						.padding(INPUT_PADDING)
						.into()
				} else {
					pick_list(
						ysplats,
						ysplat.map(Into::<Translated<YSplat>>::into),
						|val| Event::YSplatChanged(val.unwrap()),
					)
					.padding(INPUT_PADDING)
					.width(Length::Fill)
					.into()
				};

				let zsplat: Element<Event, Theme> = if let Some(zsplat) = zsplat.clone()
					&& zsplat.is_custom()
				{
					text_input("", zsplat.name())
						.on_input({
							let zsplat = zsplat.clone();
							move |val| {
								let mut zsplat = zsplat.clone();
								*zsplat.name_mut().unwrap() = val;
								Event::ZSplatChanged(zsplat)
							}
						})
						.padding(INPUT_PADDING)
						.into()
				} else {
					pick_list(
						zsplats,
						zsplat.map(Into::<Translated<ZSplat>>::into),
						|val| Event::ZSplatChanged(val.unwrap()),
					)
					.padding(INPUT_PADDING)
					.width(Length::Fill)
					.into()
				};

				let xsplat_name = character.splat.xsplat_name().map(|k| i18n::LANGUAGE_LOADER.get(k)).unwrap_or_default();
				let ysplat_name = character.splat.ysplat_name().map(|k| i18n::LANGUAGE_LOADER.get(k)).unwrap_or_default();
				let zsplat_name = character.splat.zsplat_name().map(|k| i18n::LANGUAGE_LOADER.get(k)).unwrap_or_default();

				row![
					column![
						text(format!("{xsplat_name}:")),
						text(format!("{ysplat_name}:")),
						text(format!("{zsplat_name}:"))
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
		.width(Length::Fill)
		.spacing(10)
		.into()
	}
}

impl<'a, Message, Theme> From<InfoBar<Message>> for Element<'a, Message, Theme>
where
	Message: 'a,
	Theme: text_input::StyleSheet + text::StyleSheet,
	Theme: pick_list::StyleSheet
		+ scrollable::StyleSheet
		+ menu::StyleSheet
		+ container::StyleSheet
		+ 'static,
	<Theme as menu::StyleSheet>::Style: From<<Theme as pick_list::StyleSheet>::Style>,
{
	fn from(info_bar: InfoBar<Message>) -> Self {
		component(info_bar)
	}
}
