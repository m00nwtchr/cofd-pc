use iced::{
	widget::{checkbox, column, pick_list, row, text, text_input, Column},
	Alignment, Length,
};
use std::{cell::RefCell, rc::Rc};

use crate::component::{forms_component, list};
use crate::widget::dots;
use crate::{
	fl,
	i18n::{flt, Translated},
	widget::dots::{Shape, SheetDots},
	Element, H2_SIZE, H3_SIZE, INPUT_PADDING, TITLE_SPACING,
};
use cofd::{
	prelude::*,
	splat::{
		changeling::Contract,
		mage::{Arcanum, Rote},
		werewolf::{Rite, ShadowGift, WolfGift},
		Splat,
	},
};
use iced::widget::{button, component, container, overlay, scrollable, Component};

fn func<C: Clone, T, Message>(
	c: C,
	f: impl Fn(&mut C, T),
	msg: impl Fn(C) -> Message,
) -> impl Fn(T) -> Message {
	move |val: T| {
		let mut c = c.clone();
		f(&mut c, val);
		(msg)(c)
	}
}
// |i: usize, contract: Contract, f: Box<dyn Fn(Contract, T) -> Contract>| {
// 				move |val: T| {
// 					Event::ContractChanged(i, f(contract.clone(), val))
// 				}
// 			}

pub struct SplatExtrasTab {
	character: Rc<RefCell<Character>>,
}

pub fn splat_extras_tab(character: Rc<RefCell<Character>>) -> SplatExtrasTab {
	SplatExtrasTab::new(character)
}

#[derive(Clone)]
pub enum Event {
	RoteChanged(usize, Rote),

	ShadowGiftChanged(usize, ShadowGift),
	WolfGiftChanged(usize, WolfGift),
	RiteChanged(usize, Rite),

	ContractChanged(usize, Contract),

	Msg,
}

impl SplatExtrasTab {
	pub fn new(character: Rc<RefCell<Character>>) -> Self {
		Self { character }
	}
}

impl<Message, Theme> Component<Message, Theme> for SplatExtrasTab
where
	Message: Clone,
	Theme: 'static
		+ text::StyleSheet
		+ text_input::StyleSheet
		+ pick_list::StyleSheet
		+ scrollable::StyleSheet
		+ overlay::menu::StyleSheet
		+ container::StyleSheet
		+ dots::StyleSheet
		+ checkbox::StyleSheet
		+ button::StyleSheet
		+ 'static,
	<Theme as overlay::menu::StyleSheet>::Style: From<<Theme as pick_list::StyleSheet>::Style>,
{
	type State = ();

	type Event = Event;

	fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
		let mut character = self.character.borrow_mut();

		match event {
			Event::RoteChanged(i, rote) => {
				if let Splat::Mage(.., data) = &mut character.splat {
					data.rotes.push(rote);
					data.rotes.swap_remove(i);
				}
			}
			Event::ShadowGiftChanged(i, val) => {
				if let Splat::Werewolf(.., data) = &mut character.splat {
					if let Some(m) = data.shadow_gifts.get_mut(i) {
						*m = val;
					} else {
						data.shadow_gifts.push(val);
					}
				}
			}

			Event::WolfGiftChanged(i, val) => {
				if let Splat::Werewolf(.., data) = &mut character.splat {
					if let Some(m) = data.wolf_gifts.get_mut(i) {
						*m = val;
					} else {
						data.wolf_gifts.push(val);
					}
				}
			}

			Event::Msg => {}
			Event::RiteChanged(i, val) => {
				if let Splat::Werewolf(.., data) = &mut character.splat {
					if let Rite::_Custom(name) = &val
						&& name.eq("")
					{
						data.rites.remove(i);
					} else if let Some(m) = data.rites.get_mut(i) {
						*m = val;
					} else {
						data.rites.push(val);
					}
				}
			}
			Event::ContractChanged(i, rote) => {
				if let Splat::Changeling(.., data) = &mut character.splat {
					data.contracts.push(rote);
					data.contracts.swap_remove(i);
				}
			}
		}

		None
	}

	#[allow(clippy::too_many_lines)]
	fn view(&self, _state: &Self::State) -> Element<Event, Theme> {
		let character = self.character.borrow();

		let rotes: Element<Self::Event, Theme> = if let Splat::Mage(.., data) = &character.splat {
			let col = |txt, ratio| -> Column<Event, Theme> {
				column![text(txt)]
					.align_items(Alignment::Center)
					.width(Length::FillPortion(ratio))
					.spacing(3)
			};

			let mut arcanum = col("Arcanum", 3);
			let mut level = col("Level", 1);
			let mut spell = col("Spell", 6);
			let mut creator = col("Creator", 3);
			let mut rote_skill = col("Rote Skill", 3);

			let arcana: Vec<Translated<Arcanum>> = Vec::from(Arcanum::all().map(Into::into));
			let skills = Vec::from(Skill::all().map(Into::<Translated<Skill>>::into));

			for (i, rote) in data.rotes.iter().enumerate() {
				arcanum = arcanum.push(
					pick_list(
						arcana.clone(),
						Some::<Translated<Arcanum>>(rote.arcanum.clone().into()),
						func(
							rote.clone(),
							|rote, val: Translated<Arcanum>| rote.arcanum = val.unwrap(),
							move |rote| Event::RoteChanged(i, rote),
						),
					)
					.width(Length::Fill),
				);
				level = level.push(text_input("", &rote.level.to_string()).on_input(func(
					rote.clone(),
					|rote, val: String| rote.level = val.parse().unwrap_or(rote.level),
					move |rote| Event::RoteChanged(i, rote),
				)));
				spell = spell.push(text_input("", &rote.spell).on_input(func(
					rote.clone(),
					|rote, val| rote.spell = val,
					move |rote| Event::RoteChanged(i, rote),
				)));
				creator = creator.push(text_input("", &rote.creator).on_input(func(
					rote.clone(),
					|rote, val| rote.creator = val,
					move |rote| Event::RoteChanged(i, rote),
				)));
				rote_skill = rote_skill.push(
					pick_list(
						skills.clone(),
						Some::<Translated<Skill>>(rote.skill.into()),
						func(
							rote.clone(),
							|rote, val: Translated<Skill>| rote.skill = val.unwrap(),
							move |rote| Event::RoteChanged(i, rote),
						),
					)
					.width(Length::Fill),
				);
			}

			column![
				text("Rotes").size(H3_SIZE),
				row![arcanum, level, spell, creator, rote_skill].spacing(5)
			]
			.align_items(Alignment::Center)
			.spacing(TITLE_SPACING)
			.into()
		} else {
			column![].into()
		};

		let forms = forms_component(self.character.clone(), Event::Msg);

		let gifts = if let Splat::Werewolf(auspice, _, _, data) = &character.splat {
			let shadow_gifts: Vec<Translated<ShadowGift>> = ShadowGift::all()
				.into_iter()
				.filter(|g| !data.shadow_gifts.contains(g))
				.map(Into::into)
				.collect();
			let wolf_gifts: Vec<Translated<WolfGift>> = WolfGift::all()
				.into_iter()
				.filter(|g| !data.wolf_gifts.contains(g))
				.map(Into::into)
				.collect();

			let shadow_gifts = list(
				fl!("shadow-gifts"),
				Some(data.shadow_gifts.len() + 1),
				None,
				data.shadow_gifts.clone(),
				{
					let shadow_gifts = shadow_gifts;
					move |i, val| {
						pick_list(
							shadow_gifts.clone(),
							val.map(Into::<Translated<ShadowGift>>::into),
							move |val| Event::ShadowGiftChanged(i, val.unwrap()),
						)
						.padding(INPUT_PADDING)
						.into()
					}
				},
			);

			let wolf_gifts = list(
				fl!("wolf-gifts"),
				Some(data.wolf_gifts.len() + 1),
				None,
				data.wolf_gifts.clone(),
				{
					let wolf_gifts = wolf_gifts;
					move |i, val| {
						pick_list(
							wolf_gifts.clone(),
							val.map(Into::<Translated<WolfGift>>::into),
							move |val| Event::WolfGiftChanged(i, val.unwrap()),
						)
						.padding(INPUT_PADDING)
						.into()
					}
				},
			);

			let m = if let Some(auspice) = auspice {
				// let moon_gifts = row![];

				let gift = auspice.get_moon_gift();

				let val = *character
					.abilities
					.get(&auspice.get_renown().clone().into())
					.unwrap();

				column![
					text(fl!("moon-gifts")).size(H3_SIZE),
					row![
						text(flt("moon-gifts", Some(gift.name())).unwrap()),
						SheetDots::new(val, 0, 5, Shape::Dots, None, |_| Event::Msg)
							.width(Length::Shrink)
					]
				]
				.width(Length::Fill)
				.align_items(Alignment::Center)
				.spacing(TITLE_SPACING)
			} else {
				column![]
			};

			column![m, row![shadow_gifts, wolf_gifts]].align_items(Alignment::Center)
		} else {
			column![]
		};

		let rites = if let Splat::Werewolf(.., data) = &character.splat {
			let list = list(
				fl!("rites"),
				Some(data.rites.len() + 1),
				None,
				data.rites.clone(),
				{
					move |i, rite| {
						if let Some(Rite::_Custom(name)) = rite {
							text_input("", &name)
								.on_input(move |val| Event::RiteChanged(i, Rite::_Custom(val)))
								.into()
						} else {
							pick_list(
								vec![Rite::_Custom(fl!("custom")).into()],
								rite.map(Into::<Translated<Rite>>::into),
								move |val: Translated<Rite>| Event::RiteChanged(i, val.unwrap()),
							)
							.padding(INPUT_PADDING)
							.into()
						}
					}
				},
			);

			row![list].align_items(Alignment::Center)
		} else {
			row![]
		};

		let contracts = if let Splat::Changeling(.., data) = &character.splat {
			let col = |txt, ratio| -> Column<Event, Theme> {
				column![text(txt)]
					.align_items(Alignment::Center)
					.width(Length::FillPortion(ratio))
					.spacing(3)
			};

			let mut name = col("Name", 3).align_items(Alignment::Start);
			let mut goblin = col("Goblin?", 1);
			let mut cost = col("Cost", 1);
			let mut dice = col("Dice", 2);
			let mut action = col("Action", 1);
			let mut duration = col("Duration", 1);
			let mut loophole = col("Loophole", 2);
			let mut seeming_benefit = col("Seeming Benefit", 3);

			for (i, contract) in data.contracts.iter().enumerate() {
				name = name.push(text_input("", &contract.name).on_input(func(
					contract.clone(),
					|contract, val| contract.name = val,
					move |val| Event::ContractChanged(i, val),
				)));
				goblin = goblin.push(checkbox("", contract.goblin).on_toggle(func(
					contract.clone(),
					|contract, val| contract.goblin = val,
					move |val| Event::ContractChanged(i, val),
				)));
				cost = cost.push(text_input("", &contract.cost).on_input(func(
					contract.clone(),
					|contract, val| contract.cost = val,
					move |val| Event::ContractChanged(i, val),
				)));
				dice = dice.push(text_input("", &contract.dice).on_input(func(
					contract.clone(),
					|contract, val| contract.dice = val,
					move |val| Event::ContractChanged(i, val),
				)));
				action = action.push(text_input("", &contract.action).on_input(func(
					contract.clone(),
					|contract, val| contract.action = val,
					move |val| Event::ContractChanged(i, val),
				)));
				duration = duration.push(text_input("", &contract.duration).on_input(func(
					contract.clone(),
					|contract, val| contract.duration = val,
					move |val| Event::ContractChanged(i, val),
				)));
				loophole = loophole.push(text_input("", &contract.loophole).on_input(func(
					contract.clone(),
					|contract, val| contract.loophole = val,
					move |val| Event::ContractChanged(i, val),
				)));
				seeming_benefit =
					seeming_benefit.push(text_input("", &contract.seeming_benefit).on_input(func(
						contract.clone(),
						|contract, val| contract.seeming_benefit = val,
						move |val| Event::ContractChanged(i, val),
					)));
			}

			column![
				text("Contracts").size(H2_SIZE),
				row![
					name,
					goblin,
					cost,
					dice,
					action,
					duration,
					loophole,
					seeming_benefit
				]
				.spacing(5)
			]
			.align_items(Alignment::Center)
			.spacing(TITLE_SPACING)
		} else {
			column![]
		};

		let mut row = Column::new().width(Length::Fill);

		match &character.splat {
			Splat::Vampire(..) => {}
			Splat::Werewolf(..) => {
				row = row
					.align_items(Alignment::Center)
					.push(forms)
					.push(text("Gifts and Rites").size(H2_SIZE))
					.push(gifts)
					.push(rites);
			}
			Splat::Mage(..) => {
				row = row.push(rotes);
			}
			Splat::Changeling(..) => row = row.push(contracts),
			_ => {}
		}

		row.into()
	}
}

impl<'a, Message> From<SplatExtrasTab> for Element<'a, Message>
where
	Message: 'a + Clone,
{
	fn from(splat_extras_tab: SplatExtrasTab) -> Self {
		component(splat_extras_tab)
	}
}
