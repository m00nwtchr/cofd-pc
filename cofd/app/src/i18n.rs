use std::{
	fmt::{self, Display},
	rc::Rc,
	sync::Arc,
};

use cofd::splat::{ability::Ability, changeling::Regalia, Merit, XSplat, YSplat, ZSplat};
use i18n_embed::{
	fluent::{fluent_language_loader, FluentLanguageLoader},
	DefaultLocalizer, LanguageRequester, Localizer,
};
cfg_if! {
	if #[cfg(target_arch = "wasm32")] {
		use i18n_embed::WebLanguageRequester;
	} else {
		use i18n_embed::DesktopLanguageRequester;
	}
}
use cfg_if::cfg_if;
use once_cell::sync::{Lazy, OnceCell};
use rust_embed::RustEmbed;
use unic_langid::langid;

#[derive(RustEmbed)]
#[folder = "i18n"] // path to the compiled localization resources
struct Localizations;

pub static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| fluent_language_loader!());

#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::i18n::LANGUAGE_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::i18n::LANGUAGE_LOADER, $message_id, $($args), *)
    }};
}

pub fn fl(message_id: &str, attribute: Option<&str>) -> Option<String> {
	let mut message = OnceCell::new();
	LANGUAGE_LOADER.with_bundles_mut(|bundle| {
		if message.get().is_none() {
			if let Some(msg) = bundle.get_message(message_id) {
				if let Some(pattern) = if let Some(attribute) = attribute {
					msg.get_attribute(attribute).map(|v| v.value())
				} else {
					msg.value()
				} {
					message
						.set(
							bundle
								.format_pattern(pattern, None, &mut vec![])
								.to_string(),
						)
						.unwrap();
				}
			}
		}
	});
	// println!("{}.{:?}", message_id, attribute);
	message.take()
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

pub fn setup() -> Box<dyn LanguageRequester<'static>> {
	let localizer = DefaultLocalizer::new(&*LANGUAGE_LOADER, &Localizations);
	let localizer_arc: Arc<dyn Localizer> = Arc::new(localizer);

	let mut language_requester = Box::new({
		cfg_if! {
			if #[cfg(target_arch = "wasm32")] {
				WebLanguageRequester::new()
			} else {
				DesktopLanguageRequester::new()
			}
		}
	});

	language_requester.add_listener(Arc::downgrade(&localizer_arc));
	language_requester.poll().unwrap();

	LANGUAGE_LOADER.set_use_isolating(false);

	language_requester
}

#[derive(Clone, PartialEq, Eq)]
pub enum Translated {
	XSplat(XSplat),
	YSplat(YSplat),
	ZSplat(ZSplat),
	Ability(Ability),
	Merit(Merit),
	Regalia(Regalia),
}

// impl Translated {
// 	pub fn unwrap<T>(self) -> &T {
// 		match self {
// 			Translated::Merit(merit) => merit,
// 		}
// 	}
// }

impl<'a> fmt::Display for Translated {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Translated::XSplat(xsplat) => {
				if xsplat.is_custom() {
					write!(f, "{}", xsplat.name())
				} else {
					write!(
						f,
						"{}",
						fl(
							match xsplat {
								&XSplat::Mage(_) => "mage",
								&XSplat::Vampire(_) => "vampire",
								&XSplat::Werewolf(_) => "werewolf",
								&XSplat::Changeling(_) => "changeling",
							},
							Some(&xsplat.name())
						)
						.unwrap_or(xsplat.name().to_string())
					)
				}
			}
			Translated::YSplat(ysplat) => {
				if ysplat.is_custom() {
					write!(f, "{}", ysplat.name())
				} else {
					write!(
						f,
						"{}",
						fl(
							match ysplat {
								&YSplat::Mage(_) => "mage",
								&YSplat::Vampire(_) => "vampire",
								&YSplat::Werewolf(_) => "werewolf",
								&YSplat::Changeling(_) => "changeling",
							},
							Some(&ysplat.name())
						)
						.unwrap_or(ysplat.name().to_string())
					)
				}
			}
			Translated::ZSplat(zsplat) => {
				if zsplat.is_custom() {
					write!(f, "{}", zsplat.name())
				} else {
					write!(
						f,
						"{}",
						fl(
							match zsplat {
								&ZSplat::Mage(_) => "mage",
								&ZSplat::Vampire(_) => "vampire",
								&ZSplat::Werewolf(_) => "werewolf",
								&ZSplat::Changeling(_) => "changeling",
							},
							Some(&zsplat.name())
						)
						.unwrap_or(zsplat.name().to_string())
					)
				}
			}
			Self::Merit(Merit::_Custom(name)) => write!(f, "{}", name),
			Self::Merit(merit) => write!(
				f,
				"{}",
				fl("merits", Some(&merit.name())).unwrap_or(merit.name())
			),
			Translated::Ability(ability) => {
				if ability.is_custom() {
					write!(f, "{}", ability.name())
				} else {
					write!(
						f,
						"{}",
						fl(
							match ability {
								&Ability::Discipline(_) => "vampire",
								&Ability::MoonGift(_) => "werewolf",
								_ => "",
							},
							Some(&ability.name())
						)
						.unwrap_or(ability.name().to_string())
					)
				}
			}
			Translated::Regalia(Regalia::_Custom(name)) => write!(f, "{}", name),
			Translated::Regalia(regalia) => write!(
				f,
				"{}",
				fl("changeling", Some(&regalia.name())).unwrap_or(regalia.name().to_string())
			),
		}
	}
}

impl From<XSplat> for Translated {
	fn from(xsplat: XSplat) -> Self {
		Translated::XSplat(xsplat)
	}
}
impl From<YSplat> for Translated {
	fn from(ysplat: YSplat) -> Self {
		Translated::YSplat(ysplat)
	}
}
impl From<ZSplat> for Translated {
	fn from(zsplat: ZSplat) -> Self {
		Translated::ZSplat(zsplat)
	}
}

impl From<Ability> for Translated {
	fn from(ability: Ability) -> Self {
		Translated::Ability(ability)
	}
}

impl From<Merit> for Translated {
	fn from(merit: Merit) -> Self {
		Translated::Merit(merit)
	}
}

impl From<Regalia> for Translated {
	fn from(regalia: Regalia) -> Self {
		Translated::Regalia(regalia)
	}
}
