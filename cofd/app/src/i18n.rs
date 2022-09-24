use std::sync::Arc;

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
use once_cell::sync::Lazy;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "i18n"] // path to the compiled localization resources
struct Localizations;

pub static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| fluent_language_loader!());

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

	language_requester.add_listener(Arc::downgrade(&localizer_arc));
	language_requester.poll().unwrap();

	println!("{:?}", language_requester.current_languages());



	// i18n_embed::select(&language_loader, &Localizations, &requested_languages).unwrap();
	// localizer.select(requested_languages)
	// language_loader
}
