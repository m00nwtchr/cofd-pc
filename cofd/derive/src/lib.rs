#![feature(let_chains)]
use std::{collections::HashMap, env, fs, path::Path};

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
	parse_macro_input, spanned::Spanned, Data, DeriveInput, Error, Fields, GenericArgument,
	PathArguments, Type, Variant,
};

use convert_case::Casing;

macro_rules! derive_error {
	($string: tt) => {
		Error::new(Span::call_site(), $string)
			.to_compile_error()
			.into()
	};
}

fn parse_args(variant: &Variant, map: &mut HashMap<String, TokenStream>) {
	for attr in &variant.attrs {
		if attr.path.is_ident("splat") {
			let meta: syn::Meta = attr.parse_meta().unwrap();

			if let syn::Meta::List(list) = meta {
				list.nested.into_pairs().for_each(|pair| {
					if let syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) = pair.value() {
						let lit = &nv.lit;
						map.insert(nv.path.get_ident().unwrap().to_string(), quote! { #lit });
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

type TokenStreams = (TokenStream, TokenStream, TokenStream, TokenStream);

fn parse_variant_field(
	variant: &Variant,
	iter: &mut syn::punctuated::Iter<syn::Field>,
	stream: &mut TokenStreams,
	xyz: XYZ,
	impls: &mut TokenStream,
	args: &mut HashMap<String, TokenStream>,
) {
	let variant_name = &variant.ident;
	let variant_name_lower = variant_name.to_string().to_case(convert_case::Case::Snake);
	if let Some(field) = iter.next() {
		if field.attrs.iter().any(|attr| attr.path.is_ident("skip")) {
			return;
		}

		if let Type::Path(ty) = &field.ty {
			if let Some(segment) = ty.path.segments.first() {
				let mut type_ = None;

				let is_option = segment.ident.eq("Option");
				let is_vec = segment.ident.eq("Vec");

				if let PathArguments::AngleBracketed(arguments) = &segment.arguments &&
					!arguments.args.is_empty() &&
					let Some(GenericArgument::Type(ty)) = arguments.args.first()
						{ if let Type::Path(ty) = ty { type_ = Some(ty);  } } else { type_ = Some(ty); }

				if let Some(ty) = type_ {
					let key = String::from(xyz.key());
					let name = xyz.name();

					stream.0.extend(quote_spanned! { ty.span()=>
						#[expand]
						#variant_name(#ty),
					});
					stream.1.extend(quote_spanned! { ty.span()=>
						Self::#variant_name(..) => #ty::all().into_iter().map(Into::into).collect(),
					});

					let s = if is_option {
						quote! { val.clone().map(Into::into) }
					} else if is_vec {
						quote! { val.first().clone().map(Into::into) }
					} else {
						quote! { Some(val.clone().into()) }
					};

					let fields = match xyz {
						XYZ::X => quote! {(val, ..)},
						XYZ::Y => quote! {(_, val, ..)},
						XYZ::Z => quote! {(_, _, val, ..)},
					};

					stream.2.extend(quote_spanned! { ty.span()=>
						Self::#variant_name #fields => #s,
					});

					let set = if is_option {
						quote! {
							match splat {
								Some(splat) => {
									if let #name::#variant_name(splat) = splat {
										*val = Some(splat)
									}
								},
								None => *val = None,
							}
						}
					} else {
						quote! {
							if let Some(#name::#variant_name (splat)) = splat {
								*val = splat
							}
						}
					};

					stream.3.extend(quote_spanned! { ty.span()=>
						Self::#variant_name #fields => #set,
					});

					let lower = ty
						.path
						.get_ident()
						.unwrap()
						.to_string()
						.to_case(convert_case::Case::Snake);
					args.insert(key, quote! { #lower });
					impls.extend(quote_spanned! { ty.span()=>
						impl From<#ty> for #name {
							fn from(val: #ty) -> Self {
								#name::#variant_name(val)
							}
						}
						impl NameKey for #ty {
							fn name_key(&self) -> String {
								format!("{}.{}", #variant_name_lower, self.name())
							}
						}
					});
				}
			}
		}
	}
}

enum XYZ {
	X,
	Y,
	Z,
}

impl XYZ {
	pub fn key(&self) -> &str {
		match self {
			XYZ::X => "xsplat",
			XYZ::Y => "ysplat",
			XYZ::Z => "zsplat",
		}
	}
	pub fn name(&self) -> TokenStream {
		match self {
			XYZ::X => quote! { XSplat },
			XYZ::Y => quote! { YSplat },
			XYZ::Z => quote! { ZSplat },
		}
	}
}

#[proc_macro_derive(SplatEnum, attributes(splat, skip))]
pub fn derive_splat_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = &input.ident;
	let data = &input.data;

	let mut xsplat = (
		TokenStream::new(),
		TokenStream::new(),
		TokenStream::new(),
		TokenStream::new(),
	);
	let mut ysplat = (
		TokenStream::new(),
		TokenStream::new(),
		TokenStream::new(),
		TokenStream::new(),
	);
	let mut zsplat = (
		TokenStream::new(),
		TokenStream::new(),
		TokenStream::new(),
		TokenStream::new(),
	);

	let mut impls = TokenStream::new();

	let mut variants_map = HashMap::new();

	if let Data::Enum(data_enum) = data {
		let mut args = HashMap::new();

		for variant in &data_enum.variants {
			let variant_name = &variant.ident;
			let _variant_name_lower = variant_name.to_string().to_case(convert_case::Case::Snake);

			args.clear();
			parse_args(variant, &mut args);

			let fields_in_variant = variant_fields(variant);

			if let Fields::Unnamed(fields) = &variant.fields {
				let mut iter = fields.unnamed.iter();

				parse_variant_field(
					variant,
					&mut iter,
					&mut xsplat,
					XYZ::X,
					&mut impls,
					&mut args,
				);
				parse_variant_field(
					variant,
					&mut iter,
					&mut ysplat,
					XYZ::Y,
					&mut impls,
					&mut args,
				);
				parse_variant_field(
					variant,
					&mut iter,
					&mut zsplat,
					XYZ::Z,
					&mut impls,
					&mut args,
				);
			}

			let mut gen_match_arm = |key: &str, b: bool| {
				if let Some(val) = args.get(key) {
					let v = if b {
						quote_spanned! { variant.span()=> Some(#val) }
					} else {
						quote_spanned! { variant.span()=> #val }
					};

					if !variants_map.contains_key(key) {
						variants_map.insert(key.to_string(), TokenStream::new());
					}
					variants_map
						.get_mut(key)
						.unwrap()
						.extend(quote_spanned! {variant.span()=>
							#name::#variant_name #fields_in_variant => #v,
						});
				}
			};

			gen_match_arm("virtue_anchor", false);
			gen_match_arm("vice_anchor", false);

			gen_match_arm("xsplat", true);
			gen_match_arm("ysplat", true);
			gen_match_arm("zsplat", true);

			gen_match_arm("ability", true);
			gen_match_arm("abilities_finite", false);

			gen_match_arm("st", true);
			gen_match_arm("alt_beats", true);

			gen_match_arm("fuel", true);
			gen_match_arm("integrity", false);
		}
	} else {
		return derive_error!("SplatEnum is only implemented for enums");
	}

	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let mut funcs = TokenStream::new();
	let mut funcss = TokenStream::new();

	let mut gen_func = |key: &str, name: &str, b: bool, default: Option<&str>| {
		if let Some(val) = variants_map.get(key) {
			let ident = syn::Ident::new(name, Span::call_site());
			let o = if b {
				quote! { Option<&str> }
			} else {
				quote! { &str }
			};

			let def_match = if b {
				quote! { None }
			} else if let Some(default) = default {
				quote! { #default }
			} else {
				quote! { "" }
			};

			funcs.extend(quote! {
				pub fn #ident(&self) -> #o {
					match self {
						#val
						_ => #def_match
					}
				}
			});
		}
	};

	gen_func("virtue_anchor", "virtue_anchor", false, Some("virtue"));
	gen_func("vice_anchor", "vice_anchor", false, Some("vice"));

	gen_func("xsplat", "xsplat_name", true, None);
	gen_func("ysplat", "ysplat_name", true, None);
	gen_func("zsplat", "zsplat_name", true, None);

	gen_func("ability", "ability_name", true, None);
	if let Some(val) = variants_map.get("abilities_finite") {
		funcss.extend(quote! {
			pub fn are_abilities_finite(&self) -> bool {
				match self {
					#val
					_ => true
				}
			}
		});
	}

	gen_func("st", "supernatural_tolerance", true, None);
	gen_func("alt_beats", "alternate_beats", true, None);

	gen_func("fuel", "fuel", true, None);
	gen_func("integrity", "integrity", false, Some("integrity"));

	let (xsplats, xsplats_all, xsplat, set_xsplat) = xsplat;
	let (ysplats, ysplats_all, ysplat, set_ysplat) = ysplat;
	let (zsplats, zsplats_all, zsplat, set_zsplat) = zsplat;

	let expanded = quote! {
		impl #impl_generics #name #ty_generics #where_clause {
			#funcs
			#funcss

			pub fn set_xsplat(&mut self, splat: Option<XSplat>) {
				match self {
					#set_xsplat
					_ => {}
				}
			}

			pub fn set_ysplat(&mut self, splat: Option<YSplat>) {
				match self {
					#set_ysplat
					_ => {}
				}
			}

			pub fn set_zsplat(&mut self, splat: Option<ZSplat>) {
				match self {
					#set_zsplat
					_ => {}
				}
			}

			pub fn xsplat(&self) -> Option<XSplat> {
				match self {
					#xsplat
					_ => None,
				}
			}
			pub fn ysplat(&self) -> Option<YSplat> {
				match self {
					#ysplat
					_ => None,
				}
			}
			pub fn zsplat(&self) -> Option<ZSplat> {
				match self {
					#zsplat
					_ => None,
				}
			}

			pub fn xsplats(&self) -> Vec<XSplat> {
				match self {
					#xsplats_all
					_ => Vec::new(),
				}
			}
			pub fn ysplats(&self) -> Vec<YSplat> {
				match self {
					#ysplats_all
					_ => Vec::new(),
				}
			}
			pub fn zsplats(&self) -> Vec<ZSplat> {
				match self {
					#zsplats_all
					_ => Vec::new(),
				}
			}
		}

		#[derive(Debug, Clone, PartialEq, Eq, VariantName, NameKey)]
		pub enum XSplat {
			#xsplats
		}
		#[derive(Debug, Clone, PartialEq, Eq, VariantName, NameKey)]
		pub enum YSplat {
			#ysplats
		}
		#[derive(Debug, Clone, PartialEq, Eq, VariantName, NameKey)]
		pub enum ZSplat {
			#zsplats
		}

		#impls
	};

	proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(VariantName, attributes(expand))]
pub fn derive_variant_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = &input.ident;
	let data = &input.data;

	let mut name_fun_variants = TokenStream::new();

	if let Data::Enum(data) = data {
		for variant in &data.variants {
			let variant_name = &variant.ident;

			let mut flag = false;

			for attr in &variant.attrs {
				if attr.path.is_ident("expand") {
					flag = true;
					break;
				}
			}

			if variant_name.eq("_Custom") {
				if let Fields::Unnamed(fields) = &variant.fields {
					if let Some(field) = fields.unnamed.first() {
						name_fun_variants.extend(quote_spanned! {field.span()=>
							#name::#variant_name (name, ..) => name,
						});
					}
				}
			} else {
				let mut match_arm = TokenStream::new();

				let fields_in_variant = if flag && variant.fields.len() == 1 {
					quote_spanned! {variant.span()=> (val) }
				} else {
					variant_fields(variant)
				};

				let variant_name_lower =
					variant_name.to_string().to_case(convert_case::Case::Snake);

				if flag {
					if let Fields::Unnamed(fields) = &variant.fields && let Some(field) = fields.unnamed.first() && let Type::Path(ty) = &field.ty {
						if ty.path.segments.first().unwrap().ident.eq("Option") {
							match_arm.extend(quote_spanned! {ty.span()=>
								match val {
									Some(val) => VariantName::name(val),
									None => #variant_name_lower
								}
							});
						} else {
							match_arm.extend(quote_spanned! {ty.span()=>
								VariantName::name(val)
							});
						}
					}
				} else {
					match_arm.extend(quote_spanned! {variant.span()=> #variant_name_lower });
				}

				name_fun_variants.extend(quote_spanned! {variant.span()=>
					#name::#variant_name #fields_in_variant => #match_arm,
				});
			}
		}
	} else {
		return derive_error!("VariantName is only implemented for enums");
	}

	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let expanded = quote! {
		impl #impl_generics cofd_util::VariantName for #name #ty_generics #where_clause {
			fn name(&self) -> &str {
				match self {
					#name_fun_variants
				}
			}
		}
	};

	proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(AllVariants, attributes(expand))]
pub fn derive_all_variants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = &input.ident;
	let data = &input.data;

	let mut all_variants = TokenStream::new();
	let mut sub_enums = TokenStream::new();

	let mut num: usize = 0;

	let mut flag = false;

	if let Data::Enum(data) = data {
		for variant in &data.variants {
			let variant_name = &variant.ident;

			if variant_name.eq("_Custom") {
				continue;
			}

			let mut flag1 = false;

			for attr in &variant.attrs {
				if attr.path.is_ident("expand") {
					flag1 = true;
					flag = true;
					break;
				}
			}

			let fields = match &variant.fields {
				Fields::Unnamed(fields) => {
					let mut field_tokens = TokenStream::new();

					if !flag1 {
						for field in &fields.unnamed {
							field_tokens
								.extend(quote_spanned! { field.span()=> Default::default(), });
						}
					} else if let Some(field) = fields.unnamed.first() {
						if let syn::Type::Path(ty) = &field.ty {
							if let Some(segment) = ty.path.segments.first() {
								if let PathArguments::AngleBracketed(arguments) = &segment.arguments
								{
									field_tokens.extend(
										quote_spanned! { field.span()=> Default::default(), },
									);
									if let Some(GenericArgument::Type(ty2)) = arguments.args.first()
									{
										sub_enums.extend(
											quote_spanned! { field.span()=> vec.extend(<#ty2 as AllVariants>::all().map(Into::into)); },
										);
									}
								} else {
									sub_enums.extend(
										quote_spanned! { field.span()=> vec.extend(<#ty as AllVariants>::all().map(Into::into)); },
									);

									continue;
								}
							}
						}
					}
					quote_spanned! {variant.span()=> (#field_tokens) }
				}
				Fields::Unit => quote_spanned! { variant.span()=> },
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

	let mut all_vec = TokenStream::new();

	if flag {
		all_vec.extend(quote! {
			impl #impl_generics #name #ty_generics #where_clause {
				pub fn all() -> Vec<#name> {
					let mut vec = std::vec::Vec::from(<#name as cofd_util::AllVariants>::all());
					#sub_enums
					vec
				}
			}
		});
	}

	let expanded = quote! {
		impl #impl_generics cofd_util::AllVariants for #name #ty_generics #where_clause {
			type T = #name;
			const N: usize = #num;
			fn all() -> [Self::T; Self::N] {
				[
					#all_variants
				]
			}
		}
		#all_vec
	};

	proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(NameKey, attributes(expand))]
pub fn derive_name_key(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = &input.ident;
	let data = &input.data;

	let mut match_cases = TokenStream::new();

	if let Data::Enum(data) = data {
		for variant in &data.variants {
			let variant_name = &variant.ident;

			match_cases.extend(quote_spanned! {variant.span()=>
				#name::#variant_name(val) => val.name_key(),
			});
		}
	}

	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let expanded = quote! {
		impl #impl_generics NameKey for #name #ty_generics #where_clause {
			fn name_key(&self) -> String {
				match self {
					#match_cases
				}
			}
		}
	};

	proc_macro::TokenStream::from(expanded)
}
