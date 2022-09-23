use i18n_embed::fluent::{fluent_language_loader, FluentLanguageLoader};
cfg_if! {
	if #[cfg(target_arch = "wasm32")] {
		use i18n_embed::WebLanguageRequester;
	} else {
		use i18n_embed::DesktopLanguageRequester;
	}
}
use rust_embed::RustEmbed;

use cfg_if::cfg_if;

#[derive(RustEmbed)]
#[folder = "i18n"] // path to the compiled localization resources
struct Localizations;

pub fn load() -> FluentLanguageLoader {
	let language_loader: FluentLanguageLoader = fluent_language_loader!();

	let requested_languages = {
		cfg_if! {
			if #[cfg(target_arch = "wasm32")] {
				WebLanguageRequester::requested_languages()
			} else {
				DesktopLanguageRequester::requested_languages()
			}
		}
	};

	i18n_embed::select(&language_loader, &Localizations, &requested_languages).unwrap();

	language_loader
}
