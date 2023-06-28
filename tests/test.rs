use swc_plugin_transform_cjs_imports::{transform_cjs_imports, Config};
use std::path::PathBuf;
use swc_core::ecma::{transforms::testing::test_fixture, visit::as_folder};
use swc_ecma_parser::{Syntax, EsConfig};

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    let config = EsConfig {
      import_assertions: true,
      ..EsConfig::default()
    };
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        Syntax::Es(config),
        &|_| as_folder(
          transform_cjs_imports(Config {
            modules: vec![
                "lib".to_string(),
                "lib2".to_string(),
                "lib3".to_string()
            ],
            treat_json_as_cjs: true,
            add_json_import_assertions: true,
          })
        ),
        &input,
        &output,
        Default::default(),
    );
}
