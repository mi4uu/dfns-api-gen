mod schema_generator;
mod type_mapper;
mod utils;

use crate::codegen::utils::to_pascal_case;
use oas3::spec::{ObjectOrReference, ObjectSchema, Spec};
use std::collections::{HashMap, HashSet};

pub struct Generator {
    spec: Spec,
    generated_types: HashSet<String>,
    inline_schemas: HashMap<String, ObjectSchema>,
}

impl Generator {
    pub fn new(spec: Spec) -> Self {
        Self {
            spec,
            generated_types: HashSet::new(),
            inline_schemas: HashMap::new(),
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

        // Generate types from components/schemas
        let schemas_to_generate: Vec<(String, ObjectOrReference<ObjectSchema>)> =
            if let Some(components) = &self.spec.components {
                let mut items: Vec<_> = components
                    .schemas
                    .iter()
                    .map(|(name, schema_ref)| (name.clone(), schema_ref.clone()))
                    .collect();
                items.sort_by(|a, b| a.0.cmp(&b.0));
                items
            } else {
                vec![]
            };

        for (schema_name, schema_ref) in schemas_to_generate {
            let code = self.generate_schema_type(&schema_name, &schema_ref);
            if !code.is_empty() {
                output.push_str(&code);
                output.push_str("\n\n");
            }
        }

        // Generate types from inline schemas found in paths
        let mut inline_names: Vec<_> = self.inline_schemas.keys().cloned().collect();
        inline_names.sort();

        for name in inline_names {
            if let Some(schema) = self.inline_schemas.get(&name).cloned() {
                let code = self.generate_schema_type(&name, &ObjectOrReference::Object(schema));
                if !code.is_empty() {
                    output.push_str(&code);
                    output.push_str("\n\n");
                }
            }
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
            // Clean up path for use in type names
            let path_name = self.path_to_name(&path);
            let (mod_name, path_name) = self.path_to_mod_and_name(&path);

            // Check each operation (get, post, put, delete, etc.)
            if let Some(op) = &path_item.get {
                self.extract_schemas_from_operation(&path_name, "Get", op);
            }
            if let Some(op) = &path_item.post {
                self.extract_schemas_from_operation(&path_name, "Post", op);
            }
            if let Some(op) = &path_item.put {
                self.extract_schemas_from_operation(&path_name, "Put", op);
            }
            if let Some(op) = &path_item.delete {
                self.extract_schemas_from_operation(&path_name, "Delete", op);
            }
            if let Some(op) = &path_item.patch {
                self.extract_schemas_from_operation(&path_name, "Patch", op);
            }
        }
    }

    fn extract_schemas_from_operation(
        &mut self,
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
                            self.extract_schema_from_media(&name, schema);
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
                                println!("Extract response schemas : new name : {}, path name: {},  method: {}", &name, path_name, method);
                                self.extract_schema_from_media(&name, schema);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn extract_schema_from_media(&mut self, name: &str, schema: &ObjectOrReference<ObjectSchema>) {
        match schema {
            ObjectOrReference::Object(obj_schema) => {
                // Only add if it has properties or is an enum
                if !obj_schema.properties.is_empty() || !obj_schema.enum_values.is_empty() {
                    self.inline_schemas
                        .insert(name.to_string(), obj_schema.clone());
                }
            }
            ObjectOrReference::Ref { .. } => {
                // Skip refs, they point to components/schemas
            }
        }
    }

    fn path_to_name(&self, path: &str) -> String {
        // Convert /path/{param}/action to PathParamAction
        path.split('/')
            .filter(|s| !s.is_empty())
            .map(|s| {
                // Remove parameter braces
                let cleaned = s.replace('{', "").replace('}', "");
                to_pascal_case(&cleaned)
            })
            .collect::<Vec<_>>()
            .join("")
    }
    fn path_to_mod_and_name(&self, path: &str) -> (String, String) {
        // Convert /path/{param}/action to PathParamAction
        let pathandmod = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| {
                // Remove parameter braces
                let cleaned = s.replace('{', "").replace('}', "");
                to_pascal_case(&cleaned)
            })
            .collect::<Vec<_>>();
        // let modname=pathandmod.first().unwrap();
        let (modname, name) = pathandmod.split_at(1);
        let name = name.join("");
        (
            String::from(modname.join("").to_lowercase()),
            String::from(name),
        )
    }

    fn generate_schema_type(
        &mut self,
        name: &str,
        schema_ref: &ObjectOrReference<ObjectSchema>,
    ) -> String {
        use schema_generator::SchemaGenerator;

        if self.generated_types.contains(name) {
            return String::new();
        }

        self.generated_types.insert(name.to_string());

        match schema_ref {
            ObjectOrReference::Object(obj_schema) => SchemaGenerator::generate(name, obj_schema),
            ObjectOrReference::Ref { .. } => {
                // Handle $ref - for now we'll skip, as they'll be generated when we encounter the actual definition
                String::new()
            }
        }
    }
}
