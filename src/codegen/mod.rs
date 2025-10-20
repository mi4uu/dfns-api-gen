mod schema_generator;
mod type_mapper;
mod utils;

use oas3::spec::{ObjectOrReference, ObjectSchema, Spec};
use std::collections::HashSet;

pub struct Generator {
    spec: Spec,
    generated_types: HashSet<String>,
}

impl Generator {
    pub fn new(spec: Spec) -> Self {
        Self {
            spec,
            generated_types: HashSet::new(),
        }
    }

    pub fn generate(&mut self) -> String {
        let mut output = String::new();

        // Add header and imports
        output.push_str("// Auto-generated from OpenAPI schema\n");
        output.push_str("// Do not edit manually\n\n");
        output.push_str("use serde::{Deserialize, Serialize};\n\n");

        // Generate types from components/schemas
        // Collect schema names and refs first to avoid borrow checker issues
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

        output
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
