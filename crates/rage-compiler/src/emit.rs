use std::mem::transmute;

use rage_common::traits::{Emit, EmitCtx, StmtEmit};
use swc_ecma_ast::ModuleItem;

pub trait EmitExt<T>: Emit<T, ModuleItem> {}
impl<T, X: Emit<T, ModuleItem> + ?Sized> EmitExt<T> for X {}
pub trait StmtEmitExt: StmtEmit<ModuleItem> {}
impl<T: StmtEmit<ModuleItem> + ?Sized> StmtEmitExt for T {}
pub trait EmitCtxExt: EmitCtx {
    fn backend_impl(&self) -> Option<&Backend> {
        self.backend().downcast_ref()
    }
}
impl<T: EmitCtx + ?Sized> EmitCtxExt for T {}
pub struct Backend {}
