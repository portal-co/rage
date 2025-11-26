use crate::*;
pub trait Mapper {
    type Ctx: Clone;
    fn ctx_of(&self, id: &Ident) -> Self::Ctx;
}
pub trait Remap<Ctx> {
    type Output<'a>: 'a
    where
        Self: 'a;
    fn remap(&self, m: &(dyn Mapper<Ctx = Ctx> + '_)) -> Self::Output<'_>;
}
pub trait Emit<T> {
    fn emit(&self, cfg: &Cfg) -> T;
}
pub trait RemapEmit<T, Ctx> {
    fn remap(&self, m: &(dyn Mapper<Ctx = Ctx> + '_)) -> Arc<dyn Emit<T> + '_>;
}
impl<T, Ctx, U: for<'a> Remap<Ctx, Output<'a>: Emit<T>>> RemapEmit<T, Ctx> for U {
    fn remap(&self, m: &(dyn Mapper<Ctx = Ctx> + '_)) -> Arc<dyn Emit<T> + '_> {
        Arc::new(Remap::remap(self, m))
    }
}
