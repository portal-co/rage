use std::{collections::BTreeMap, path::Path};
use anyhow::Context;
use itertools::Itertools;
use once_map::OnceMap;
use proc_macro2::TokenStream;
use sha3::Digest;
use swc_atoms::Atom;
use swc_common::{Mark, SourceFile, SourceMap, SyntaxContext};
use swc_ecma_ast::{Id, ImportDecl, ImportNamedSpecifier, ModuleItem, VarDecl, VarDeclarator};
pub(crate) mod source;
pub(crate) mod import;
#[derive(Default, Clone)]
#[non_exhaustive]
pub struct Cfg{
    pub hmr: bool,
    pub core: rage_common::Cfg,
}
