use crate::component::{forms, list, FormsComponent};
use crate::i18n::Translated;
use crate::widget::dots;
use crate::{
	fl,
	i18n::Translate,
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
use iced::{
	widget::{checkbox, column, pick_list, row, text, text_input, Column},
	Alignment, Length,
};

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

#[derive(Debug, Clone)]
pub struct SplatExtrasTab {
	forms: FormsComponent,
}

#[derive(Clone)]
pub enum Message {
	RoteChanged(usize, Rote),

	ShadowGiftChanged(usize, ShadowGift),
	WolfGiftChanged(usize, WolfGift),
	RiteChanged(usize, Rite),

	ContractChanged(usize, Contract),

	FormsComponent(forms::Message),
	Msg,
}

impl SplatExtrasTab {
	pub fn new() -> Self {
		Self {
			forms: FormsComponent::new(),
		}
	}

	pub fn update(&mut self, message: Message, character: &mut Character) {
		match message {
			Message::RoteChanged(i, rote) => {
				if let Splat::Mage(.., data) = &mut character.splat {
					data.rotes.push(rote);
					data.rotes.swap_remove(i);
				}
			}
			Message::ShadowGiftChanged(i, val) => {
				if let Splat::Werewolf(.., data) = &mut character.splat {
					if let Some(m) = data.shadow_gifts.get_mut(i) {
						*m = val;
					} else {
						data.shadow_gifts.push(val);
					}
				}
			}

			Message::WolfGiftChanged(i, val) => {
				if let Splat::Werewolf(.., data) = &mut character.splat {
					if let Some(m) = data.wolf_gifts.get_mut(i) {
						*m = val;
					} else {
						data.wolf_gifts.push(val);
					}
				}
			}

			Message::RiteChanged(i, val) => {
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
			Message::ContractChanged(i, rote) => {
				if let Splat::Changeling(.., data) = &mut character.splat {
					data.contracts.push(rote);
					data.contracts.swap_remove(i);
				}
			}

			Message::FormsComponent(message) => self.forms.update(message, character),

			Message::Msg => {}
		}
	}

	#[allow(clippy::too_many_lines)]
	pub fn view(&self, character: &Character) -> Element<Message> {
		let col = |txt, ratio| -> Column<Message> {
			column![text(txt)]
				.align_items(Alignment::Center)
				.width(Length::FillPortion(ratio))
				.spacing(3)
		};

		let mut row = Column::new().width(Length::Fill);

		match &character.splat {
			Splat::Vampire(..) => {}
			Splat::Werewolf(data) => {
				let forms = self.forms.view(character).map(Message::FormsComponent);

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
					move |i, val| {
						pick_list(
							shadow_gifts.clone(),
							val.map(Into::<Translated<ShadowGift>>::into),
							move |val| Message::ShadowGiftChanged(i, val.unwrap()),
						)
						.padding(INPUT_PADDING)
						.into()
					},
				);

				let wolf_gifts = list(
					fl!("wolf-gifts"),
					Some(data.wolf_gifts.len() + 1),
					None,
					data.wolf_gifts.clone(),
					move |i, val| {
						pick_list(
							wolf_gifts.clone(),
							val.map(Into::<Translated<WolfGift>>::into),
							move |val| Message::WolfGiftChanged(i, val.unwrap()),
						)
						.padding(INPUT_PADDING)
						.into()
					},
				);

				let moon_gifts = if let Some(auspice) = &data.auspice {
					// let moon_gifts = row![];

					let gift = auspice.get_moon_gift();

					let val = character
						.abilities
						.get(&auspice.get_renown().clone().into())
						.copied()
						.unwrap_or_default();

					column![
						text(fl!("moon-gifts")).size(H3_SIZE),
						row![
							text(gift.translated()),
							SheetDots::new(val, 0, 5, Shape::Dots, None, |_| Message::Msg)
								.width(Length::Shrink)
						]
					]
					.width(Length::Fill)
					.align_items(Alignment::Center)
					.spacing(TITLE_SPACING)
				} else {
					column![]
				};

				let rites = list(
					fl!("rites"),
					Some(data.rites.len() + 1),
					None,
					data.rites.clone(),
					{
						move |i, rite| {
							if let Some(Rite::_Custom(name)) = rite {
								text_input("", &name)
									.on_input(move |val| {
										Message::RiteChanged(i, Rite::_Custom(val))
									})
									.into()
							} else {
								pick_list(
									vec![Rite::_Custom(fl!("custom")).into()],
									rite.map(Into::<Translated<Rite>>::into),
									move |val: Translated<Rite>| {
										Message::RiteChanged(i, val.unwrap())
									},
								)
								.padding(INPUT_PADDING)
								.into()
							}
						}
					},
				);

				row = row
					.align_items(Alignment::Center)
					.push(forms)
					.push(text("Gifts and Rites").size(H2_SIZE))
					.push(
						column![moon_gifts, row![shadow_gifts, wolf_gifts]]
							.align_items(Alignment::Center),
					)
					.push(row![rites].align_items(Alignment::Center));
			}
			Splat::Mage(data) => {
				let mut arcanum = col("Arcanum", 3);
				let mut level = col("Level", 1);
				let mut spell = col("Spell", 6);
				let mut creator = col("Creator", 3);
				let mut rote_skill = col("Rote Skill", 3);

				let arcana: Vec<Translated<_>> =
					Arcanum::all().iter().copied().map(Into::into).collect();
				let skills: Vec<Translated<_>> =
					Skill::all().iter().copied().map(Into::into).collect();

				for (i, rote) in data.rotes.iter().enumerate() {
					arcanum = arcanum.push(
						pick_list(
							arcana.clone(),
							Some::<Translated<Arcanum>>(rote.arcanum.clone().into()),
							func(
								rote.clone(),
								|rote, val: Translated<Arcanum>| rote.arcanum = val.unwrap(),
								move |rote| Message::RoteChanged(i, rote),
							),
						)
						.width(Length::Fill),
					);
					level = level.push(text_input("", &rote.level.to_string()).on_input(func(
						rote.clone(),
						|rote, val: String| rote.level = val.parse().unwrap_or(rote.level),
						move |rote| Message::RoteChanged(i, rote),
					)));
					spell = spell.push(text_input("", &rote.spell).on_input(func(
						rote.clone(),
						|rote, val| rote.spell = val,
						move |rote| Message::RoteChanged(i, rote),
					)));
					creator = creator.push(text_input("", &rote.creator).on_input(func(
						rote.clone(),
						|rote, val| rote.creator = val,
						move |rote| Message::RoteChanged(i, rote),
					)));
					rote_skill = rote_skill.push(
						pick_list(
							skills.clone(),
							Some::<Translated<Skill>>(rote.skill.into()),
							func(
								rote.clone(),
								|rote, val: Translated<Skill>| rote.skill = val.unwrap(),
								move |rote| Message::RoteChanged(i, rote),
							),
						)
						.width(Length::Fill),
					);
				}

				let rotes = column![
					text("Rotes").size(H3_SIZE),
					row![arcanum, level, spell, creator, rote_skill].spacing(5)
				]
				.align_items(Alignment::Center)
				.spacing(TITLE_SPACING);

				row = row.push(rotes);
			}
			Splat::Changeling(data) => {
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
						move |val| Message::ContractChanged(i, val),
					)));
					goblin = goblin.push(checkbox("", contract.goblin).on_toggle(func(
						contract.clone(),
						|contract, val| contract.goblin = val,
						move |val| Message::ContractChanged(i, val),
					)));
					cost = cost.push(text_input("", &contract.cost).on_input(func(
						contract.clone(),
						|contract, val| contract.cost = val,
						move |val| Message::ContractChanged(i, val),
					)));
					dice = dice.push(text_input("", &contract.dice).on_input(func(
						contract.clone(),
						|contract, val| contract.dice = val,
						move |val| Message::ContractChanged(i, val),
					)));
					action = action.push(text_input("", &contract.action).on_input(func(
						contract.clone(),
						|contract, val| contract.action = val,
						move |val| Message::ContractChanged(i, val),
					)));
					duration = duration.push(text_input("", &contract.duration).on_input(func(
						contract.clone(),
						|contract, val| contract.duration = val,
						move |val| Message::ContractChanged(i, val),
					)));
					loophole = loophole.push(text_input("", &contract.loophole).on_input(func(
						contract.clone(),
						|contract, val| contract.loophole = val,
						move |val| Message::ContractChanged(i, val),
					)));
					seeming_benefit = seeming_benefit.push(
						text_input("", &contract.seeming_benefit).on_input(func(
							contract.clone(),
							|contract, val| contract.seeming_benefit = val,
							move |val| Message::ContractChanged(i, val),
						)),
					);
				}

				let contracts = column![
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
				.spacing(TITLE_SPACING);

				row = row.push(contracts);
			}
			_ => {}
		}

		row.into()
	}
}
