use std::{cell::RefCell, rc::Rc};

use cofd::{
	character::InfoTrait,
	prelude::Character,
	splat::{Splat, XSplat, YSplat},
};
use iced::{
	widget::{column, pick_list, row, text, text_input, Column, Row},
	Alignment, Length,
};
use iced_lazy::Component;
use iced_native::Element;

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
pub enum Event {
	InfoTraitChanged(String, InfoTrait),
	XSplatChanged(XSplat),
	YSplatChanged(YSplat),
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

			col1 = col1.push(text(format!("{}:", fl(msg, attribute))));
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

	fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
		let mut character = self.character.borrow_mut();

		match event {
			Event::InfoTraitChanged(val, _trait) => *character.info.get_mut(&_trait) = val,
			Event::XSplatChanged(xsplat) => character.splat.set_xsplat(Some(xsplat)),
			Event::YSplatChanged(ysplat) => character.splat.set_ysplat(Some(ysplat)),
		}

		Some((self.on_change)())
	}

	#[allow(clippy::similar_names, clippy::single_match_else)]
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
				let xsplats = XSplat::all(&character.splat._type());
				let ysplats = YSplat::all(&character.splat._type());

				// xsplats.extend(self.custom_xsplats.iter().filter_map(|xsplat| {
				// 	match (xsplat, &character.splat) {
				// 		(XSplat::Vampire(_), Splat::Vampire(_, _, _))
				// 		| (XSplat::Werewolf(_), Splat::Werewolf(_, _, _, _))
				// 		| (XSplat::Mage(_), Splat::Mage(_, _, _))
				// 		| (XSplat::Changeling(_), Splat::Changeling(_, _, _, _)) => Some(xsplat.clone()),
				// 		_ => None,
				// 	}
				// }));

				row![
					column![
						text(format!(
							"{}:",
							fl(character.splat.name(), Some(character.splat.xsplat_name()))
						)),
						text(format!(
							"{}:",
							fl(character.splat.name(), Some(character.splat.ysplat_name()))
						))
					]
					.spacing(3),
					column![
						pick_list(xsplats, character.splat.xsplat(), Event::XSplatChanged)
							.padding(1)
							.width(Length::Fill),
						pick_list(ysplats, character.splat.ysplat(), Event::YSplatChanged)
							.padding(1)
							.width(Length::Fill)
					]
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
