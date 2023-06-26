use swc_common::{BytePos, DUMMY_SP};
use swc_ecma_ast::*;
use swc_core::ecma::atoms::JsWord;
use swc_ecma_visit::{VisitMut, VisitMutWith};

pub struct TransformVisitor {
    modules: Vec<String>,
}

pub fn transform_cjs_imports(modules: Vec<String>) -> impl VisitMut
{
    TransformVisitor { modules }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_module(&mut self, n: &mut Module) {
        let mut new_body = vec![];
        let mut extra_decls = vec![];
        let mut counter = 0;
        let mut import_end_index = 0;

        for (i, item) in n.body.iter().enumerate() {
            match item {
                ModuleItem::ModuleDecl(ModuleDecl::Import(imp)) => {
                    import_end_index = i;
                    let src = imp.src.value.as_ref();

                    if self.modules.contains(&src.to_string()) {
                        let mut default_ident = None;
                        let mut named_specifiers = vec![];

                        for specifier in &imp.specifiers {
                            match specifier {
                                ImportSpecifier::Named(named_spec) => {
                                    named_specifiers.push(named_spec.local.clone());
                                }
                                ImportSpecifier::Default(default_spec) => {
                                    default_ident = Some(default_spec.local.clone());
                                }
                                _ => {}
                            }
                        }

                        if !named_specifiers.is_empty() {
                            
                            new_body.push(ModuleItem::ModuleDecl(ModuleDecl::Import(
                                ImportDecl {
                                    span: DUMMY_SP,
                                    specifiers: vec![ImportSpecifier::Default(
                                        ImportDefaultSpecifier {
                                            span: DUMMY_SP,
                                            local: Ident::new(JsWord::from(format!("cjsModule{}", counter)).into(), DUMMY_SP),
                                        },
                                    )],
                                    src: imp.src.clone(),
                                    type_only: false,
                                    asserts: None,
                                },
                            )));

                            let var_declarator = VarDeclarator {
                                span: DUMMY_SP,
                                name: Pat::Object(ObjectPat {
                                    span: DUMMY_SP,
                                    optional: false,
                                    props: named_specifiers
                                        .into_iter()
                                        .map(|ident| ObjectPatProp::Assign(AssignPatProp {
                                            span: DUMMY_SP,
                                            key: ident.clone(),
                                            value: None,
                                        }))
                                        .collect(),
                                    type_ann: None,
                                }),
                                init: Some(Box::new(Expr::Ident(Ident::new(JsWord::from(
                                    format!("cjsModule{}", counter),
                                ).into(),
                                DUMMY_SP)))),
                                definite: false,
                            };

                            let var_decl = VarDecl {
                                span: DUMMY_SP,
                                kind: VarDeclKind::Const,
                                declare: false,
                                decls: vec![var_declarator],
                            };

                            if let Some(default_ident) = default_ident {
                                let default_var_decl = VarDecl {
                                    span: DUMMY_SP,
                                    kind: VarDeclKind::Const,
                                    declare: false,
                                    decls: vec![VarDeclarator {
                                        span: DUMMY_SP,
                                        name: Pat::Ident(BindingIdent {
                                            id: default_ident.clone(),
                                            type_ann: None,
                                        }),
                                        init: Some(Box::new(Expr::Ident(Ident::new(
                                            JsWord::from(format!("cjsModule{}", counter)).into(),
                                            DUMMY_SP,
                                        )))),
                                        definite: false,
                                    }],
                                };

                                extra_decls.push(ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(default_var_decl)))));
                            }

                            extra_decls.push(ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(var_decl)))));
                            counter += 1;
                            continue;
                        }
                    }
                }
                _ => {}
            }
            new_body.push(item.clone());
        }
        new_body.splice(import_end_index+1..import_end_index+1, extra_decls);
        n.body = new_body;
    }
}
