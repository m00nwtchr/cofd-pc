use crate::i18n;
use cfg_if::cfg_if;
use cofd::prelude::{Template, VariantName};
use cofd::splat::ability::Ability;
use cofd::splat::changeling::Regalia;
use cofd::splat::werewolf::{HuntersAspect, KuruthTriggers, MoonGift, Rite, ShadowGift, WolfGift};
use cofd::splat::{Merit, NameKey, Splat, XSplat, YSplat, ZSplat};
use cofd::template::mage::Arcanum;
use cofd::template::SupernaturalTolerance;
use cofd::traits::TraitCategory;
use cofd::{
	character::InfoTrait,
	splat::werewolf::Form,
	template::{Anchor, Fuel, Integrity},
	traits::{attribute::Attribute, skill::Skill, Trait},
};
use i18n_embed::{
	fluent::{fluent_language_loader, FluentLanguageLoader},
	DefaultLocalizer, LanguageLoader, LanguageRequester, Localizer,
};
use once_cell::sync::{Lazy, OnceCell};
use rust_embed::RustEmbed;
use std::fmt::Display;
use std::ops::Deref;
use std::{
	fmt::{self},
	sync::Arc,
};

#[derive(RustEmbed)]
#[folder = "i18n"] // path to the compiled localization resources
struct Localizations;

pub static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
	let loader = fluent_language_loader!();

	loader
		.load_fallback_language(&Localizations)
		.expect("Error while loading fallback language");

	loader
});

#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::i18n::LANGUAGE_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::i18n::LANGUAGE_LOADER, $message_id, $($args), *)
    }};
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum Locale {
// 	System,
// 	Lang(LanguageIdentifier),
// }

// impl Default for Locale {
// 	fn default() -> Self {
// 		Self::Lang(langid!("en-US"))
// 	}
// }

// impl Display for Locale {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		match self {
// 			Locale::System => f.write_str("System"),
// 			Locale::Lang(id) => f.write_str(&id.to_string()),
// 		}
// 	}
// }

pub fn setup() -> anyhow::Result<Box<dyn LanguageRequester<'static>>> {
	let localizer: Arc<dyn Localizer> =
		Arc::new(DefaultLocalizer::new(&*LANGUAGE_LOADER, &Localizations));

	let mut language_requester = Box::new({
		cfg_if! {
			if #[cfg(target_arch = "wasm32")] {
				i18n_embed::WebLanguageRequester::new()
			} else {
				i18n_embed::DesktopLanguageRequester::new()
			}
		}
	});

	language_requester.add_listener(Arc::downgrade(&localizer));
	language_requester.poll()?;

	LANGUAGE_LOADER.set_use_isolating(false);

	Ok(language_requester)
}

pub trait Translate {
	fn translated(&self) -> String;
}

#[derive(Clone, Eq, PartialEq)]
pub struct Translated<T: Translate>(T);

impl<T: Translate> Translated<T> {
	pub fn unwrap(self) -> T {
		self.0
	}
}

impl<T: Translate> Display for Translated<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&self.0.translated())
	}
}

impl<T: Translate> From<T> for Translated<T> {
	fn from(t: T) -> Self {
		Translated(t)
	}
}

impl<T: Translate> Deref for Translated<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Translate for Attribute {
	fn translated(&self) -> String {
		LANGUAGE_LOADER.get(self.name())
	}
}

impl Translate for Skill {
	fn translated(&self) -> String {
		LANGUAGE_LOADER.get(self.name())
	}
}

impl Translate for Trait {
	fn translated(&self) -> String {
		LANGUAGE_LOADER.get(self.name())
	}
}

impl Translate for Integrity {
	fn translated(&self) -> String {
		LANGUAGE_LOADER.get(self.name())
	}
}

impl Translate for Fuel {
	fn translated(&self) -> String {
		LANGUAGE_LOADER.get(self.name())
	}
}

impl Translate for Form {
	fn translated(&self) -> String {
		LANGUAGE_LOADER.get(self.name())
	}
}

impl Translate for Anchor {
	fn translated(&self) -> String {
		LANGUAGE_LOADER.get(self.name())
	}
}

impl Translate for InfoTrait {
	fn translated(&self) -> String {
		LANGUAGE_LOADER.get(self.name())
	}
}

impl Translate for Regalia {
	fn translated(&self) -> String {
		LANGUAGE_LOADER.get(self.name())
	}
}

impl Translate for KuruthTriggers {
	fn translated(&self) -> String {
		LANGUAGE_LOADER.get(self.name().unwrap_or("custom"))
	}
}

impl Translate for SupernaturalTolerance {
	fn translated(&self) -> String {
		LANGUAGE_LOADER.get(self.name())
	}
}

impl Translate for Arcanum {
	fn translated(&self) -> String {
		LANGUAGE_LOADER.get(self.name())
	}
}

impl Translate for XSplat {
	fn translated(&self) -> String {
		if self.is_custom() {
			self.name().to_string()
		} else {
			LANGUAGE_LOADER.get(self.name())
		}
	}
}

impl Translate for YSplat {
	fn translated(&self) -> String {
		if self.is_custom() {
			self.name().to_string()
		} else {
			LANGUAGE_LOADER.get(self.name())
		}
	}
}

impl Translate for ZSplat {
	fn translated(&self) -> String {
		if self.is_custom() {
			self.name().to_string()
		} else {
			LANGUAGE_LOADER.get(self.name())
		}
	}
}

impl Translate for Merit {
	fn translated(&self) -> String {
		if let Self::_Custom(name) = &self {
			name.clone()
		} else {
			LANGUAGE_LOADER.get(self.name())
		}
	}
}

impl Translate for TraitCategory {
	fn translated(&self) -> String {
		LANGUAGE_LOADER.get(self.name())
	}
}

impl Translate for Template {
	fn translated(&self) -> String {
		LANGUAGE_LOADER.get(self.name())
	}
}

impl Translate for Splat {
	fn translated(&self) -> String {
		LANGUAGE_LOADER.get(self.name())
	}
}

impl Translate for Ability {
	fn translated(&self) -> String {
		match self {
			Self::Haunt(_) => LANGUAGE_LOADER.get_attr("haunts", self.name()),
			Self::Renown(_) => LANGUAGE_LOADER.get_attr("renown", self.name()),
			_ => LANGUAGE_LOADER.get(self.name())
		}
	}
}

impl Translate for HuntersAspect {
	fn translated(&self) -> String {
		if let Self::_Custom(name) = &self {
			name.clone()
		} else {
			LANGUAGE_LOADER.get(self.name())
		}
	}
}

impl Translate for WolfGift {
	fn translated(&self) -> String {
		LANGUAGE_LOADER.get(self.name())
	}
}

impl Translate for ShadowGift {
	fn translated(&self) -> String {
		LANGUAGE_LOADER.get(self.name())
	}
}

impl Translate for MoonGift {
	fn translated(&self) -> String {
		LANGUAGE_LOADER.get(self.name())
	}
}

impl Translate for Rite {
	fn translated(&self) -> String {
		if let Self::_Custom(name) = &self {
			name.clone()
		} else {
			LANGUAGE_LOADER.get(self.name())
		}
	}
}

// impl<T: NameKey> Translate for T {
// 	fn translated(&self) -> String {
// 		fl!("app-name")
// 	}
// }
