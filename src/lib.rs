use swc_common::{BytePos, DUMMY_SP};
use swc_ecma_ast::*;
use swc_core::ecma::atoms::JsWord;
use swc_ecma_visit::{VisitMut, VisitMutWith};
use swc_core::{
    plugin::{
        metadata::TransformPluginMetadataContextKind, plugin_transform,
        proxies::TransformPluginProgramMetadata,
    },
};
use serde::Deserialize;

fn default_as_false() -> bool {
    false
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub modules: Vec<String>,

    #[serde(default = "default_as_false")]
    pub treat_json_as_cjs: bool,
}

pub struct TransformVisitor {
    modules: Vec<String>,
    treat_json_as_cjs: bool,
}

pub fn transform_cjs_imports(config: Config) -> impl VisitMut
{
    TransformVisitor { 
        modules: config.modules, 
        treat_json_as_cjs: config.treat_json_as_cjs,
    }
}

#[plugin_transform]
fn transform_cjs_imports_plugin(
    mut program: Program,
    data: TransformPluginProgramMetadata,
) -> Program {
    let config = serde_json::from_str::<Config>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for transform-cjs-imports"),
    )
    .expect("invalid config for transform-cjs-imports");

    program.visit_mut_with(&mut transform_cjs_imports(config));

    program
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
                    let is_json = self.treat_json_as_cjs && src.ends_with(".json");

                    if self.modules.contains(&src.to_string()) || is_json {
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

                            let mut import_assertion = None;

                            if is_json {
                                import_assertion = Some(Box::new(ObjectLit {
                                    span: DUMMY_SP,
                                    props: vec![PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                                        key: PropName::Ident(Ident::new("type".into(), DUMMY_SP)),
                                        value: Box::new(Expr::Lit(Lit::Str(Str {
                                            span: DUMMY_SP,
                                            value: "json".into(),
                                            raw: None,
                                        }))),
                                    })))],
                                }));
                            }
                            
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
                                    asserts: import_assertion,
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

        if !extra_decls.is_empty() {
            new_body.splice(import_end_index+1..import_end_index+1, extra_decls);
            n.body = new_body;
        }
    }
}
