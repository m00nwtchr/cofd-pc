#![feature(let_chains)]
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

use std::{
	fmt::{Debug, Formatter},
	mem,
};

use cofd::prelude::*;
use iced::{
	executor,
	widget::{button, column},
	Alignment, Application, Command, Element, Length, Settings, Theme,
};
#[cfg(target_arch = "wasm32")]
use log::Level;

mod component;
mod i18n;
mod store;
mod view;
mod widget;

use store::Store;

use crate::view::{character_list::CharacterList, sheet::SheetView, *};

const H2_SIZE: u16 = 25;
const H3_SIZE: u16 = 20;

const MAX_INPUT_WIDTH: f32 = 200f32;
pub const INPUT_PADDING: u16 = 1;

const TITLE_SPACING: u16 = 2;
const COMPONENT_SPACING: u16 = 8;

pub enum View {
	CharacterList(character_list::CharacterList),
	CharacterCreator,
	Sheet(sheet::SheetView, usize),
}

struct PlayerCompanionApp {
	view: View,
	prev_view: Option<View>,
	characters: Vec<Character>,

	store: Store,
	// locale: Locale,
	// language_requester: Box<dyn LanguageRequester<'static>>,
}

#[derive(Clone)]
enum Message {
	AddCharacter(Character),
	NewCharacter,

	CharacterList(character_list::Message),
	Sheet(sheet::Message),
}

impl Debug for Message {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "")
	}
}

impl PlayerCompanionApp {
	pub fn prev(&mut self) {
		if let Some(state) = self.prev_view.take() {
			self.view = state;
		}
	}
	pub fn next(&mut self, mut state: View) {
		mem::swap(&mut self.view, &mut state);
		self.prev_view = Some(state);
	}

	pub fn save(&self) -> anyhow::Result<()> {
		self.store.set("characters", &self.characters)?;
		Ok(())
	}

	pub fn load(&mut self) -> anyhow::Result<()> {
		let characters: Vec<Character> = self
			.store
			.get("characters")?
			.unwrap_or_else(demo::characters);

		self.characters = characters
			.into_iter()
			.inspect(|val| {
				val.calc_mod_map();
			})
			.collect();

		Ok(())
	}
}

impl Application for PlayerCompanionApp {
	type Executor = executor::Default;
	type Message = Message;
	type Theme = Theme;
	type Flags = ();

	fn new(_flags: ()) -> (Self, Command<Self::Message>) {
		let _language_requester = i18n::setup();

		if let Err(err) = _language_requester {
			println!("{err:?}");
		}

		let store = Store::new().expect("Data store not available");

		let mut self_ = Self {
			view: View::CharacterList(CharacterList::new()),
			// state: State::CharacterCreator,
			prev_view: Default::default(),
			characters: Vec::new(),
			store,
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
		};

		if let Err(err) = self_.load() {
			log::error!("{}", err);
		}

		(self_, Command::none())
	}

	fn title(&self) -> String {
		// fl!("app-name")
		String::from("App")
	}

	fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
		match message {
			Message::CharacterList(message) => {
				if let View::CharacterList(view) = &mut self.view {
					let character_list::Action::PickCharacter(i) = view.update(message);
					self.next(View::Sheet(SheetView::new(), i));
				}
			}

			Message::Sheet(sheet::Message::Back) => self.prev(),
			Message::Sheet(sheet::Message::Save) => match self.save() {
				Ok(()) => {}
				Err(err) => {
					log::error!("{}", err);
				}
			},

			Message::Sheet(message) => {
				if let View::Sheet(view, character) = &mut self.view {
					view.update(message, self.characters.get_mut(*character).unwrap());
				}
			}

			Message::AddCharacter(character) => {
				self.characters.push(character);
				self.next(View::CharacterList(CharacterList::new()));
			}
			Message::NewCharacter => {
				self.next(View::CharacterCreator);
			}
		}

		#[cfg(target_arch = "wasm32")]
		{
			use iced::{window, window::Id};
			let window = web_sys::window().unwrap();
			let (width, height) = (
				window.inner_width().unwrap().as_f64().unwrap() as f32,
				window.inner_height().unwrap().as_f64().unwrap() as f32,
			);
			window::resize(Id::MAIN, Size { width, height })
		}
		#[cfg(not(target_arch = "wasm32"))]
		Command::none()
	}

	fn view(&self) -> Element<Self::Message, Self::Theme> {
		match &self.view {
			View::CharacterList(view) => column![
				view.view(&self.characters).map(Message::CharacterList),
				button("New Character").on_press(Message::NewCharacter)
			]
			.width(Length::Fill)
			.align_items(Alignment::Center)
			.into(),

			View::CharacterCreator => view::creator_view(Message::AddCharacter).into(),
			View::Sheet(view, character) => view
				.view(self.characters.get(*character).unwrap())
				.map(Message::Sheet),
		}
	}
}

fn main() -> anyhow::Result<()> {
	#[cfg(not(target_arch = "wasm32"))]
	env_logger::init();
	#[cfg(target_arch = "wasm32")]
	{
		console_log::init_with_level(Level::Warn).map_err(|err| anyhow::anyhow!(err))?;
		std::panic::set_hook(Box::new(console_error_panic_hook::hook));
	}

	PlayerCompanionApp::run(Settings {
		..Settings::default()
	})?;
	Ok(())
}

mod demo {
	use cofd::{
		character::CharacterInfo,
		prelude::*,
		splat::{changeling::*, geist::*, mage::*, vampire::*, werewolf::*, Merit},
	};

	#[allow(unused_imports)]
	use crate::store::Store;

	#[test]
	pub fn save() -> anyhow::Result<()> {
		let vec = characters();
		let store = Store::new().unwrap();

		store.set("characters", &vec)?;
		Ok(())
	}

	#[allow(clippy::too_many_lines)]
	pub fn characters() -> Vec<Character> {
		let character = Character::builder().build();

		let vampire_character = Character::builder()
			.with_splat(
				Vampire::new(
					Clan::Ventrue,
					Some(Covenant::OrdoDracul),
					Some(Bloodline::_Custom(
						"Dragolescu".to_string(),
						Some([
							Discipline::Animalism,
							Discipline::Dominate,
							Discipline::Resilience,
							Discipline::Auspex,
						]),
					)),
				)
				.with_attr_bonus(Attribute::Resolve),
			)
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
				presence: 2,
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
					Merit::ProfessionalTraining {
						profession: String::new(),
						skills: [Skill::Expression, Skill::Occult],
						skill: None,
					},
					2,
				),
				// (Merit::Contacts(String::new()), 2),
				(Merit::SafePlace(String::new()), 3),
				(Merit::Resources, 3),
				(VampireMerit::NestGuardian.into(), 1),
			])
			.build();

		let werewolf_character = Character::builder()
			.with_splat(
				Werewolf::new()
					.with_auspice(Auspice::Rahu)
					.with_tribe(Tribe::BloodTalons),
			)
			// .with_splat(Splat::Werewolf(
			// 	Some(Auspice::Rahu),
			// 	Some(Tribe::BloodTalons),
			// 	None,
			// 	Box::new(WerewolfData {
			// 		skill_bonus: Some(Skill::Brawl),
			// 		triggers: KuruthTriggers::Moon,
			// 		shadow_gifts: vec![
			// 			ShadowGift::Rage,     // Slaughterer (Purity)
			// 			ShadowGift::Strength, // Primal Strength (Purity)
			// 		],
			// 		wolf_gifts: vec![
			// 			WolfGift::Change, // Father's Form
			// 		],
			// 		rites: vec![Rite::SacredHunt],
			// 		..Default::default()
			// 	}),
			// ))
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
				(WerewolfMerit::FavoredForm { form: Form::Gauru }.into(), 2),
				(WerewolfMerit::EfficientKiller.into(), 2),
				(Merit::RelentlessAssault, 2),
				(Merit::Language("First Tongue".to_owned()), 1),
				(WerewolfMerit::Totem.into(), 1),
			])
			.build();

		let mut mage_character = Character::builder()
			.with_splat(Mage::new(Path::Mastigos).with_order(Order::Mysterium))
			// .with_splat(Splat::Mage(
			// 	Path::Mastigos,
			// 	Some(Order::Mysterium),
			// 	None,
			// 	Box::new(Mage {
			// 		attr_bonus: Some(Attribute::Resolve),
			// 		obsessions: vec!["Open the Gate".to_string()],
			// 		rotes: vec![
			// 			Rote {
			// 				arcanum: Arcanum::Space,
			// 				level: 3,
			// 				spell: "Co-Location".to_string(),
			// 				creator: String::new(),
			// 				skill: Skill::Occult,
			// 			},
			// 			Rote {
			// 				arcanum: Arcanum::Prime,
			// 				level: 2,
			// 				spell: "Supernal Veil".to_string(),
			// 				creator: String::new(),
			// 				skill: Skill::Occult,
			// 			},
			// 			Rote {
			// 				arcanum: Arcanum::Space,
			// 				level: 3,
			// 				spell: "Perfect Sympathy".to_string(),
			// 				creator: String::new(),
			// 				skill: Skill::Occult,
			// 			},
			// 		],
			// 	}),
			// ))
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
				resolve: 2,
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
					Merit::ProfessionalTraining {
						profession: String::new(),
						skills: [Skill::Investigation, Skill::Science],
						skill: None,
					},
					3,
				),
				(Merit::TrainedObserver, 1),
				//
				//
			])
			.build();

		mage_character.aspirations = vec!["Solve the Mentor's riddle (Long Term)".to_string()];

		let changeling_character = Character::builder()
			.with_splat(Changeling::new(Seeming::Wizened))
			// .with_splat(Splat::Changeling(
			// 	Seeming::Wizened,
			// 	Some(Court::Autumn),
			// 	None,
			// 	Box::new(ChangelingData {
			// 		attr_bonus: Some(Attribute::Dexterity),
			// 		regalia: Some(Regalia::Crown),
			// 		contracts: vec![Default::default()],
			// 		..Default::default()
			// 	}),
			// ))
			.with_info(CharacterInfo {
				// name: String::from("Darren Webb"),
				player: String::from("m00n"),
				// chronicle: String::from("Night Trains"),
				// virtue_anchor: String::from("Scholar"),
				// vice_anchor: String::from("Authoritarian"),
				concept: String::from("Fae Magic Enthusiast"),
				..Default::default()
			})
			.with_attributes(Default::default())
			.with_skills(Default::default())
			// .with_specialties(Skill::Larceny, vec![String::from("Sleight of Hand")])
			// .with_specialties(Skill::Streetwise, vec![String::from("Rumours")])
			// .with_specialties(Skill::Subterfuge, vec![String::from("Detecting Lies")])
			.with_merits([])
			.build();

		let bound_character = Character::builder()
			// .with_splat(Splat::Bound(
			// 	Burden::Bereaved,
			// 	Archetype::Mourners,
			// 	BoundData {
			// 		keys: vec![Key::Stillness],
			// 	},
			// ))
			.with_splat(Bound {
				burden: Burden::Bereaved,
				archetype: Archetype::Mourners,
				keys: vec![Key::Stillness],
			})
			.with_info(CharacterInfo {
				// name: String::from("Darren Webb"),
				player: String::from("m00n"),
				// chronicle: String::from("Night Trains"),
				// virtue_anchor: String::from("Scholar"),
				// vice_anchor: String::from("Authoritarian"),
				concept: String::from("Dancing with your Ghost // Lost and Found"),
				..Default::default()
			})
			.with_attributes(Attributes {
				intelligence: 3,
				wits: 3,
				resolve: 2,
				strength: 2,
				dexterity: 2,
				stamina: 2,
				presence: 2,
				manipulation: 3,
				composure: 2,
			})
			.with_skills(Skills {
				academics: 1,
				computer: 1,
				investigation: 3,
				medicine: 2,
				occult: 2,
				politics: 1,
				larceny: 1,
				weaponry: 1,
				empathy: 1,
				persuasion: 1,
				streetwise: 2,
				subterfuge: 3,
				..Default::default()
			})
			.with_merits([])
			.build();

		vec![
			character,
			vampire_character,
			mage_character,
			werewolf_character,
			changeling_character,
			bound_character,
		]
	}
}
