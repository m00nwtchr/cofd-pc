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

use std::{cell::RefCell, rc::Rc};
#[cfg(target_arch = "wasm32")]
use log::Level;

use iced::{executor, widget::container, Application, Command, Element, Settings, Theme};
// use iced_aw::pure::{TabLabel, Tabs};

// use i18n_embed::LanguageRequester;

use cofd::{
	prelude::*,
	splat::{
		ability::{Ability, AbilityVal},
		changeling::{Court, Seeming},
		vampire::{Discipline, VampireMerit, Clan, Covenant, Bloodline},
		Merit, Splat, mage::{Path, Order},
	},
};

mod component;
mod i18n;
mod view;
mod widget;

use i18n::fl;

struct PlayerCompanionApp {
	active_tab: usize,
	character: Rc<RefCell<Character>>,
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
	TabSelected(usize),
}

impl PlayerCompanionApp {}

impl Application for PlayerCompanionApp {
	type Executor = executor::Default;
	type Flags = ();
	type Message = Message;
	type Theme = Theme;

	fn new(_flags: ()) -> (Self, Command<Self::Message>) {
		let _language_requester = i18n::setup();

		let character = Character::builder()
			.with_st(3)
			// .with_splat(Splat::Changeling(
			// 	Seeming::Wizened,
			// 	// Seeming::_Custom("bler".to_string(), Regalia::Jewels),
			// 	Some(Court::Autumn),
			// 	None,
			// 	Default::default(),
			// ))
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
				Default::default()
			))
			// .with_splat(Splat::Mage(Path::Mastigos, Some(Order::Mysterium), None))
			// .with_splat(Splat::Werewolf(
			// 	Some(Auspice::Rahu),
			// 	Some(Tribe::BloodTalons),
			// 	None,
			// 	Default::default(),
			// ))
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
			.with_abilities([
				AbilityVal(Ability::Discipline(Discipline::Animalism), 1),
				AbilityVal(Ability::Discipline(Discipline::Dominate), 2),
				AbilityVal(
					Ability::Discipline(Discipline::_Custom("Coil of the Voivode".to_string())),
					2,
				),
			])
			.with_merits([
				AbilityVal(Ability::Merit(Merit::Status("Ordo Dracul".to_string())), 1),
				AbilityVal(Ability::Merit(Merit::Status("City".to_string())), 1),
				AbilityVal(
					Ability::Merit(Merit::Vampire(VampireMerit::CacophonySavvy)),
					3,
				),
				AbilityVal(Ability::Merit(Merit::FastTalking), 1),
				AbilityVal(
					Ability::Merit(Merit::ProfessionalTraining(
						String::new(),
						Some([Skill::Expression, Skill::Occult]),
						None,
					)),
					2,
				),
				// AbilityVal(Ability::Merit(Merit::Contacts(String::new())), 2),
				AbilityVal(Ability::Merit(Merit::SafePlace(String::new())), 3),
				AbilityVal(Ability::Merit(Merit::Resources), 3),
				AbilityVal(
					Ability::Merit(Merit::Vampire(VampireMerit::NestGuardian)),
					1,
				),
			])
			.build();

		(
			Self {
				active_tab: 0,
				character: Rc::new(RefCell::new(character)),
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
			Message::TabSelected(tab) => self.active_tab = tab,
		}

		Command::none()
	}

	fn view(&self) -> Element<Self::Message> {
		container(view::overview_tab(self.character.clone())).into()
		// Tabs::new(self.active_tab, Message::TabSelected)
		// .push(
		// 	TabLabel::Text(String::from("Overview")),
		// 	self.overview_tab(),
		// )
		// .push(TabLabel::Text("UwU".to_string()), self.overview_tab())
		// .into()
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
