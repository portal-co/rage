use std::{any::Any, pin::Pin};

use async_trait::async_trait;

use crate::*;
pub trait Mapper {
    type Ctx: Clone;
    fn ctx_of(&self, id: &Ident) -> Self::Ctx;
}
pub trait RemapOutput<'a, Ctx: 'a> {
    type Output;
}
pub trait Remap<Ctx>: for<'a> RemapOutput<'a, Ctx> {
    fn remap<'a>(
        &'a self,
        m: &'a (dyn Mapper<Ctx = Ctx> + 'a),
    ) -> <Self as RemapOutput<'a, Ctx>>::Output;
}
pub trait EmitCtx {
    fn backend(&self) -> &(dyn Any);
}
// #[async_trait]
pub trait Emit<T, Stmt> {
    fn emit(&self, cfg: &Cfg, ctx: &(dyn EmitCtx + '_)) -> EmitRes<'_, T, Stmt>;
}
pub trait StmtEmit<Stmt>: Emit<Stmt, Stmt> {
    fn collapse(&self, cfg: &Cfg, ctx: &(dyn EmitCtx + '_)) -> Vec<Stmt> {
        self.emit(cfg, ctx).collapse(cfg, ctx)
    }
}
impl<Stmt, T: Emit<Stmt, Stmt> + ?Sized> StmtEmit<Stmt> for T {}
pub struct EmitRes<'a, T, Stmt> {
    pub value: T,
    pub statements: Vec<Arc<dyn Emit<Stmt, Stmt> + 'a>>,
}
impl<'a, T, Stmt> EmitRes<'a, T, Stmt> {
    pub fn with_value<U>(self, go: impl FnOnce(T) -> EmitRes<'a, U, Stmt>) -> EmitRes<'a, U, Stmt> {
        let EmitRes {
            value,
            mut statements,
        } = self;
        let EmitRes {
            value,
            statements: s2,
        } = go(value);
        statements.extend(s2);
        EmitRes { value, statements }
    }
    pub fn with_iter<U, I: IntoIterator<Item = EmitRes<'a, U, Stmt>>>(
        self,
        go: impl FnOnce(T) -> I,
    ) -> EmitRes<'a, Vec<U>, Stmt> {
        let EmitRes {
            value,
            mut statements,
        } = self;
        EmitRes {
            value: go(value)
                .into_iter()
                .map(
                    |EmitRes {
                         value,
                         statements: s2,
                     }| {
                        statements.extend(s2);
                        return value;
                    },
                )
                .collect(),
            statements,
        }
    }
}
impl<'a, Stmt> EmitRes<'a, Stmt, Stmt> {
    pub fn collapse(self, cfg: &Cfg, ctx: &(dyn EmitCtx + '_)) -> Vec<Stmt> {
        return [self.value]
            .into_iter()
            .chain(self.statements.iter().flat_map(|a| a.collapse(cfg, ctx)))
            .collect();
    }
}
pub trait RemapEmit<T, Stmt, Ctx> {
    fn remap<'a>(&'a self, m: &'a (dyn Mapper<Ctx = Ctx> + 'a)) -> Arc<dyn Emit<T, Stmt> + 'a>;
}
impl<T, Stmt, Ctx, U: Remap<Ctx> + for<'a> RemapOutput<'a, Ctx, Output: Emit<T, Stmt>>>
    RemapEmit<T, Stmt, Ctx> for U
{
    fn remap<'a>(&'a self, m: &'a (dyn Mapper<Ctx = Ctx> + 'a)) -> Arc<dyn Emit<T, Stmt> + 'a> {
        Arc::new(Remap::remap(self, m))
    }
}
