use std::{path::PathBuf, rc::Rc};

use swc_common::{FileLoader, FileName, source_map::RealFileLoader, sync::Lrc};

use crate::*;

pub(crate) struct RustLoader {
    pub(crate) map: OnceMap<[u8; 32], Box<swc_common::sync::Lrc<SourceFile>>>,
    pub(crate) files: OnceMap<[u8; 32], Box<PathBuf>>,
    pub(crate) sm: swc_common::sync::Lrc<SourceMap>,
    pub(crate) loader: swc_common::sync::Lrc<dyn FileLoader>,
}
impl Default for RustLoader {
    fn default() -> Self {
        Self {
            map: Default::default(),
            sm: Default::default(),
            files: Default::default(),
            loader: Lrc::new(RealFileLoader {}),
        }
    }
}
impl RustLoader {
    pub(crate) fn load(&self, a: &Path) -> anyhow::Result<swc_common::sync::Lrc<SourceFile>> {
        return Ok(self
            .map
            .try_insert(
                sha3::Sha3_256::digest(a.as_os_str().as_encoded_bytes()).into(),
                |_| {
                    self.loader
                        .read_file(a)
                        .map(|b| {
                            self.sm
                                .new_source_file(Lrc::new(FileName::Real(a.to_path_buf())), b)
                        })
                        .map(Box::new)
                },
            )?
            .clone());
    }
    pub(crate) fn map_span(&self, s: &proc_macro2::Span) -> anyhow::Result<swc_common::Span> {
        let b = s.byte_range();
        let f = self
            .files
            .get::<[u8; 32]>(&sha3::Sha3_256::digest(s.file()).into())
            .context("in getting the mapped file")?;
        let f = self.load(&f).context("in loading the file")?;
        let lo = f.start_pos.0 + (b.start as u32);
        let hi = f.start_pos.0 + (b.end as u32);
        Ok(swc_common::Span {
            lo: swc_common::BytePos(lo),
            hi: swc_common::BytePos(hi),
        })
    }
    pub(crate) fn syn<T: syn::parse::Parse + syn::spanned::Spanned>(
        &self,
        a: &Path,
    ) -> anyhow::Result<T> {
        let l = self.load(a)?;
        let s: T = syn::parse_str(&l.src)?;
        self.files.insert(
            sha3::Sha3_256::digest(syn::spanned::Spanned::span(&s).file()).into(),
            |_| Box::new(a.to_owned()),
        );
        Ok(s)
    }
}
