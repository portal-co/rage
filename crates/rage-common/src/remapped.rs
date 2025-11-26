use crate::{
    traits::{Mapper, Remap, RemapOutput},
    *,
};
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Id<Ctx>(pub Ident, pub Ctx);
impl<'a, Ctx: 'a> RemapOutput<'a, Ctx> for Ident {
    type Output = Id<Ctx>;
}
impl<Ctx: Clone + 'static> Remap<Ctx> for Ident {
    fn remap<'a>(
        &'a self,
        m: &'a (dyn Mapper<Ctx = Ctx> + 'a),
    ) -> <Self as RemapOutput<'a, Ctx>>::Output {
        Id(self.clone(), m.ctx_of(self))
    }
}
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Expr<Ctx> {
    Return(Box<Expr<Ctx>>),
    Int(u128),
    Ident(Id<Ctx>),
}
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Pat<Ctx> {
    Ident(Id<Ctx>),
}
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Item<Ctx> {
    Fn(FnItem<Ctx>),
}
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct FnItem<Ctx> {
    pub name: Id<Ctx>,
    pub params: Vec<Pat<Ctx>>,
    pub body: Box<Expr<Ctx>>,
}
