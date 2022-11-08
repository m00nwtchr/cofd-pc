#![feature(is_some_and)]
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

use std::{cell::RefCell, fs::File, io::Write, mem, path::PathBuf, rc::Rc};

use directories::ProjectDirs;
use iced::{
	executor,
	widget::{button, row, Column},
	Application, Command, Element, Settings, Theme,
};

#[cfg(target_arch = "wasm32")]
use log::Level;

use cofd::{prelude::*, splat::Splat};

mod component;
mod i18n;
mod view;
mod widget;

use i18n::fl;
use ron::ser::PrettyConfig;

#[derive(Debug, Clone)]
pub enum Tab {
	Overview,
	Equipment,

	Forms,
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

	project_dirs: ProjectDirs,
	// character: Rc<RefCell<Character>>,
	// custom_xsplats: Vec<XSplat>,
	// locale: Locale,
	// language_requester: Box<dyn LanguageRequester<'static>>,
}

const H2_SIZE: u16 = 25;
const H3_SIZE: u16 = 20;

const MAX_INPUT_WIDTH: u32 = 200;
pub const INPUT_PADDING: u16 = 1;

const TITLE_SPACING: u16 = 2;
const COMPONENT_SPACING: u16 = 8;

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
	Msg,

	Save,
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

	pub fn save(&self) -> anyhow::Result<()> {
		let vec: Vec<Character> = self
			.characters
			.iter()
			.map(|rip| rip.borrow().clone())
			.collect();

		let path = self.save_path();
		let dir = path.parent();
		if dir.is_some() && !dir.unwrap().exists() {
			std::fs::create_dir_all(dir.unwrap())?;
		}

		let val = ron::ser::to_string_pretty(&vec, PrettyConfig::default())?;
		let mut file = File::create(path)?;

		file.write_all(val.as_bytes())?;

		Ok(())
	}

	pub fn load(&mut self) -> anyhow::Result<()> {
		let str = std::fs::read_to_string(self.save_path())?;
		let characters: Vec<Character> = ron::de::from_str(&str)?;

		self.characters = characters
			.into_iter()
			.map(|mut val| {
				val.calc_mod_map();
				val
			})
			.map(|val| Rc::new(RefCell::new(val)))
			.collect();

		Ok(())
	}

	fn save_path(&self) -> PathBuf {
		self.project_dirs.data_dir().join("characters.ron")
	}
}

impl Application for PlayerCompanionApp {
	type Executor = executor::Default;
	type Flags = ();
	type Message = Message;
	type Theme = Theme;

	fn new(_flags: ()) -> (Self, Command<Self::Message>) {
		let _language_requester = i18n::setup();

		let mut self_ = Self {
			state: State::CharacterList,
			prev_state: Default::default(),
			characters: demo::characters().map(|f| Rc::new(RefCell::new(f))).into(),
			project_dirs: ProjectDirs::from("", "", "cofd-pc").unwrap(),
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

		self_.load();

		(self_, Command::none())
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
			Message::Msg => {}

			Message::Save => match self.save() {
				Ok(_) => {}
				Err(err) => {
					log::error!("{}", err);
				}
			},
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
				let brw = character.borrow();

				let tab: Element<Self::Message> = match active_tab {
					Tab::Overview => view::overview_tab(character.clone()).into(),
					Tab::Equipment => view::equipment_tab(character.clone()).into(),
					Tab::Forms => {
						if let Splat::Werewolf(_, _, _, _) = brw.splat {
							view::werewolf::form_tab(character.clone(), Message::Msg).into()
						} else {
							unreachable!()
						}
					}
				};

				let mut row = row![
					button("Back").on_press(Message::Previous),
					button("Save").on_press(Message::Save),
					button("Home").on_press(Message::TabSelected(Tab::Overview)),
				];

				if let Splat::Werewolf(_, _, _, data) = &brw.splat {
					row = row.push(button("Forms").on_press(Message::TabSelected(Tab::Forms)));
				}

				row = row.push(button("Equipment").on_press(Message::TabSelected(Tab::Equipment)));

				Column::new().push(row).spacing(1).push(tab).into()
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

// TODO: Add demo mortal.
mod demo {
	use std::{fs::File, io::Write};

	use cofd::{
		character::CharacterInfo,
		prelude::*,
		splat::{changeling::*, mage::*, vampire::*, werewolf::*, Merit, Splat},
	};
	use directories::ProjectDirs;
	use ron::ser::PrettyConfig;

	#[test]
	pub fn save() -> anyhow::Result<()> {
		let vec = characters();

		let val = ron::ser::to_string_pretty(&vec, PrettyConfig::default())?;
		let mut file = File::create(
			ProjectDirs::from("", "", "cofd-pc")
				.unwrap()
				.data_dir()
				.join("characters.ron"),
		)?;

		file.write_all(val.as_bytes())?;

		Ok(())
	}

	#[allow(clippy::too_many_lines)]
	pub fn characters() -> [Character; 5] {
		let character = Character::builder().build();

		let vampire_character = Character::builder()
			.with_splat(Splat::Vampire(
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
				VampireData {
					attr_bonus: Some(Attribute::Presence),
					..Default::default()
				},
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
				WerewolfData {
					skill_bonus: Some(Skill::Brawl),
					triggers: KuruthTriggers::Moon,
					..Default::default()
				},
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

		let changeling_character = Character::builder()
			.with_splat(Splat::Changeling(
				Seeming::Wizened,
				Some(Court::Autumn),
				None,
				ChangelingData {
					attr_bonus: Some(Attribute::Dexterity),
					regalia: Some(Regalia::Crown),
					..Default::default()
				},
			))
			.with_info(CharacterInfo {
				// name: String::from("Darren Webb"),
				player: String::from("m00n"),
				// chronicle: String::from("Night Trains"),
				// virtue_anchor: String::from("Scholar"),
				// vice_anchor: String::from("Authoritarian"),
				concept: String::from("Fae Magic Enthusiast"),
				..Default::default()
			})
			.with_attributes(Attributes {
				..Default::default()
			})
			.with_skills(Skills {
				..Default::default()
			})
			// .with_specialties(Skill::Larceny, vec![String::from("Sleight of Hand")])
			// .with_specialties(Skill::Streetwise, vec![String::from("Rumours")])
			// .with_specialties(Skill::Subterfuge, vec![String::from("Detecting Lies")])
			.with_merits([])
			.build();

		[
			character,
			vampire_character,
			mage_character,
			werewolf_character,
			changeling_character,
		]
	}
}
