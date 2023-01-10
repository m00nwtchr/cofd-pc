use std::{
	fmt::{self},
	sync::Arc,
};

use cofd::splat::NameKey;
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

pub fn flt(message_id: &str, attribute: Option<&str>) -> Option<String> {
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
	message.take()
}

pub fn fll(key: &str) -> Option<String> {
	let mut iter = key.split('.');

	let message_id = iter.next().unwrap();
	let attribute = iter.next();

	flt(message_id, attribute)
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
	language_requester.poll()?;

	LANGUAGE_LOADER.set_use_isolating(false);

	Ok(language_requester)
}

#[derive(Clone, PartialEq, Eq)]
pub struct Translated<T: NameKey>(T);

impl<T: NameKey> Translated<T> {
	pub fn unwrap(self) -> T {
		self.0
	}
}

impl<T: NameKey> fmt::Display for Translated<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let name_key = self.0.name_key();
		let mut iter = name_key.split('.');

		let msg_id = iter.next().unwrap();
		let attr = iter.next();
		write!(
			f,
			"{}",
			flt(msg_id, attr).unwrap_or_else(|| if let Some(name) = attr {
				name.to_string()
			} else {
				name_key.to_string()
			})
		)
	}
}

impl<T: NameKey> From<T> for Translated<T> {
	fn from(t: T) -> Self {
		Translated(t)
	}
}
