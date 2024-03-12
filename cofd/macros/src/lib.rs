#![feature(let_chains)]
use std::{env, fs, path::Path};

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Error;

use cofd_util::scraper::{Gift, Merit};

macro_rules! derive_error {
	($string: tt) => {
		Error::new(Span::call_site(), $string)
			.to_compile_error()
			.into()
	};
}

#[proc_macro]
pub fn gifts(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let path = Path::new(&env::var("CARGO_MANIFEST_DIR").expect(""))
		.join("data")
		.join("gifts.ron");

	let Ok(str) = fs::read_to_string(path) else {
		return derive_error!("Error reading gits.ron file");
	};

	let vec: Vec<Gift> = ron::from_str(&str).expect("Parsing error");

	let mut moon_gift_variants = TokenStream::new();
	let mut shadow_gift_variants = TokenStream::new();
	let mut wolf_gift_variants = TokenStream::new();
	let mut facet_variants = TokenStream::new();

	let mut shadow_gift_facets_variants = TokenStream::new();
	let mut wolf_gift_facets_variants = TokenStream::new();

	for gift in vec {
		if let Ok(name) = gift.name.parse::<TokenStream>() {
			let (ts, ts2) = match gift.type_.as_str() {
				"Moon" => (&mut moon_gift_variants, None),
				"Shadow" => (
					&mut shadow_gift_variants,
					Some(&mut shadow_gift_facets_variants),
				),
				"Wolf" => (
					&mut wolf_gift_variants,
					Some(&mut wolf_gift_facets_variants),
				),
				_ => return derive_error!("Unkown type"),
			};

			ts.extend(quote! {
				#name,
			});

			let mut facets_arr = TokenStream::new();
			for facet in gift.facets {
				if let Ok(facet_name) = facet.name().parse::<TokenStream>() {
					facet_variants.extend(quote! {
						#facet_name,
					});
					facets_arr.extend(quote! {
						Facet::#facet_name,
					});
				}
			}
			if let Some(ts2) = ts2 {
				ts2.extend(quote! {
					Self::#name => &[
						#facets_arr
					],
				});
			}
		}
	}

	let expanded = quote! {
		#[derive(
			Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, VariantName, AllVariants, Hash
		)]
		pub enum MoonGift {
			#moon_gift_variants
			_Custom(String)
		}

		#[derive(
			Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, VariantName, AllVariants,
		)]
		pub enum ShadowGift {
			#shadow_gift_variants
			_Custom(String, [Facet; 5])
		}

		#[derive(
			Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, VariantName, AllVariants,
		)]
		pub enum WolfGift {
			#wolf_gift_variants
			_Custom(String, [Facet; 5])
		}

		#[derive(
			Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, VariantName, AllVariants,
		)]
		pub enum Facet {
			#facet_variants
			_Custom(String)
		}

		impl ShadowGift {
			pub fn get_facets(&self) -> &[Facet; 5] {
				match self {
					#shadow_gift_facets_variants
					Self::_Custom(.., facets) => facets
				}
			}
		}

		impl WolfGift {
			pub fn get_facets(&self) -> &[Facet; 5] {
				match self {
					#wolf_gift_facets_variants
					Self::_Custom(.., facets) => facets
				}
			}
		}
	};

	proc_macro::TokenStream::from(expanded)
}

#[proc_macro]
pub fn merits(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let path = Path::new(&env::var("CARGO_MANIFEST_DIR").expect(""))
		.join("data")
		.join("merits_universal.ron");

	let Ok(str) = fs::read_to_string(path) else {
		return derive_error!("Error reading merits_universal.ron file");
	};

	let vec: Vec<Merit> = ron::from_str(&str).expect("Parsing error");
	for merit in vec {}

	let expanded = quote! {};

	proc_macro::TokenStream::from(expanded)
}
