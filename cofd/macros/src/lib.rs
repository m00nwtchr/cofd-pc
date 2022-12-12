#![feature(let_chains)]
use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
	parse_macro_input, spanned::Spanned, AngleBracketedGenericArguments, Data, DeriveInput, Error,
	Fields, GenericArgument, PathArguments, PathSegment, Type, Variant,
};

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

fn parse_variant_field(
	variant: &Variant,
	iter: &mut syn::punctuated::Iter<syn::Field>,
	stream: &mut XYZ,
	impls: &mut TokenStream,
	args: &mut HashMap<String, String>,
) {
	let variant_name = &variant.ident;
	if let Some(field) = iter.next() {
		if let Type::Path(ty) = &field.ty {
			if let Some(segment) = ty.path.segments.first() {
				let mut type_ = None;

				if let PathArguments::AngleBracketed(arguments) = &segment.arguments &&
					!arguments.args.is_empty() &&
					let Some(GenericArgument::Type(ty)) = arguments.args.first()
						{ if let Type::Path(ty) = ty { type_ = Some(ty);  } } else { type_ = Some(ty); }

				if let Some(ty) = type_ {
					let key = String::from(stream.key());
					let name = stream.name();

					stream.unwrap_0().extend(quote_spanned! { ty.span()=>
						#[expand]
						#variant_name(#ty),
					});
					stream.unwrap_1().extend(quote_spanned! { ty.span()=>
						SplatType::#variant_name => #ty::all().into_iter().map(Into::into).collect(),
					});
					args.insert(
						key,
						ty.path
							.get_ident()
							.unwrap()
							.to_string()
							.to_case(convert_case::Case::Snake),
					);
					impls.extend(quote_spanned! { ty.span()=>
						impl From<#ty> for #name {
							fn from(val: #ty) -> Self {
								#name::#variant_name(val)
							}
						}
					});
				}
			}
		}
	}
}

enum XYZ<'a> {
	X(&'a mut TokenStream, &'a mut TokenStream),
	Y(&'a mut TokenStream, &'a mut TokenStream),
	Z(&'a mut TokenStream, &'a mut TokenStream),
}

impl<'a> XYZ<'a> {
	pub fn key(&self) -> &str {
		match self {
			XYZ::X(..) => "xsplat",
			XYZ::Y(..) => "ysplat",
			XYZ::Z(..) => "zsplat",
		}
	}
	pub fn name(&self) -> TokenStream {
		match self {
			XYZ::X(..) => quote! { XSplat },
			XYZ::Y(..) => quote! { YSplat },
			XYZ::Z(..) => quote! { ZSplat },
		}
	}
	pub fn unwrap_0(&mut self) -> &mut TokenStream {
		match self {
			XYZ::X(s, _) | XYZ::Y(s, _) | XYZ::Z(s, _) => s,
		}
	}
	pub fn unwrap_1(&mut self) -> &mut TokenStream {
		match self {
			XYZ::X(_, s) | XYZ::Y(_, s) | XYZ::Z(_, s) => s,
		}
	}
}

#[proc_macro_derive(SplatEnum, attributes(splat))]
pub fn derive_splat_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let name = &input.ident;
	let data = &input.data;

	let mut splat_idents = TokenStream::new();
	let mut xsplats = TokenStream::new();
	let mut ysplats = TokenStream::new();
	let mut zsplats = TokenStream::new();

	let mut xsplats_all = TokenStream::new();
	let mut ysplats_all = TokenStream::new();
	let mut zsplats_all = TokenStream::new();

	let mut impls = TokenStream::new();

	let mut name_keys = TokenStream::new();

	let mut variants_map = HashMap::new();

	if let Data::Enum(data_enum) = data {
		let mut args = HashMap::new();

		for variant in &data_enum.variants {
			let variant_name = &variant.ident;
			let variant_name_lower = variant_name.to_string().to_case(convert_case::Case::Snake);

			args.clear();
			parse_args(variant, &mut args);

			let fields_in_variant = variant_fields(variant);

			splat_idents.extend(quote_spanned! {variant.span()=>
				#variant_name,
			});

			if let Fields::Unnamed(fields) = &variant.fields {
				let mut iter = fields.unnamed.iter();

				parse_variant_field(
					variant,
					&mut iter,
					&mut XYZ::X(&mut xsplats, &mut xsplats_all),
					&mut impls,
					&mut args,
				);
				parse_variant_field(
					variant,
					&mut iter,
					&mut XYZ::Y(&mut ysplats, &mut ysplats_all),
					&mut impls,
					&mut args,
				);
				parse_variant_field(
					variant,
					&mut iter,
					&mut XYZ::Z(&mut zsplats, &mut zsplats_all),
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

			gen_match_arm("st", true);
			gen_match_arm("alt_beats", true);

			gen_match_arm("fuel", true);
			gen_match_arm("integrity", false);

			if let Fields::Unnamed(_) = &variant.fields {
				name_keys.extend(quote! {
					Self::#variant_name(val) => format!("{}.{}", val.name(), #variant_name_lower),
				});
			}
		}
	} else {
		return derive_error!("SplatEnum is only implemented for enums");
	}

	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	let mut funcs = TokenStream::new();

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

	gen_func("st", "supernatural_tolerance", true, None);
	gen_func("alt_beats", "alternate_beats", true, None);

	gen_func("fuel", "fuel", true, None);
	gen_func("integrity", "integrity", false, Some("integrity"));

	let expanded = quote! {
		#[derive(Debug, Clone, Copy, VariantName, AllVariants)]
		pub enum SplatType {
			#splat_idents
		}

		impl #impl_generics #name #ty_generics #where_clause {
			#funcs
		}

		#[derive(Debug, Clone, PartialEq, Eq, VariantName)]
		pub enum XSplat {
			#xsplats
		}
		#[derive(Debug, Clone, PartialEq, Eq, VariantName)]
		pub enum YSplat {
			#ysplats
		}
		#[derive(Debug, Clone, PartialEq, Eq, VariantName)]
		pub enum ZSplat {
			#zsplats
		}
		impl XSplat {
			pub fn all(st: SplatType) -> Vec<Self> {
				match st {
					#xsplats_all
					_ => Vec::new(),
				}
			}
		}
		impl YSplat {
			pub fn all(st: SplatType) -> Vec<Self> {
				match st {
					#ysplats_all
					_ => Vec::new(),
				}
			}
		}
		impl ZSplat {
			pub fn all(st: SplatType) -> Vec<Self> {
				match st {
					#zsplats_all
					_ => Vec::new(),
				}
			}
		}
		#impls

		impl NameKey for XSplat {
			fn name_key(&self) -> String {
				match self {
					#name_keys
				}
			}
		}
		impl NameKey for YSplat {
			fn name_key(&self) -> String {
				match self {
					#name_keys
				}
			}
		}
		impl NameKey for ZSplat {
			fn name_key(&self) -> String {
				match self {
					#name_keys
				}
			}
		}
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
					let mut vec = std::vec::Vec::from(<#name as cofd_traits::AllVariants>::all());
					#sub_enums
					vec
				}
			}
		});
	}

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
		#all_vec
	};

	proc_macro::TokenStream::from(expanded)
}
