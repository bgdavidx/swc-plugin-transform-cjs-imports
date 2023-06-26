use swc_plugin_transform_cjs_imports::transform_cjs_imports;
use std::path::PathBuf;
use swc_core::ecma::{transforms::testing::test_fixture, visit::as_folder};

#[testing::fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");

    test_fixture(
        Default::default(),
        &|_| as_folder(
            transform_cjs_imports(vec![
                  "lib".to_string(),
                  "lib2".to_string(),
                  "lib3".to_string()
              ])
        ),
        &input,
        &output,
        Default::default(),
    );
}
