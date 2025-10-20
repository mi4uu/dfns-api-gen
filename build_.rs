use openapi_struct_gen::generate;

fn main() {
    generate(
        format!(
            "{}/{}",
            std::env::var("CARGO_MANIFEST_DIR").unwrap(),
            "openapi30.yml"
        ),
        format!("{}/{}", std::env::var("OUT_DIR").unwrap(), "oapi.rs"),
        Some(&["Clone", "Serialize", "Deserialize"]),
        Some(&[("serde", "Serialize"), ("serde", "Deserialize")]),
        Some(&[(r#"#[skip_serializing_none]"#, None)]),
        Some(&[(r#"#[serde(rename_all = "camelCase")]"#, Some(&["Struct"]))]),
    )
    .unwrap();
}
