use std::sync::Arc;

use proc_macro2::Span;
use syn::Ident;

#[derive(Default, Clone)]
#[non_exhaustive]
pub struct Cfg {}
pub mod traits;
pub mod remapped;