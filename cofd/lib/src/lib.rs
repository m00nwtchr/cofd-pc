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
			vampire::{self, Bloodline, Clan, Covenant, Discipline, DisciplineAbility},
			werewolf::{Auspice, Renown, RenownAbility, Tribe},
			AbilityKey, Merit, MeritAbility, Splat,
		},
	};

	#[test]
	fn it_works() {
		let voivode = Discipline::_Custom("Coil of the Voivode".to_string());
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
			// .with_abilities([
			// 	(
			// 		AbilityKey::Discipline(Discipline::Animalism),
			// 		Box::new(DisciplineAbility(1, Discipline::Animalism)),
			// 	),
			// 	(
			// 		AbilityKey::Discipline(Discipline::Dominate),
			// 		Box::new(DisciplineAbility(2, Discipline::Dominate)),
			// 	),
			// 	(
			// 		AbilityKey::Discipline(voivode.clone()),
			// 		Box::new(DisciplineAbility(1, voivode)),
			// 	),
			// ])
			.with_merits([
				(
					AbilityKey::Merit(Merit::Status("Ordo Dracul".to_string())),
					MeritAbility(1, Merit::Status("Ordo Dracul".to_string())),
				),
				(
					AbilityKey::Merit(Merit::Status("City".to_string())),
					MeritAbility(1, Merit::Status("City".to_string())),
				),
				(
					AbilityKey::Merit(Merit::CacophonySavvy),
					MeritAbility(3, Merit::CacophonySavvy),
				),
				(
					AbilityKey::Merit(Merit::FastTalking),
					MeritAbility(1, Merit::FastTalking),
				),
				(
					AbilityKey::Merit(Merit::ProfessionalTraining(
						"".to_string(),
						[Skill::Expression, Skill::Occult],
						None,
					)),
					MeritAbility(
						2,
						Merit::ProfessionalTraining(
							"".to_string(),
							[Skill::Expression, Skill::Occult],
							None,
						),
					),
				),
				// (
				// 	AbilityKey::Merit(Merit::Contacts("".to_string())),
				// 	MeritAbility(2, Merit::Contacts("".to_string())),
				// ),
				(
					AbilityKey::Merit(Merit::SafePlace("".to_string())),
					MeritAbility(3, Merit::SafePlace("".to_string())),
				),
				(
					AbilityKey::Merit(Merit::Resources),
					MeritAbility(3, Merit::Resources),
				),
				(
					AbilityKey::Merit(Merit::NestGuardian),
					MeritAbility(1, Merit::NestGuardian),
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
				brawl: 3,
				stealth: 2,
				survival: 3,
				intimidation: 3,
				persuasion: 4,
				..Default::default()
			})
			// .with_abilities([
			// 	(
			// 		AbilityKey::Renown(Renown::Glory),
			// 		Box::new(RenownAbility(1)),
			// 	),
			// 	(
			// 		AbilityKey::Renown(Renown::Purity),
			// 		Box::new(RenownAbility(3)),
			// 	),
			// ])
			.build();

		werewolf_character.power = 3;

		assert_eq!(werewolf_character.max_fuel(), 12);
	}
}
