#![feature(generic_const_exprs)]

extern crate cofd_macros;

pub use cofd_macros::*;

// mod traits {

pub trait VariantName {
	fn name(&self) -> &str;
}

pub trait AllVariants {
	type T;
	const N: usize;
	fn all() -> [Self::T; Self::N];
}

// pub use traits::*;
