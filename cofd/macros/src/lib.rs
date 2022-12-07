use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Error, Fields, Variant};

use convert_case::Casing;

macro_rules! derive_error {
	($string: tt) => {
		Error::new(Span::call_site(), $string)
			.to_compile_error()
			.into()
	};
}

fn parse_args(variant: &Variant, map: &mut HashMap<String, String>) {
	for attr in &variant.attrs {
		if attr.path.is_ident("splat") {
			let meta: syn::Meta = attr.parse_meta().unwrap();

			if let syn::Meta::List(list) = meta {
				list.nested.into_pairs().for_each(|pair| {
					if let syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) = pair.value() {
						if let syn::Lit::Str(val) = &nv.lit {
							map.insert(nv.path.get_ident().unwrap().to_string(), val.value());
						}
					}
				});
			}

			break;
		}
	}
}

fn variant_fields(variant: &Variant) -> TokenStream {
	match &variant.fields {
		Fields::Unnamed(_) => quote_spanned! {variant.span()=> (..) },
		Fields::Unit => quote_spanned! { variant.span()=> },
		Fields::Named(_) => quote_spanned! {variant.span()=> {..} },
	}
}

#[proc_macro_derive(SplatEnum, attributes(splat))]
pub fn derive_splat_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = &input.ident;
	let data = &input.data;

	let mut virtue_fun_variants = TokenStream::new();
	let mut vice_fun_variants = TokenStream::new();

	let mut xsplat_fun_variants = TokenStream::new();
	let mut ysplat_fun_variants = TokenStream::new();
	let mut zsplat_fun_variants = TokenStream::new();

	let mut ability_fun_variants = TokenStream::new();
	let mut st_fun_variants = TokenStream::new();
	let mut alt_beats_fun_variants = TokenStream::new();
	let mut fuel_fun_variants = TokenStream::new();
	let mut integrity_fun_variants = TokenStream::new();

	if let Data::Enum(data_enum) = data {
		let mut args = HashMap::new();

		for variant in &data_enum.variants {
			let variant_name = &variant.ident;
			// let attrs = &variant.attrs;
			args.clear();
			parse_args(variant, &mut args);

			let fields_in_variant = variant_fields(variant);

			if let Some(virtue) = args.get("virtue") {
				virtue_fun_variants.extend(quote_spanned! {variant.span()=>
					#name::#variant_name #fields_in_variant => #virtue,
				});
			}

			if let Some(vice) = args.get("vice") {
				vice_fun_variants.extend(quote_spanned! {variant.span()=>
					#name::#variant_name #fields_in_variant => #vice,
				});
			}

			if let Some(xsplat) = args.get("xsplat") {
				xsplat_fun_variants.extend(quote_spanned! {variant.span()=>
					#name::#variant_name #fields_in_variant => Some(#xsplat),
				});
			}

			if let Some(ysplat) = args.get("ysplat") {
				ysplat_fun_variants.extend(quote_spanned! {variant.span()=>
					#name::#variant_name #fields_in_variant => Some(#ysplat),
				});
			}

			if let Some(zsplat) = args.get("zsplat") {
				zsplat_fun_variants.extend(quote_spanned! {variant.span()=>
					#name::#variant_name #fields_in_variant => Some(#zsplat),
				});
			}

			if let Some(ability) = args.get("ability") {
				ability_fun_variants.extend(quote_spanned! {variant.span()=>
					#name::#variant_name #fields_in_variant => Some(#ability),
				});
			}

			if let Some(st) = args.get("st") {
				st_fun_variants.extend(quote_spanned! {variant.span()=>
					#name::#variant_name #fields_in_variant => Some(#st),
				});
			}

			if let Some(alt_beats) = args.get("alt_beats") {
				alt_beats_fun_variants.extend(quote_spanned! {variant.span()=>
					#name::#variant_name #fields_in_variant => Some(#alt_beats),
				});
			}

			if let Some(fuel) = args.get("fuel") {
				fuel_fun_variants.extend(quote_spanned! {variant.span()=>
					#name::#variant_name #fields_in_variant => Some(#fuel),
				});
			}

			if let Some(integrity) = args.get("integrity") {
				integrity_fun_variants.extend(quote_spanned! {variant.span()=>
					#name::#variant_name #fields_in_variant => #integrity,
				});
			}
		}
	} else {
		return derive_error!("SplatEnum is only implemented for enums");
	}

	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let expanded = quote! {
		impl #impl_generics #name #ty_generics #where_clause {
			pub fn virtue_anchor(&self) -> &str {
				match self {
					#virtue_fun_variants
					_ => "virtue",
				}
			}

			pub fn vice_anchor(&self) -> &str {
				match self {
					#vice_fun_variants
					_ => "vice",
				}
			}

			pub fn xsplat_name(&self) -> Option<&str> {
				match self {
					#xsplat_fun_variants
					_ => None,
				}
			}

			pub fn ysplat_name(&self) -> Option<&str> {
				match self {
					#ysplat_fun_variants
					_ => None,
				}
			}

			pub fn zsplat_name(&self) -> Option<&str> {
				match self {
					#zsplat_fun_variants
					_ => None,
				}
			}

			pub fn ability_name(&self) -> Option<&str> {
				match self {
					#ability_fun_variants
					_ => None,
				}
			}

			pub fn supernatural_tolerance(&self) -> Option<&str> {
				match self {
					#st_fun_variants
					_ => None,
				}
			}

			pub fn alternate_beats(&self) -> Option<&str> {
				match self {
					#alt_beats_fun_variants
					_ => None,
				}
			}

			pub fn fuel(&self) -> Option<&str> {
				match self {
					#fuel_fun_variants
					_ => None,
				}
			}

			pub fn integrity(&self) -> &str {
				match self {
					#integrity_fun_variants
					_ => "integrity",
				}
			}
		}
	};

	proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(VariantName)]
pub fn derive_variant_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = &input.ident;
	let data = &input.data;

	let mut name_fun_variants = TokenStream::new();

	if let Data::Enum(data) = data {
		for variant in &data.variants {
			let variant_name = &variant.ident;

			if variant_name.eq("_Custom") {
				if let Fields::Unnamed(fields) = &variant.fields {
					if let Some(_field) = fields.unnamed.first() {
						name_fun_variants.extend(quote_spanned! {variant.span()=>
							#name::#variant_name (name, ..) => name,
						});
					}
				}
			} else {
				let fields_in_variant = variant_fields(variant);
				let variant_name_lower =
					variant_name.to_string().to_case(convert_case::Case::Snake);
				name_fun_variants.extend(quote_spanned! {variant.span()=>
					#name::#variant_name #fields_in_variant => #variant_name_lower,
				});
			}
		}
	} else {
		return derive_error!("VariantName is only implemented for enums");
	}

	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let expanded = quote! {
		impl #impl_generics cofd_traits::VariantName for #name #ty_generics #where_clause {
			fn name(&self) -> &str {
				match self {
					#name_fun_variants
				}
			}
		}
	};

	proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(AllVariants)]
pub fn derive_all_variants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = &input.ident;
	let data = &input.data;

	let mut all_variants = TokenStream::new();
	let mut num: usize = 0;

	if let Data::Enum(data) = data {
		for variant in &data.variants {
			let variant_name = &variant.ident;

			let fields = match &variant.fields {
				// Fields::Unnamed(_) => quote_spanned! {variant.span()=> (..) },
				Fields::Unit => quote_spanned! { variant.span()=> },
				// Fields::Named(_) => quote_spanned! {variant.span()=> {..} },
				_ => continue,
			};

			all_variants.extend(quote_spanned! {variant.span()=>
				#name::#variant_name #fields,
			});
			num += 1;
		}
	} else {
		return derive_error!("AllVariants is only implemented for enums");
	}

	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let expanded = quote! {
		impl #impl_generics cofd_traits::AllVariants for #name #ty_generics #where_clause {
			type T = #name;
			const N: usize = #num;
			fn all() -> [Self::T; Self::N] {
				[
					#all_variants
				]
			}
		}
	};

	proc_macro::TokenStream::from(expanded)
}
