use crate::*;
#[derive(Default)]
pub(crate) struct RustLoader {
    pub(crate) map: OnceMap<[u8; 32], Box<swc_common::sync::Lrc<SourceFile>>>,
    pub(crate) sm: swc_common::sync::Lrc<SourceMap>,
}
impl RustLoader {
    pub(crate) fn load(&self, a: &Path) -> anyhow::Result<swc_common::sync::Lrc<SourceFile>> {
        return Ok(self
            .map
            .try_insert(
                sha3::Sha3_256::digest(a.as_os_str().as_encoded_bytes()).into(),
                |_| self.sm.load_file(a).map(Box::new),
            )?
            .clone());
    }
    pub(crate) fn map_span(&self, s: &proc_macro2::Span) -> anyhow::Result<swc_common::Span> {
        let b = s.byte_range();
        let f = s.local_file().context("in getting the file")?;
        let f = self.load(&f).context("in loading the file")?;
        let lo = f.start_pos.0 + (b.start as u32);
        let hi = f.start_pos.0 + (b.end as u32);
        Ok(swc_common::Span {
            lo: swc_common::BytePos(lo),
            hi: swc_common::BytePos(hi),
        })
    }
    pub(crate) fn syn<T: syn::parse::Parse>(&self, a: &Path) -> anyhow::Result<T> {
        let l = self.load(a)?;
        let s: T = syn::parse_str(&l.src)?;
        Ok(s)
    }
}
