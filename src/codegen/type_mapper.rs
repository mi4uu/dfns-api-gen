use oas3::spec::{ObjectOrReference, ObjectSchema, Schema, SchemaType, SchemaTypeSet};

pub struct TypeMapper;

impl TypeMapper {
    /// Map ObjectSchema to Rust type
    pub fn map_type_from_object_schema(schema: &ObjectSchema, is_required: bool) -> String {
        let base_type = Self::get_base_type_from_object_schema(schema);

        if is_required {
            base_type
        } else {
            format!("Option<{}>", base_type)
        }
    }

    fn get_base_type_from_object_schema(schema: &ObjectSchema) -> String {
        // Handle array types
        if let Some(items) = &schema.items {
            let item_type = match items.as_ref() {
                Schema::Object(obj_ref) => match obj_ref.as_ref() {
                    ObjectOrReference::Object(item_schema) => {
                        Self::get_base_type_from_object_schema(item_schema)
                    }
                    ObjectOrReference::Ref { .. } => {
                        Self::extract_ref_from_obj_ref(obj_ref.as_ref())
                    }
                },
                Schema::Boolean(_) => "serde_json::Value".to_string(),
            };
            if item_type
                .clone()
                .eq_ignore_ascii_case(String::from("serde_json::Value").as_str())
            {
                println!("!!!!!!  {}", &item_type);
                println!("!!!!!!  {:#?}", &schema.items);
            }
            return format!("Vec<{}>", item_type);
        }

        // Check for enum values
        if !schema.enum_values.is_empty() {
            // Enums are generated as separate types
            // This should only be reached for inline enums
            return "String".to_string();
        }

        // Handle schema type - SchemaTypeSet is either Single(SchemaType) or Multiple(Vec<SchemaType>)
        if let Some(type_set) = &schema.schema_type {
            let rust_type = match type_set {
                SchemaTypeSet::Single(schema_type) => {
                    Self::map_single_type(schema_type, &schema.format)
                }
                SchemaTypeSet::Multiple(types) => {
                    // For multiple types, we'll use the first non-null type
                    // or fall back to serde_json::Value
                    types
                        .first()
                        .map(|t| Self::map_single_type(t, &schema.format))
                        .unwrap_or_else(|| "serde_json::Value".to_string())
                }
            };
            return rust_type;
        }

        // No type specified, use generic Value
        "serde_json::Value".to_string()
    }

    fn map_single_type(schema_type: &SchemaType, format: &Option<String>) -> String {
        match schema_type {
            SchemaType::String => {
                if let Some(fmt) = format {
                    match fmt.as_str() {
                        "date-time" => "String".to_string(), // Could use chrono::DateTime
                        "date" => "String".to_string(),
                        "uuid" => "String".to_string(),
                        _ => "String".to_string(),
                    }
                } else {
                    "String".to_string()
                }
            }
            SchemaType::Integer => {
                if let Some(fmt) = format {
                    match fmt.as_str() {
                        "int32" => "i32".to_string(),
                        "int64" => "i64".to_string(),
                        _ => "i64".to_string(),
                    }
                } else {
                    "i64".to_string()
                }
            }
            SchemaType::Number => {
                if let Some(fmt) = format {
                    match fmt.as_str() {
                        "float" => "f32".to_string(),
                        "double" => "f64".to_string(),
                        _ => "f64".to_string(),
                    }
                } else {
                    "f64".to_string()
                }
            }
            SchemaType::Boolean => "bool".to_string(),
            SchemaType::Array => "Vec<serde_json::Value>".to_string(),
            SchemaType::Object => "serde_json::Value".to_string(),
            SchemaType::Null => "()".to_string(),
        }
    }

    /// Extract the type name from a $ref path
    pub fn extract_ref_name(ref_path: &str) -> String {
        ref_path
            .split('/')
            .last()
            .unwrap_or("UnknownRef")
            .to_string()
    }

    /// Extract ref name from ObjectOrReference
    fn extract_ref_from_obj_ref(obj_ref: &ObjectOrReference<ObjectSchema>) -> String {
        match obj_ref {
            ObjectOrReference::Ref { ref_path, .. } => Self::extract_ref_name(ref_path),
            ObjectOrReference::Object(_) => "serde_json::Value".to_string(),
        }
    }
}
