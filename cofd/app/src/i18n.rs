use std::{fmt::Display, rc::Rc, sync::Arc};

use i18n_embed::{
	fluent::{fluent_language_loader, FluentLanguageLoader},
	unic_langid::LanguageIdentifier,
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

pub fn fl(message_id: &str, attribute: Option<&str>) -> String {
	let message = Rc::new(OnceCell::new());
	let message_clone = message.clone();
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
	message_clone.get().unwrap().clone()
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

	language_requester
}
