mod codegen;

use std::fs;

#[tokio::main]
async fn main() {
    println!("Fetching OpenAPI spec from Dfns...");
    let r = reqwest::get("https://docs.dfns.co/openapi.yaml")
        .await
        .expect("Failed to fetch OpenAPI spec");
    let yaml_content = r.text().await.expect("Failed to read response");
    let output = openapi31to30::convert(&yaml_content).unwrap();

    fs::write("openapi30.yml", output.clone()).expect("Failed to write 30v");

    let openapi = oas3::from_yaml(&output).expect("Failed to parse OpenAPI YAML");
    let openapi_json = oas3::to_json(&openapi).expect("Failed to convert to JSON");

    fs::write("openapi30.json", &openapi_json).expect("Failed to write openapi.json");
    println!("Saved OpenAPI spec to openapi.json");
    // tokio::task::block_in_place(|| {
    //     generate(
    //         format!(
    //             "{}/{}",
    //             std::env::var("CARGO_MANIFEST_DIR").unwrap(),
    //             "openapi30.yml"
    //         ),
    //         format!("{}/{}", std::env::var("OUT_DIR").unwrap(), "oapi.rs"),
    //         Some(&["Clone", "Serialize", "Deserialize"]),
    //         Some(&[("serde", "Serialize"), ("serde", "Deserialize")]),
    //         Some(&[(r#"#[skip_serializing_none]"#, None)]),
    //         Some(&[(r#"#[serde(rename_all = "camelCase")]"#, Some(&["Struct"]))]),
    //     )
    //     .unwrap()
    // });
}
