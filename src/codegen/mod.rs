mod endpoint_generator;
mod schema_generator;
mod type_mapper;
mod utils;

pub use endpoint_generator::EndpointGenerator;

use crate::codegen::utils::to_pascal_case;
use oas3::spec::{ObjectOrReference, ObjectSchema, Spec};
use std::collections::{BTreeMap, HashSet};

pub struct Generator {
    spec: Spec,
    generated_types: HashSet<String>,
    // Map: module_name -> [(type_name, schema)]
    module_schemas: BTreeMap<String, Vec<(String, ObjectSchema)>>,
    component_schemas: Vec<(String, ObjectSchema)>,
}

impl Generator {
    pub fn new(spec: Spec) -> Self {
        Self {
            spec,
            generated_types: HashSet::new(),
            module_schemas: BTreeMap::new(),
            component_schemas: Vec::new(),
        }
    }

    pub fn generate(&mut self) -> String {
        let mut output = String::new();

        // Add header and imports
        output.push_str("// Auto-generated from OpenAPI schema\n");
        output.push_str("// Do not edit manually\n\n");
        output.push_str("use serde::{Deserialize, Serialize};\n\n");

        // First, extract all inline schemas from paths
        self.extract_schemas_from_paths();

        // Collect component schemas
        if let Some(components) = &self.spec.components {
            for (name, schema_ref) in &components.schemas {
                if let ObjectOrReference::Object(obj_schema) = schema_ref {
                    self.component_schemas
                        .push((name.clone(), obj_schema.clone()));
                }
            }
            self.component_schemas.sort_by(|a, b| a.0.cmp(&b.0));
        }

        // Generate component schemas (top-level, not in modules)
        let components_to_generate = self.component_schemas.clone();
        for (schema_name, schema) in components_to_generate {
            let code = self.generate_schema_type(&schema_name, &schema);
            if !code.is_empty() {
                output.push_str(&code);
                output.push_str("\n\n");
            }
        }

        // Generate modules for path-based schemas
        let modules_to_generate = self.module_schemas.clone();

        for (module_name, schemas) in modules_to_generate {
            output.push_str(&format!("pub mod {} {{\n", module_name));
            output.push_str("    use super::*;\n\n");

            for (type_name, schema) in schemas {
                let code = self.generate_schema_type(&type_name, &schema);
                if !code.is_empty() {
                    // Indent the code for the module
                    let indented = code
                        .lines()
                        .map(|line| {
                            if line.is_empty() {
                                String::new()
                            } else {
                                format!("    {}", line)
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    output.push_str(&indented);
                    output.push_str("\n\n");
                }
            }

            output.push_str("}\n\n");
        }

        output
    }

    fn extract_schemas_from_paths(&mut self) {
        // Clone paths to avoid borrow checker issues
        let paths_clone = if let Some(paths) = &self.spec.paths {
            paths.clone()
        } else {
            return;
        };

        for (path, path_item) in paths_clone {
            // Get module name and path name
            let (mod_name, path_name) = Self::path_to_mod_and_name(&path);

            // Check each operation (get, post, put, delete, etc.)
            if let Some(op) = &path_item.get {
                self.extract_schemas_from_operation(&mod_name, &path_name, "GET", &op);
            }
            if let Some(op) = &path_item.post {
                self.extract_schemas_from_operation(&mod_name, &path_name, "POST", &op);
            }
            if let Some(op) = &path_item.put {
                self.extract_schemas_from_operation(&mod_name, &path_name, "PUT", &op);
            }
            if let Some(op) = &path_item.delete {
                self.extract_schemas_from_operation(&mod_name, &path_name, "DELETE", &op);
            }
            if let Some(op) = &path_item.patch {
                self.extract_schemas_from_operation(&mod_name, &path_name, "PATCH", &op);
            }
        }
    }

    fn extract_schemas_from_operation(
        &mut self,
        mod_name: &str,
        path_name: &str,
        method: &str,
        operation: &oas3::spec::Operation,
    ) {
        // Extract request body schema
        if let Some(request_body) = &operation.request_body {
            match request_body {
                ObjectOrReference::Object(body) => {
                    if let Some(content) = body.content.get("application/json") {
                        if let Some(schema) = &content.schema {
                            let name = format!("{}{}Request", path_name, method);
                            self.extract_schema_to_module(mod_name, &name, schema);
                        }
                    }
                }
                _ => {}
            }
        }

        // Extract response schemas
        if let Some(responses) = &operation.responses {
            for (status_code, response) in responses {
                match response {
                    ObjectOrReference::Object(resp) => {
                        if let Some(content) = resp.content.get("application/json") {
                            if let Some(schema) = &content.schema {
                                let status = status_code.replace("XX", "").replace("\"", "");
                                let name = format!("{}{}Response{}", path_name, method, status);
                                self.extract_schema_to_module(mod_name, &name, schema);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn extract_schema_to_module(
        &mut self,
        mod_name: &str,
        type_name: &str,
        schema: &ObjectOrReference<ObjectSchema>,
    ) {
        match schema {
            ObjectOrReference::Object(obj_schema) => {
                // Only add if it has properties or is an enum
                if !obj_schema.properties.is_empty() || !obj_schema.enum_values.is_empty() {
                    self.module_schemas
                        .entry(mod_name.to_string())
                        .or_insert_with(Vec::new)
                        .push((type_name.to_string(), obj_schema.clone()));
                }
            }
            ObjectOrReference::Ref { .. } => {
                // Skip refs, they point to components/schemas
            }
        }
    }

    fn path_to_mod_and_name(path: &str) -> (String, String) {
        // Convert /path/{param}/action to ("path", "ParamAction")
        let parts: Vec<String> = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| {
                // Remove parameter braces
                let cleaned = s.replace('{', "").replace('}', "");
                to_pascal_case(&cleaned)
            })
            .collect();

        if parts.is_empty() {
            return ("root".to_string(), String::new());
        }

        // First part becomes the module name (lowercase)
        let mod_name = parts[0].to_lowercase();

        // Rest becomes the type name prefix
        let type_name_prefix = parts[1..].join("");

        (mod_name, type_name_prefix)
    }

    fn generate_schema_type(&mut self, name: &str, schema: &ObjectSchema) -> String {
        use schema_generator::SchemaGenerator;

        if self.generated_types.contains(name) {
            return String::new();
        }

        self.generated_types.insert(name.to_string());
        SchemaGenerator::generate(name, schema)
    }
}
