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

pub mod character;
pub mod splat;

pub mod prelude {
	pub use crate::character::{Attribute, Attributes, Character, Skill, Skills};
}

#[cfg(test)]
mod tests {
	use ron::ser::PrettyConfig;

	use crate::{
		character::{Attributes, Character, CharacterInfo, Skill, Skills},
		prelude::Attribute,
		splat::{
			mage::{Arcanum, MageData, MageMerit, Order, Path},
			vampire::{Bloodline, Clan, Covenant, Discipline, VampireMerit},
			werewolf::{Auspice, Form, Renown, Tribe, WerewolfMerit},
			Merit, Splat,
		},
	};

	#[test]
	#[allow(clippy::too_many_lines)]
	fn it_works() {
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

		println!("{:?}", vampire_character);
		println!("{:?}", vampire_character.attributes());

		println!(
			"{}",
			ron::ser::to_string_pretty(&vampire_character, PrettyConfig::default()).unwrap()
		);

		assert_eq!(vampire_character.max_health(), 7);
		assert_eq!(vampire_character.attributes().strength, 1);
		assert_eq!(vampire_character.max_fuel(), 10);

		let mut werewolf_character = Character::builder()
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

		werewolf_character.power = 3;

		println!("{:?}", werewolf_character);

		assert_eq!(werewolf_character.max_fuel(), 12);
		assert_eq!(werewolf_character.defense(), 6);
		assert_eq!(werewolf_character.perception(), 7);
		assert_eq!(werewolf_character.max_health(), 12);

		if let Splat::Werewolf(_, _, _, ww) = &mut werewolf_character.splat {
			ww.form = Form::Gauru;
		}

		assert_eq!(werewolf_character.perception(), 7);

		let t = std::time::Instant::now();
		werewolf_character.calc_mod_map();
		println!("{:?}", std::time::Instant::now().duration_since(t));

		assert_eq!(werewolf_character.perception(), 9);

		let mut mage_character = Character::builder()
			.with_splat(Splat::Mage(
				Path::Mastigos,
				Some(Order::Mysterium),
				None,
				MageData {
					attr_bonus: Attribute::Intelligence,
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
				resolve: 5,
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

		mage_character.calc_mod_map();

		if let Splat::Mage(_, _, _, data) = &mut mage_character.splat {
			data.attr_bonus = Attribute::Resolve;
		}

		mage_character.calc_mod_map();

		assert_ne!(mage_character.attributes().resolve, 6);

		mage_character.base_attributes_mut().resolve = 4;
		assert_eq!(mage_character.attributes().resolve, 5);
	}
}
