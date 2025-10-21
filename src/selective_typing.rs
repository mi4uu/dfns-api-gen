use openapi_type_picker::*;
fn main() {
    let s = oas3::from_yaml(include_str!("../openapi.yml")).unwrap();
    let s = s.components.unwrap();
    //     let s = write_openapi_types(
    //         // OpenApi::from_str(include_str!("../openapi.yml")),
    //         OpenApi::from_file("openapi.json"),
    //         FilterConfig::from_str(
    //             r#"""{
    //           "include": {
    //             "Swap": ["*"]
    //           },
    //           auto_include_dependencies: true
    //         }
    // """#,
    //         ),
    //         "crazygen.rs", // path to output file
    //     )
    //     .unwrap();
    println!("{s:?}");
}
