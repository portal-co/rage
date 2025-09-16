use crate::*;
#[derive(Default)]
pub(crate) struct ImportManifest {
    pub(crate) map: BTreeMap<Atom, BTreeMap<Atom, Id>>,
    pub(crate) globals: BTreeMap<Vec<Atom>, Id>,
}
impl ImportManifest {
    pub(crate) fn get(&mut self, module: Atom, name: Atom) -> Id {
        return self
            .map
            .entry(module)
            .or_default()
            .entry(name.clone())
            .or_insert_with(|| (name.clone(), SyntaxContext::empty().apply_mark(Mark::new())))
            .clone();
    }
    pub(crate) fn global(&mut self, x: &[Atom]) -> Id {
        return self
            .globals
            .entry(x.to_owned())
            .or_insert_with(|| {
                (
                    Atom::new(x.iter().join("$")),
                    SyntaxContext::empty().apply_mark(Mark::new()),
                )
            })
            .clone();
    }
    pub(crate) fn render(&self, span: swc_common::Span) -> impl Iterator<Item = ModuleItem> {
        return self
            .map
            .iter()
            .map(move |(a, b)| {
                ModuleItem::ModuleDecl(swc_ecma_ast::ModuleDecl::Import(ImportDecl {
                    span,
                    specifiers: b
                        .iter()
                        .map(|(c, d)| {
                            swc_ecma_ast::ImportSpecifier::Named(ImportNamedSpecifier {
                                span,
                                local: swc_ecma_ast::Ident {
                                    span,
                                    ctxt: d.1,
                                    sym: d.0.clone(),
                                    optional: false,
                                },
                                imported: Some(swc_ecma_ast::ModuleExportName::Str(
                                    swc_ecma_ast::Str {
                                        span,
                                        value: c.clone(),
                                        raw: None,
                                    },
                                )),
                                is_type_only: false,
                            })
                        })
                        .collect(),
                    type_only: false,
                    with: None,
                    phase: Default::default(),
                    src: Box::new(swc_ecma_ast::Str {
                        span,
                        raw: None,
                        value: a.clone(),
                    }),
                }))
            })
            .chain(self.globals.iter().map(move |(a, b)| {
                ModuleItem::Stmt(swc_ecma_ast::Stmt::Decl(swc_ecma_ast::Decl::Var(Box::new(
                    VarDecl {
                        span,
                        ctxt: b.1,
                        kind: swc_ecma_ast::VarDeclKind::Const,
                        declare: false,
                        decls: vec![VarDeclarator {
                            span,
                            name: swc_ecma_ast::Pat::Ident(b.clone().into()),
                            init: Some(a.iter().fold(
                                Box::new(swc_ecma_ast::Expr::Ident(swc_ecma_ast::Ident {
                                    span,
                                    ctxt: Default::default(),
                                    sym: Atom::new("globalThis"),
                                    optional: false,
                                })),
                                |e, i| {
                                    Box::new(swc_ecma_ast::Expr::Member(swc_ecma_ast::MemberExpr {
                                        span,
                                        obj: e,
                                        prop: swc_ecma_ast::MemberProp::Ident(
                                            swc_ecma_ast::IdentName {
                                                span,
                                                sym: i.clone(),
                                            },
                                        ),
                                    }))
                                },
                            )),
                            definite: false,
                        }],
                    },
                ))))
            }));
    }
}
