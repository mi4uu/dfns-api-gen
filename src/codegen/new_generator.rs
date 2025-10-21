// New generator with better naming and nested modules
use crate::codegen::schema_generator::SchemaGenerator;
use crate::codegen::utils::{to_pascal_case, to_snake_case};
use oas3::spec::{ObjectOrReference, ObjectSchema, Spec};
use std::collections::{BTreeMap, HashMap, HashSet};

pub struct NewGenerator {
    spec: Spec,
    generated_types: HashSet<String>,
}

#[derive(Debug)]
struct NestedModule {
    name: String,
    types: Vec<(String, ObjectSchema)>,
    submodules: BTreeMap<String, NestedModule>,
}

impl NestedModule {
    fn new(name: String) -> Self {
        Self {
            name,
            types: Vec::new(),
            submodules: BTreeMap::new(),
        }
    }

    fn navigate_or_create(&mut self, path: &[String]) -> &mut Self {
        if path.is_empty() {
            return self;
        }

        let first = &path[0];
        let rest = &path[1..];

        self.submodules
            .entry(first.clone())
            .or_insert_with(|| NestedModule::new(first.clone()))
            .navigate_or_create(rest)
    }

    fn generate_code(&self, generated_types: &mut HashSet<String>, indent: usize) -> String {
        let mut output = String::new();
        let indent_str = "    ".repeat(indent);

        // Generate types in this module
        for (type_name, schema) in &self.types {
            let full_name = format!("{}::{}", self.name, type_name);
            if !generated_types.contains(&full_name) {
                generated_types.insert(full_name);
                let code = SchemaGenerator::generate(type_name, schema);
                if !code.is_empty() {
                    for line in code.lines() {
                        if !line.is_empty() {
                            output.push_str(&indent_str);
                        }
                        output.push_str(line);
                        output.push('\n');
                    }
                    output.push('\n');
                }
            }
        }

        // Generate submodules
        for (_, submodule) in &self.submodules {
            output.push_str(&format!("{}pub mod {} {{\n", indent_str, submodule.name));
            output.push_str(&format!("{}    use super::*;\n\n", indent_str));
            output.push_str(&submodule.generate_code(generated_types, indent + 1));
            output.push_str(&format!("{}}}\n\n", indent_str));
        }

        output
    }
}

impl NewGenerator {
    pub fn new(spec: Spec) -> Self {
        Self {
            spec,
            generated_types: HashSet::new(),
        }
    }

    pub fn generate(&mut self) -> String {
        let mut output = String::new();

        // Header
        output.push_str("// Auto-generated from OpenAPI schema\n");
        output.push_str("// Do not edit manually\n\n");
        output.push_str("use serde::{Deserialize, Serialize};\n\n");

        // Generate component schemas (top-level)
        if let Some(components) = &self.spec.components {
            let mut component_schemas: Vec<_> = components.schemas.iter().collect();
            component_schemas.sort_by(|a, b| a.0.cmp(b.0));

            for (name, schema_ref) in component_schemas {
                if let ObjectOrReference::Object(obj_schema) = schema_ref {
                    if !self.generated_types.contains(name) {
                        self.generated_types.insert(name.to_string());
                        let code = SchemaGenerator::generate(name, obj_schema);
                        if !code.is_empty() {
                            output.push_str(&code);
                            output.push_str("\n\n");
                        }
                    }
                }
            }
        }

        // Build nested module structure from paths
        let mut root_modules = BTreeMap::new();

        if let Some(paths) = &self.spec.paths {
            for (path, path_item) in paths {
                let module_path = self.path_to_module_path(path);

                // Check each operation
                for (method, operation) in [
                    ("GET", &path_item.get),
                    ("POST", &path_item.post),
                    ("PUT", &path_item.put),
                    ("DELETE", &path_item.delete),
                    ("PATCH", &path_item.patch),
                ] {
                    if let Some(op) = operation {
                        let type_name = self.method_to_type_name(method, path);

                        // Extract request schema
                        if let Some(request_body) = &op.request_body {
                            if let ObjectOrReference::Object(body) = request_body {
                                if let Some(content) = body.content.get("application/json") {
                                    if let Some(ObjectOrReference::Object(schema)) = &content.schema {
                                        if !schema.properties.is_empty() || !schema.enum_values.is_empty() {
                                            let request_name = format!("{}Request", type_name);
                                            self.add_to_module_tree(&mut root_modules, &module_path, request_name, schema.clone());
                                        }
                                    }
                                }
                            }
                        }

                        // Extract response schemas
                        if let Some(responses) = &op.responses {
                            for (status_code, response) in responses {
                                if let ObjectOrReference::Object(resp) = response {
                                    if let Some(content) = resp.content.get("application/json") {
                                        if let Some(ObjectOrReference::Object(schema)) = &content.schema {
                                            if !schema.properties.is_empty() || !schema.enum_values.is_empty() {
                                                let status_suffix = if status_code == "200" {
                                                    String::new()
                                                } else {
                                                    status_code.replace("XX", "").replace("\"", "")
                                                };
                                                let response_name = format!("{}Response{}", type_name, status_suffix);
                                                self.add_to_module_tree(&mut root_modules, &module_path, response_name, schema.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Generate all root modules
        for (_, module) in root_modules {
            output.push_str(&format!("pub mod {} {{\n", module.name));
            output.push_str("    use super::*;\n\n");
            output.push_str(&module.generate_code(&mut self.generated_types, 1));
            output.push_str("}\n\n");
        }

        output
    }

    fn path_to_module_path(&self, path: &str) -> Vec<String> {
        path.split('/')
            .filter(|s| !s.is_empty())
            .map(|s| {
                let cleaned = s.replace('{', "").replace('}', "");
                to_snake_case(&cleaned)
            })
            .collect()
    }

    fn method_to_type_name(&self, method: &str, path: &str) -> String {
        let has_params = path.contains('{');

        match method.to_uppercase().as_str() {
            "GET" if !has_params => "List".to_string(),
            "GET" => "Get".to_string(),
            "POST" => "Create".to_string(),
            "PUT" => "Update".to_string(),
            "DELETE" => "Delete".to_string(),
            "PATCH" => "Patch".to_string(),
            _ => method.to_string(),
        }
    }

    fn add_to_module_tree(
        &self,
        root_modules: &mut BTreeMap<String, NestedModule>,
        path: &[String],
        type_name: String,
        schema: ObjectSchema,
    ) {
        if path.is_empty() {
            return;
        }

        let root_name = &path[0];
        let rest_path = &path[1..];

        let root = root_modules
            .entry(root_name.clone())
            .or_insert_with(|| NestedModule::new(root_name.clone()));

        let target = root.navigate_or_create(rest_path);
        target.types.push((type_name, schema));
    }
}
