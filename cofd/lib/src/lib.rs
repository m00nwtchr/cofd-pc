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
	use crate::{
		character::{Attributes, Character, Skill, Skills},
		splat::{
			ability::{Ability, AbilityVal},
			vampire::{Bloodline, Clan, Covenant, Discipline, VampireMerits},
			werewolf::{Auspice, Form, Renown, Tribe, WerewolfMerits},
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
					[
						Discipline::Animalism,
						Discipline::Dominate,
						Discipline::Resilience,
						Discipline::Auspex,
					],
				)),
			))
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
					Ability::Merit(Merit::Vampire(VampireMerits::CacophonySavvy)),
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
				AbilityVal(Ability::Merit(Merit::SafePlace(String::from(""))), 3),
				AbilityVal(Ability::Merit(Merit::Resources), 3),
				AbilityVal(
					Ability::Merit(Merit::Vampire(VampireMerits::NestGuardian)),
					1,
				),
			])
			.build();

		println!("{:?}", vampire_character);
		println!("{:?}", vampire_character.attributes());

		// println!("{}", serde_json::to_string_pretty(&character).unwrap());

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
			.with_abilities([
				AbilityVal(Ability::Renown(Renown::Glory), 1),
				AbilityVal(Ability::Renown(Renown::Purity), 3),
			])
			.with_merits([
				AbilityVal(Ability::Merit(Merit::Giant), 3),
				AbilityVal(Ability::Merit(Merit::TrainedObserver), 1),
				AbilityVal(
					Ability::Merit(Merit::DefensiveCombat(true, Skill::Brawl)),
					1,
				),
				AbilityVal(
					Ability::Merit(Merit::Werewolf(WerewolfMerits::FavoredForm(Some(
						Form::Gauru,
					)))),
					2,
				),
				AbilityVal(
					Ability::Merit(Merit::Werewolf(WerewolfMerits::EfficientKiller)),
					2,
				),
				AbilityVal(
					Ability::Merit(Merit::Werewolf(WerewolfMerits::RelentlessAssault)),
					2,
				),
				AbilityVal(
					Ability::Merit(Merit::Language("First Tongue".to_owned())),
					1,
				),
				AbilityVal(Ability::Merit(Merit::Werewolf(WerewolfMerits::Totem)), 1),
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
	}
}
