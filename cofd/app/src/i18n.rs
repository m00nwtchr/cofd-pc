use std::{rc::Rc, sync::Arc};

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
		if let None = message.get() {
			let msg = bundle.get_message(message_id).unwrap();

			let pattern = if let Some(attribute) = attribute {
				msg.get_attribute(attribute).unwrap().value()
			} else {
				msg.value().unwrap()
			};

			message
				.set(
					bundle
						.format_pattern(pattern, None, &mut vec![])
						.to_string(),
				)
				.unwrap();
		}
	});
	message_clone.get().unwrap().clone()
}

pub fn setup() {
	let localizer = DefaultLocalizer::new(&*LANGUAGE_LOADER, &Localizations);
	let localizer_arc: Arc<dyn Localizer> = Arc::new(localizer);

	let mut language_requester = {
		cfg_if! {
			if #[cfg(target_arch = "wasm32")] {
				WebLanguageRequester::new()
			} else {
				DesktopLanguageRequester::new()
			}
		}
	};

	// language_requester.set_language_override(Some(LanguageIdentifier::from_parts(
	// 	"en".parse().expect("msg"),
	// 	None,
	// 	None,
	// 	&[],
	// )));

	language_requester.add_listener(Arc::downgrade(&localizer_arc));
	language_requester.poll().unwrap();

	// localizer.select(requested_languages)

	// i18n_embed::select(&language_loader, &Localizations, &requested_languages).unwrap();
	// localizer.select(requested_languages)
	// language_loader
}
