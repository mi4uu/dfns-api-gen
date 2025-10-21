// Converts OpenAPI schemas to IR (Intermediate Representation)
//
// This module bridges the gap between OpenAPI 3.x spec parsing and
// our structured IR types, handling all the complexity of OpenAPI schemas.

use crate::codegen::utils::{sanitize_field_name, sanitize_variant_name, to_pascal_case, to_snake_case};
use crate::ir::*;
use oas3::spec::{ObjectOrReference, ObjectSchema, SchemaTypeSet, Spec};
use std::collections::BTreeMap;

/// Converter from OpenAPI Spec to IR
pub struct OpenApiToIr {
    spec: Spec,
    /// Track generated types to avoid duplicates
    generated_types: std::collections::HashSet<String>,
}

impl OpenApiToIr {
    pub fn new(spec: Spec) -> Self {
        Self {
            spec,
            generated_types: std::collections::HashSet::new(),
        }
    }

    /// Convert the entire OpenAPI spec to CodeGenOutput
    pub fn convert(&mut self) -> CodeGenOutput {
        let mut output = CodeGenOutput::new();

        // Add standard derives to the default imports
        output = output.with_import("use std::collections::HashMap;".to_string());

        // Convert component schemas (top-level types)
        if let Some(components) = &self.spec.components {
            let mut component_schemas: Vec<_> = components.schemas.iter().collect();
            component_schemas.sort_by(|a, b| a.0.cmp(b.0)); // Sort for deterministic output

            for (name, schema_ref) in component_schemas {
                if let Some(type_def) = self.schema_ref_to_type_def(name, schema_ref) {
                    output = output.with_type(type_def);
                }
            }
        }

        // Extract and convert path-based schemas
        let path_modules = self.extract_path_modules();
        for module in path_modules {
            output = output.with_module(module);
        }

        output
    }

    /// Convert a schema reference to a TypeDef
    fn schema_ref_to_type_def(
        &mut self,
        name: &str,
        schema_ref: &ObjectOrReference<ObjectSchema>,
    ) -> Option<TypeDef> {
        match schema_ref {
            ObjectOrReference::Object(schema) => self.schema_to_type_def(name, schema),
            ObjectOrReference::Ref { .. } => None, // Skip refs, they point to other schemas
        }
    }

    /// Convert an ObjectSchema to a TypeDef
    fn schema_to_type_def(&mut self, name: &str, schema: &ObjectSchema) -> Option<TypeDef> {
        // Check if already generated
        if self.generated_types.contains(name) {
            return None;
        }
        self.generated_types.insert(name.to_string());

        // Handle enums (string enums)
        if !schema.enum_values.is_empty() {
            return Some(TypeDef::Enum(self.schema_to_enum(name, schema)));
        }

        // Handle oneOf (union types as enums)
        if !schema.one_of.is_empty() {
            return Some(TypeDef::Enum(self.schema_one_of_to_enum(name, schema)));
        }

        // Handle allOf (composition)
        if !schema.all_of.is_empty() {
            return Some(TypeDef::Struct(self.schema_all_of_to_struct(name, schema)));
        }

        // Handle objects with properties (structs)
        if !schema.properties.is_empty() {
            return Some(TypeDef::Struct(self.schema_to_struct(name, schema)));
        }

        // Skip schemas without meaningful content
        None
    }

    /// Convert schema to enum (for string enums)
    fn schema_to_enum(&self, name: &str, schema: &ObjectSchema) -> EnumDef {
        let mut enum_def = EnumDef::new(name)
            .add_derive("utoipa::ToSchema".to_string())
            .add_derive("smart_default::SmartDefault".to_string());

        if let Some(desc) = &schema.description {
            enum_def = enum_def.with_doc(desc);
        }

        let mut first = true;
        for value in &schema.enum_values {
            if let Some(variant_str) = value.as_str() {
                let variant_name = sanitize_variant_name(variant_str);
                let mut variant = VariantDef::unit(&variant_name);

                // Add rename if variant name differs from original
                if variant_name != variant_str {
                    variant = variant.with_serde_rename(variant_str);
                }

                // Mark first variant as default
                if first {
                    variant = variant.as_default();
                    first = false;
                }

                enum_def = enum_def.with_variant(variant);
            }
        }

        enum_def
    }

    /// Convert oneOf to an untagged enum
    fn schema_one_of_to_enum(&mut self, name: &str, schema: &ObjectSchema) -> EnumDef {
        let mut enum_def = EnumDef::new(name)
            .with_representation(EnumRepresentation::Untagged)
            .add_derive("utoipa::ToSchema".to_string());

        if let Some(desc) = &schema.description {
            enum_def = enum_def.with_doc(desc);
        }

        for (i, variant_schema_ref) in schema.one_of.iter().enumerate() {
            match variant_schema_ref {
                ObjectOrReference::Object(variant_schema) => {
                    // Try to get meaningful name from title
                    let variant_name = if let Some(title) = &variant_schema.title {
                        to_pascal_case(title)
                    } else {
                        format!("Variant{}", i)
                    };

                    let variant_type = self.schema_to_rust_type(variant_schema, true);
                    enum_def = enum_def.with_variant(VariantDef::tuple(&variant_name, variant_type));
                }
                ObjectOrReference::Ref { ref_path, .. } => {
                    let ref_type = Self::extract_ref_name(ref_path);
                    enum_def = enum_def.with_variant(VariantDef::tuple(&ref_type, RustType::named(&ref_type)));
                }
            }
        }

        enum_def
    }

    /// Convert allOf to a struct (merge all schemas)
    fn schema_all_of_to_struct(&mut self, name: &str, schema: &ObjectSchema) -> StructDef {
        let mut merged_schema = schema.clone();

        // Merge all schemas in allOf
        for schema_ref in &schema.all_of {
            if let ObjectOrReference::Object(sub_schema) = schema_ref {
                // Merge properties
                for (key, value) in &sub_schema.properties {
                    merged_schema.properties.insert(key.clone(), value.clone());
                }
                // Merge required fields
                for req in &sub_schema.required {
                    if !merged_schema.required.contains(req) {
                        merged_schema.required.push(req.clone());
                    }
                }
            }
        }

        self.schema_to_struct(name, &merged_schema)
    }

    /// Convert schema to struct
    fn schema_to_struct(&mut self, name: &str, schema: &ObjectSchema) -> StructDef {
        let mut struct_def = StructDef::new(name)
            .add_derive("utoipa::ToSchema".to_string())
            .add_derive("smart_default::SmartDefault".to_string());

        if let Some(desc) = &schema.description {
            struct_def = struct_def.with_doc(desc);
        }

        // Sort properties for deterministic output
        let mut prop_names: Vec<_> = schema.properties.keys().collect();
        prop_names.sort();

        for prop_name in prop_names {
            if let Some(prop_schema_ref) = schema.properties.get(prop_name) {
                let is_required = schema.required.contains(prop_name);
                if let Some(field) = self.property_to_field(prop_name, prop_schema_ref, is_required, name) {
                    struct_def = struct_def.with_field(field);
                }
            }
        }

        struct_def
    }

    /// Convert an OpenAPI property to a FieldDef
    fn property_to_field(
        &mut self,
        prop_name: &str,
        schema_ref: &ObjectOrReference<ObjectSchema>,
        is_required: bool,
        parent_type_name: &str,
    ) -> Option<FieldDef> {
        let field_name = sanitize_field_name(&to_snake_case(prop_name));

        let (rust_type, doc) = match schema_ref {
            ObjectOrReference::Object(schema) => {
                let doc = schema.description.clone();

                // Handle inline enums
                if !schema.enum_values.is_empty() {
                    let enum_name = format!("{}{}", parent_type_name, to_pascal_case(prop_name));
                    // Note: Inline enum generation would need to be handled separately
                    // For now, we'll use a named type reference
                    (RustType::named(enum_name), doc)
                } else if !schema.properties.is_empty() {
                    // Handle inline structs
                    let struct_name = format!("{}{}", parent_type_name, to_pascal_case(prop_name));
                    (RustType::named(struct_name), doc)
                } else {
                    (self.schema_to_rust_type(schema, is_required), doc)
                }
            }
            ObjectOrReference::Ref { ref_path, .. } => {
                let type_name = Self::extract_ref_name(ref_path);
                (RustType::named(type_name), None)
            }
        };

        let mut field = FieldDef::new(field_name, rust_type);

        if let Some(d) = doc {
            field = field.with_doc(d);
        }

        // Add serde rename if needed
        if field.name != prop_name {
            field = field.with_serde_rename(prop_name);
        }

        // Set optional/required
        if is_required {
            field = field.required();
        } else {
            field = field.optional();
        }

        Some(field)
    }

    /// Convert OpenAPI schema to RustType
    fn schema_to_rust_type(&self, schema: &ObjectSchema, is_required: bool) -> RustType {
        // Handle arrays
        if let Some(items) = &schema.items {
            let item_type = match items.as_ref() {
                ObjectOrReference::Object(item_schema) => self.schema_to_rust_type(item_schema, true),
                ObjectOrReference::Ref { ref_path, .. } => {
                    RustType::named(Self::extract_ref_name(ref_path))
                }
            };
            return RustType::Vec(Box::new(item_type));
        }

        // Handle type mappings
        let rust_type = match &schema.schema_type {
            Some(type_set) => Self::type_set_to_rust_type(type_set, schema),
            None => {
                // No explicit type - try to infer
                if schema.properties.is_empty() {
                    RustType::JsonValue
                } else {
                    RustType::JsonValue // Object without type
                }
            }
        };

        rust_type
    }

    /// Convert SchemaTypeSet to RustType
    fn type_set_to_rust_type(type_set: &SchemaTypeSet, schema: &ObjectSchema) -> RustType {
        use oas3::spec::SchemaType;

        // Get the primary type from the set
        let primary_type = if type_set.contains(&SchemaType::String) {
            SchemaType::String
        } else if type_set.contains(&SchemaType::Integer) {
            SchemaType::Integer
        } else if type_set.contains(&SchemaType::Number) {
            SchemaType::Number
        } else if type_set.contains(&SchemaType::Boolean) {
            SchemaType::Boolean
        } else if type_set.contains(&SchemaType::Array) {
            SchemaType::Array
        } else if type_set.contains(&SchemaType::Object) {
            SchemaType::Object
        } else {
            return RustType::JsonValue;
        };

        match primary_type {
            SchemaType::String => RustType::String,
            SchemaType::Integer => {
                // Check format for int32 vs int64
                match schema.format.as_deref() {
                    Some("int32") => RustType::I32,
                    Some("int64") => RustType::I64,
                    _ => RustType::I64, // Default to i64
                }
            }
            SchemaType::Number => {
                // Check format for float vs double
                match schema.format.as_deref() {
                    Some("float") => RustType::F32,
                    Some("double") => RustType::F64,
                    _ => RustType::F64, // Default to f64
                }
            }
            SchemaType::Boolean => RustType::Bool,
            SchemaType::Array => {
                // Should be handled earlier, but fallback
                RustType::Vec(Box::new(RustType::JsonValue))
            }
            SchemaType::Object => {
                // Check if it's a map (additionalProperties)
                if let Some(additional) = &schema.additional_properties {
                    match additional.as_ref() {
                        ObjectOrReference::Object(value_schema) => {
                            if let Some(type_set) = &value_schema.schema_type {
                                let value_type = Self::type_set_to_rust_type(type_set, value_schema);
                                RustType::HashMap(Box::new(RustType::String), Box::new(value_type))
                            } else {
                                RustType::HashMap(Box::new(RustType::String), Box::new(RustType::JsonValue))
                            }
                        }
                        ObjectOrReference::Ref { ref_path, .. } => {
                            let ref_name = Self::extract_ref_name(&ref_path);
                            RustType::HashMap(
                                Box::new(RustType::String),
                                Box::new(RustType::named(ref_name)),
                            )
                        }
                    }
                } else {
                    RustType::JsonValue
                }
            }
            _ => RustType::JsonValue,
        }
    }

    /// Extract type name from $ref path
    fn extract_ref_name(ref_path: &str) -> String {
        ref_path
            .split('/')
            .last()
            .unwrap_or("Unknown")
            .to_string()
    }

    /// Extract schemas from API paths and organize them into modules
    fn extract_path_modules(&mut self) -> Vec<ModuleDef> {
        let mut modules: BTreeMap<String, Vec<TypeDef>> = BTreeMap::new();

        if let Some(paths) = &self.spec.paths {
            for (path, path_item) in paths {
                let (module_name, path_prefix) = Self::path_to_module_and_prefix(path);

                // Check each HTTP method
                for (method, operation) in [
                    ("GET", &path_item.get),
                    ("POST", &path_item.post),
                    ("PUT", &path_item.put),
                    ("DELETE", &path_item.delete),
                    ("PATCH", &path_item.patch),
                ] {
                    if let Some(op) = operation {
                        // Extract request type
                        if let Some(req_body) = &op.request_body {
                            if let ObjectOrReference::Object(body) = req_body {
                                if let Some(content) = body.content.get("application/json") {
                                    if let Some(schema_ref) = &content.schema {
                                        if let ObjectOrReference::Object(schema) = schema_ref {
                                            if !schema.properties.is_empty() || !schema.enum_values.is_empty() {
                                                let type_name = format!("{}{}Request", path_prefix, method);
                                                if let Some(type_def) = self.schema_to_type_def(&type_name, schema) {
                                                    modules.entry(module_name.clone()).or_default().push(type_def);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Extract response types
                        if let Some(responses) = &op.responses {
                            for (status, response_ref) in responses {
                                if let ObjectOrReference::Object(response) = response_ref {
                                    if let Some(content) = response.content.get("application/json") {
                                        if let Some(schema_ref) = &content.schema {
                                            if let ObjectOrReference::Object(schema) = schema_ref {
                                                if !schema.properties.is_empty() || !schema.enum_values.is_empty() {
                                                    let status_suffix = if status == "200" { String::new() } else { status.clone() };
                                                    let type_name = format!("{}{}Response{}", path_prefix, method, status_suffix);
                                                    if let Some(type_def) = self.schema_to_type_def(&type_name, schema) {
                                                        modules.entry(module_name.clone()).or_default().push(type_def);
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
            }
        }

        // Convert to ModuleDef
        modules
            .into_iter()
            .map(|(name, types)| ModuleDef::new(name).with_types(types))
            .collect()
    }

    /// Convert path to module name and type prefix
    fn path_to_module_and_prefix(path: &str) -> (String, String) {
        let parts: Vec<String> = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| {
                let cleaned = s.replace('{', "").replace('}', "");
                to_pascal_case(&cleaned)
            })
            .collect();

        if parts.is_empty() {
            return ("root".to_string(), String::new());
        }

        let module_name = parts[0].to_lowercase();
        let type_prefix = parts[1..].join("");

        (module_name, type_prefix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_ref_name() {
        assert_eq!(
            OpenApiToIr::extract_ref_name("#/components/schemas/User"),
            "User"
        );
    }

    #[test]
    fn test_path_to_module_and_prefix() {
        let (module, prefix) = OpenApiToIr::path_to_module_and_prefix("/wallets/{walletId}/export");
        assert_eq!(module, "wallets");
        assert_eq!(prefix, "WalletIdExport");
    }
}
