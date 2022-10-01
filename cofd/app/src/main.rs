#![feature(is_some_and)]
#![deny(clippy::pedantic)]
#![allow(
	clippy::must_use_candidate,
	clippy::used_underscore_binding,
	clippy::unused_self,
	clippy::match_wildcard_for_single_variants,
	clippy::module_name_repetitions,
	clippy::wildcard_imports,
	clippy::match_same_arms,
	clippy::default_trait_access
)]

#[cfg(target_arch = "wasm32")]
use log::Level;
use std::{cell::RefCell, mem, rc::Rc};

use iced::{
	executor,
	widget::{button, container, row, text, Column},
	Application, Command, Element, Length, Settings, Theme,
};
use iced_aw::{TabLabel, Tabs};

// use i18n_embed::LanguageRequester;

use cofd::{
	character::CharacterInfo,
	prelude::*,
	splat::{
		ability::{Ability, AbilityVal},
		changeling::{Court, Seeming},
		mage::{Arcanum, MageData, MageMerit, Order, Path},
		vampire::{Bloodline, Clan, Covenant, Discipline, VampireMerit},
		werewolf::{Auspice, Form, Renown, Tribe, WerewolfMerit},
		Merit, Splat,
	},
};

mod component;
mod i18n;
mod view;
mod widget;

use i18n::fl;

#[derive(Debug, Clone)]
pub enum Tab {
	Overview,
	Equipment,
}

// #[derive(Clone)]
pub enum State {
	CharacterList,
	Sheet {
		active_tab: Tab,
		character: Rc<RefCell<Character>>,
	},
}

struct PlayerCompanionApp {
	state: State,
	prev_state: Option<State>,
	characters: Vec<Rc<RefCell<Character>>>,
	// character: Rc<RefCell<Character>>,
	// custom_xsplats: Vec<XSplat>,
	// locale: Locale,
	// language_requester: Box<dyn LanguageRequester<'static>>,
}

const H2_SIZE: u16 = 25;
const H3_SIZE: u16 = 20;

// const LANGS: [Locale; 4] = [
// 	Locale::System,
// 	Locale::Lang(langid!("en-GB")),
// 	Locale::Lang(langid!("en-US")),
// 	Locale::Lang(langid!("pl-PL")),
// ];

#[derive(Debug, Clone)]
enum Message {
	TabSelected(Tab),
	PickCharacter(usize),
	Previous,
}

impl PlayerCompanionApp {
	pub fn prev(&mut self) {
		if let Some(state) = self.prev_state.take() {
			self.state = state;
		}
	}
	pub fn next(&mut self, mut state: State) {
		mem::swap(&mut self.state, &mut state);
		self.prev_state = Some(state);
	}
}

impl Application for PlayerCompanionApp {
	type Executor = executor::Default;
	type Flags = ();
	type Message = Message;
	type Theme = Theme;

	#[allow(clippy::too_many_lines)]
	fn new(_flags: ()) -> (Self, Command<Self::Message>) {
		let _language_requester = i18n::setup();

		let vampire_character = Character::builder()
			.with_splat(Splat::Vampire(
				Clan::Ventrue,
				Some(Covenant::OrdoDracul),
				Some(Bloodline::_Custom(
					"Dragolescu".to_string(),
					[
						Discipline::Animalism,
						Discipline::Dominate,
						Discipline::Resilience,
						Discipline::Auspex,
					],
				)),
				Default::default(),
			))
			.with_info(CharacterInfo {
				name: String::from("Darren Webb"),
				player: String::from("m00n"),
				chronicle: String::from("Night Trains"),
				virtue_anchor: String::from("Scholar"),
				vice_anchor: String::from("Authoritarian"),
				concept: String::from("Occult Journalist/Mastermind"),
				..Default::default()
			})
			.with_attributes(Attributes {
				intelligence: 3,
				wits: 3,
				resolve: 2,
				strength: 1,
				dexterity: 3,
				stamina: 2,
				presence: 3,
				manipulation: 2,
				composure: 3,
			})
			.with_skills(Skills {
				investigation: 2,
				occult: 3,
				politics: 2,
				larceny: 3,
				stealth: 1,
				animal_ken: 1,
				expression: 3,
				intimidation: 1,
				streetwise: 2,
				subterfuge: 4,
				..Default::default()
			})
			.with_specialties(Skill::Larceny, vec![String::from("Sleight of Hand")])
			.with_specialties(Skill::Streetwise, vec![String::from("Rumours")])
			.with_specialties(Skill::Subterfuge, vec![String::from("Detecting Lies")])
			.with_abilities([
				(Discipline::Animalism.into(), 1),
				(Discipline::Dominate.into(), 2),
				(
					Discipline::_Custom("Coil of the Voivode".to_string()).into(),
					2,
				),
			])
			.with_merits([
				(Merit::Status("Ordo Dracul".to_string()), 1),
				(Merit::Status("City".to_string()), 1),
				(VampireMerit::CacophonySavvy.into(), 3),
				(Merit::FastTalking, 1),
				(
					Merit::ProfessionalTraining(
						String::new(),
						Some([Skill::Expression, Skill::Occult]),
						None,
					),
					2,
				),
				// AbilityVal(Ability::Merit(Merit::Contacts(String::new())), 2),
				(Merit::SafePlace(String::new()), 3),
				(Merit::Resources, 3),
				(VampireMerit::NestGuardian.into(), 1),
			])
			.build();

		let werewolf_character = Character::builder()
			.with_splat(Splat::Werewolf(
				Some(Auspice::Rahu),
				Some(Tribe::BloodTalons),
				None,
				Default::default(),
			))
			.with_info(CharacterInfo {
				name: String::from("Amos Gray"),
				player: String::from("m00n"),
				virtue_anchor: String::from("Destroyer"),
				vice_anchor: String::from("Lone Wolf"),
				..Default::default()
			})
			.with_attributes(Attributes {
				intelligence: 1,
				wits: 3,
				resolve: 2,
				strength: 3,
				dexterity: 2,
				stamina: 3,
				presence: 3,
				manipulation: 1,
				composure: 3,
			})
			.with_skills(Skills {
				investigation: 2,
				medicine: 2,
				athletics: 2,
				brawl: 4,
				stealth: 2,
				survival: 3,
				expression: 3,
				intimidation: 4,
				..Default::default()
			})
			.with_specialties(Skill::Brawl, vec![String::from("Claws")])
			.with_specialties(Skill::Stealth, vec![String::from("Stalking")])
			.with_specialties(Skill::Intimidation, vec![String::from("Direct Threats")])
			.with_abilities([(Renown::Glory.into(), 1), (Renown::Purity.into(), 3)])
			.with_merits([
				(Merit::Giant, 3),
				(Merit::TrainedObserver, 1),
				(Merit::DefensiveCombat(true, Some(Skill::Brawl)), 1),
				(WerewolfMerit::FavoredForm(Some(Form::Gauru)).into(), 2),
				(WerewolfMerit::EfficientKiller.into(), 2),
				(Merit::RelentlessAssault, 2),
				(Merit::Language("First Tongue".to_owned()), 1),
				(WerewolfMerit::Totem.into(), 1),
			])
			.build();

		let mage_character = Character::builder()
			.with_splat(Splat::Mage(
				Path::Mastigos,
				Some(Order::Mysterium),
				None,
				MageData {
					attr_bonus: Attribute::Resolve,
					obsessions: vec![],
				},
			))
			.with_info(CharacterInfo {
				name: String::from("Polaris"),
				player: String::from("m00n"),
				virtue_anchor: String::from("Curious"),
				vice_anchor: String::from("Greedy"),
				concept: String::from("Astronomer"),
				..Default::default()
			})
			.with_attributes(Attributes {
				intelligence: 3,
				wits: 3,
				resolve: 3,
				strength: 2,
				dexterity: 3,
				stamina: 2,
				presence: 1,
				manipulation: 2,
				composure: 3,
			})
			.with_skills(Skills {
				academics: 2,
				computer: 1,
				crafts: 1,
				investigation: 3,
				occult: 3,
				science: 2,

				larceny: 2,
				stealth: 2,

				animal_ken: 1,
				empathy: 2,
				expression: 1,
				subterfuge: 3,
				..Default::default()
			})
			.with_specialties(Skill::Academics, vec![String::from("Research")])
			.with_specialties(Skill::AnimalKen, vec![String::from("Felines")])
			.with_specialties(Skill::Subterfuge, vec![String::from("Detecting Lies")])
			// TODO: Professional Training specialties
			.with_specialties(Skill::Investigation, vec![String::from("Riddles")])
			.with_specialties(Skill::Science, vec![String::from("Astronomy")])
			.with_abilities([
				(Arcanum::Mind.into(), 1),
				(Arcanum::Prime.into(), 2),
				(Arcanum::Space.into(), 3),
			])
			.with_merits([
				(Merit::Status("Mysterium".to_string()), 1),
				(MageMerit::HighSpeech.into(), 1),
				(
					Merit::ProfessionalTraining(
						"e".to_owned(),
						Some([Skill::Investigation, Skill::Science]),
						None,
					),
					3,
				),
				(Merit::TrainedObserver, 1),
				//
				//
			])
			.build();

		(
			Self {
				// active_tab: 0,
				// character: Rc::new(RefCell::new(character)),
				state: State::CharacterList,
				prev_state: Default::default(),
				characters: vec![
					Rc::new(RefCell::new(vampire_character)),
					Rc::new(RefCell::new(werewolf_character)),
					Rc::new(RefCell::new(mage_character)),
				],
				// locale: Default::default(), // lang_loader,
				// language_requester,
				// custom_xsplats: vec![
				// 	// My OC (Original Clan) (Do Not Steal)
				// 	// XSplat::Vampire(Clan::_Custom(
				// 	// 	"Blorbo".to_owned(),
				// 	// 	[
				// 	// 		Discipline::Majesty,
				// 	// 		Discipline::Dominate,
				// 	// 		Discipline::Auspex,
				// 	// 	],
				// 	// 	[Attribute::Intelligence, Attribute::Presence],
				// 	// )),
				// ],
			},
			Command::none(),
		)
	}

	fn title(&self) -> String {
		fl!("app-name")
	}

	fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
		match message {
			Message::TabSelected(tab) => {
				if let State::Sheet { active_tab, .. } = &mut self.state {
					*active_tab = tab;
				}
			}
			Message::PickCharacter(i) => {
				self.next(State::Sheet {
					active_tab: Tab::Overview,
					character: self.characters.get(i).unwrap().clone(),
				});
			}
			Message::Previous => self.prev(),
		}

		Command::none()
	}

	fn view(&self) -> Element<Self::Message> {
		// view::overview_tab(character.clone(), Message::Previous)
		match &self.state {
			State::CharacterList => {
				view::character_list(self.characters.clone(), Message::PickCharacter).into()
			}
			State::Sheet {
				active_tab,
				character,
			} => {
				let tab: Element<Self::Message> = match active_tab {
					Tab::Overview => view::overview_tab(character.clone()).into(),
					Tab::Equipment => view::equipment_tab(character.clone()).into(),
				};

				Column::new()
					.push(row![
						button("Back").on_press(Message::Previous),
						button("Home").on_press(Message::TabSelected(Tab::Overview)),
						button("Equipment").on_press(Message::TabSelected(Tab::Equipment))
					]).spacing(1)
					.push(tab)
					.into()
			}
		}
	}
}

fn main() -> iced::Result {
	#[cfg(not(target_arch = "wasm32"))]
	env_logger::init();
	#[cfg(target_arch = "wasm32")]
	console_log::init_with_level(Level::Info);

	PlayerCompanionApp::run(Settings {
		..Default::default()
	})
}
