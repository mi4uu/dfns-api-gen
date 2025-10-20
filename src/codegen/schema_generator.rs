use crate::codegen::type_mapper::TypeMapper;
use crate::codegen::utils::*;
use oas3::spec::{ObjectOrReference, ObjectSchema};

pub struct SchemaGenerator;

impl SchemaGenerator {
    pub fn generate(name: &str, schema: &ObjectSchema) -> String {
        // Handle enums
        if !schema.enum_values.is_empty() {
            return Self::generate_enum(name, schema, &schema.enum_values);
        }

        // Handle oneOf (enum variants with different types)
        if !schema.one_of.is_empty() {
            return Self::generate_one_of_enum(name, schema, &schema.one_of);
        }

        // Handle allOf (composition/inheritance)
        if !schema.all_of.is_empty() {
            return Self::generate_all_of_struct(name, schema, &schema.all_of);
        }

        // Handle object types
        if !schema.properties.is_empty() {
            return Self::generate_struct(name, schema);
        }

        // For simple type aliases (e.g., type definitions)
        String::new()
    }

    fn generate_enum(
        name: &str,
        schema: &ObjectSchema,
        enum_values: &[serde_json::Value],
    ) -> String {
        let mut output = String::new();

        // Add doc comment
        output.push_str(&generate_doc_comment(&schema.description));

        // Add derives
        output.push_str(
            "#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, utoipa::ToSchema)]\n",
        );

        // Start enum
        output.push_str(&format!("pub enum {} {{\n", name));

        // Add variants
        for value in enum_values {
            if let Some(variant_str) = value.as_str() {
                let variant_name = sanitize_variant_name(variant_str);

                // If the original value is different from the sanitized name, add rename
                if variant_name != variant_str {
                    output.push_str(&format!("    #[serde(rename = \"{}\")]\n", variant_str));
                }

                output.push_str(&format!("    {},\n", variant_name));
            }
        }

        output.push_str("}");
        output
    }

    fn generate_one_of_enum(
        name: &str,
        schema: &ObjectSchema,
        one_of: &[ObjectOrReference<ObjectSchema>],
    ) -> String {
        let mut output = String::new();

        // Add doc comment
        output.push_str(&generate_doc_comment(&schema.description));

        // Add derives
        output.push_str(
            "#[derive(Debug, Clone, PartialEq,  Serialize, Deserialize, utoipa::ToSchema)]\n",
        );
        output.push_str("#[serde(untagged)]\n");

        // Start enum
        output.push_str(&format!("pub enum {} {{\n", name));

        // Add variants for each oneOf option
        for (i, schema_ref) in one_of.iter().enumerate() {
            match schema_ref {
                ObjectOrReference::Object(variant_schema) => {
                    // Try to get a meaningful name from title or description
                    let variant_name = if let Some(title) = &variant_schema.title {
                        to_pascal_case(title)
                    } else {
                        format!("Variant{}", i)
                    };

                    // Generate the variant type
                    let variant_type = Self::get_variant_type(
                        variant_schema,
                        &format!("{}_{}", name, variant_name),
                    );

                    output.push_str(&format!("    {}({}),\n", variant_name, variant_type));
                }
                ObjectOrReference::Ref { ref_path, .. } => {
                    let ref_type = TypeMapper::extract_ref_name(ref_path);
                    let variant_name = ref_type.clone();
                    output.push_str(&format!("    {}({}),\n", variant_name, ref_type));
                }
            }
        }

        output.push_str("}");
        output
    }

    fn get_variant_type(schema: &ObjectSchema, _suggested_name: &str) -> String {
        if !schema.properties.is_empty() {
            // This is an inline object, we'd need to generate it separately
            // For now, use serde_json::Value
            "serde_json::Value".to_string()
        } else {
            TypeMapper::map_type_from_object_schema(schema, true)
        }
    }

    fn generate_all_of_struct(
        name: &str,
        schema: &ObjectSchema,
        all_of: &[ObjectOrReference<ObjectSchema>],
    ) -> String {
        // Merge all schemas in allOf
        let mut merged_schema = schema.clone();

        for schema_ref in all_of {
            match schema_ref {
                ObjectOrReference::Object(sub_schema) => {
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
                ObjectOrReference::Ref { .. } => {
                    // For refs in allOf, we'd need to flatten them
                    // This is complex, so for now we'll skip
                }
            }
        }

        Self::generate_struct(name, &merged_schema)
    }

    fn generate_struct(name: &str, schema: &ObjectSchema) -> String {
        let mut output = String::new();
        let mut inline_types = Vec::new();

        // Get required fields
        let required_fields = &schema.required;

        // Collect fields and inline types (enums and structs)
        let mut field_codes = String::new();
        let mut prop_names: Vec<_> = schema.properties.keys().collect();
        prop_names.sort();

        for prop_name in prop_names {
            if let Some(prop_schema_ref) = schema.properties.get(prop_name) {
                let is_required = required_fields.contains(prop_name);
                let (field_code, types) =
                    Self::generate_field(prop_name, prop_schema_ref, is_required, name);

                field_codes.push_str(&field_code);
                inline_types.extend(types);
            }
        }

        // Generate inline types (enums and structs) first
        for (type_name, type_schema) in inline_types {
            // Check if it's an enum or struct
            if !type_schema.enum_values.is_empty() {
                output.push_str(&Self::generate_enum(
                    &type_name,
                    &type_schema,
                    &type_schema.enum_values,
                ));
            } else if !type_schema.properties.is_empty() {
                output.push_str(&Self::generate_struct(&type_name, &type_schema));
            }
            output.push_str("\n\n");
        }

        // Add doc comment
        output.push_str(&generate_doc_comment(&schema.description));

        // Add derives
        output.push_str(
            "#[derive(Debug, Clone, PartialEq, Serialize, Deserialize,   utoipa::ToSchema)]\n",
        );

        // Start struct
        output.push_str(&format!("pub struct {} {{\n", name));

        // Add fields
        output.push_str(&field_codes);

        output.push_str("}");
        output
    }

    fn generate_field(
        name: &str,
        schema_ref: &ObjectOrReference<ObjectSchema>,
        is_required: bool,
        parent_type_name: &str,
    ) -> (String, Vec<(String, ObjectSchema)>) {
        let mut output = String::new();
        let mut inline_types = Vec::new();

        let schema = match schema_ref {
            ObjectOrReference::Object(s) => s,
            ObjectOrReference::Ref { ref_path, .. } => {
                // For $ref, we just use the referenced type name
                let field_name = sanitize_field_name(&to_snake_case(name));
                let rust_type = TypeMapper::extract_ref_name(ref_path);
                let final_type = if is_required {
                    rust_type
                } else {
                    format!("Option<{}>", rust_type)
                };

                // Check if we need rename
                if field_name != name {
                    output.push_str(&format!("    #[serde(rename = \"{}\")]\n", name));
                }

                // Add skip_serializing_if for Option fields
                if !is_required {
                    output.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                }

                output.push_str(&format!("    pub {}: {},\n", field_name, final_type));
                return (output, inline_types);
            }
        };

        // Check if this field is an inline enum
        if !schema.enum_values.is_empty() {
            // Generate a separate enum type for this field
            // Prefix with parent type name to avoid collisions
            let enum_type_name = format!("{}{}", parent_type_name, to_pascal_case(name));
            inline_types.push((enum_type_name.clone(), schema.clone()));

            let field_name = sanitize_field_name(&to_snake_case(name));
            let final_type = if is_required {
                enum_type_name
            } else {
                format!("Option<{}>", enum_type_name)
            };

            // Check if we need rename
            if field_name != name {
                output.push_str(&format!("    #[serde(rename = \"{}\")]\n", name));
            }

            // Add skip_serializing_if for Option fields
            if !is_required {
                output.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
            }

            output.push_str(&format!("    pub {}: {},\n", field_name, final_type));
            return (output, inline_types);
        }

        // Check if this field is an inline object (struct with properties)
        if !schema.properties.is_empty() {
            // Generate a separate struct type for this field
            // Prefix with parent type name to avoid collisions
            let struct_type_name = format!("{}{}", parent_type_name, to_pascal_case(name));
            inline_types.push((struct_type_name.clone(), schema.clone()));

            let field_name = sanitize_field_name(&to_snake_case(name));
            let final_type = if is_required {
                struct_type_name
            } else {
                format!("Option<{}>", struct_type_name)
            };

            // Add doc comment for field
            output.push_str(&generate_doc_comment(&schema.description).replace("/// ", "    /// "));

            // Check if we need rename
            if field_name != name {
                output.push_str(&format!("    #[serde(rename = \"{}\")]\n", name));
            }

            // Add skip_serializing_if for Option fields
            if !is_required {
                output.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
            }

            output.push_str(&format!("    pub {}: {},\n", field_name, final_type));
            return (output, inline_types);
        }

        // Add doc comment for field
        output.push_str(&generate_doc_comment(&schema.description).replace("/// ", "    /// "));

        let field_name = sanitize_field_name(&to_snake_case(name));
        let rust_type = TypeMapper::map_type_from_object_schema(schema, is_required);

        // Check if we need rename
        if field_name != name {
            output.push_str(&format!("    #[serde(rename = \"{}\")]\n", name));
        }

        // Add skip_serializing_if for Option fields
        if !is_required {
            output.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
        }

        output.push_str(&format!("    pub {}: {},\n", field_name, rust_type));
        (output, inline_types)
    }
}
