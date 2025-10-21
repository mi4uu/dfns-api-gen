// New generator with better naming and nested modules
use crate::codegen::schema_generator::SchemaGenerator;
use crate::codegen::utils::{to_pascal_case, to_snake_case};
use crate::ir::{CompleteIR, EndpointMetadata, TypeLocation};
use oas3::spec::{ObjectOrReference, ObjectSchema, Operation, Spec};
use std::collections::{BTreeMap, HashMap, HashSet};

pub struct NewGenerator {
    spec: Spec,
    generated_types: HashSet<String>,
    ir: CompleteIR,
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

    fn generate_code(&self, generated_types: &mut HashSet<String>, indent: usize, ir: &mut CompleteIR, module_path: &[String]) -> String {
        let mut output = String::new();
        let indent_str = "    ".repeat(indent);

        // Generate types in this module
        for (type_name, schema) in &self.types {
            let full_name = format!("{}::{}", self.name, type_name);
            if !generated_types.contains(&full_name) {
                let code = SchemaGenerator::generate(type_name, schema);
                if !code.is_empty() {
                    generated_types.insert(full_name.clone());  // Only insert if code was generated

                    // Register in IR since code was actually generated
                    let ir_key = if module_path.is_empty() {
                        type_name.clone()
                    } else {
                        format!("{}::{}", module_path.join("::"), type_name)
                    };
                    ir.register_schema_location(ir_key, module_path.to_vec(), type_name.clone());

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

            // Build the sub-module path
            let mut sub_path = module_path.to_vec();
            sub_path.push(submodule.name.clone());

            output.push_str(&submodule.generate_code(generated_types, indent + 1, ir, &sub_path));
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
            ir: CompleteIR::new(),
        }
    }

    /// Generate Rust code and return both the code and the IR
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

                        // Register in IR - top-level schemas are in the root
                        self.ir.register_schema_location(
                            name.clone(),
                            vec![],  // Empty path = top-level
                            name.clone()
                        );

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

        // Collect all endpoints first to avoid borrow checker issues
        let mut endpoints_to_process = Vec::new();
        if let Some(paths) = &self.spec.paths {
            for (path, path_item) in paths {
                let module_path = self.path_to_module_path(path);

                for (method, operation) in [
                    ("GET", &path_item.get),
                    ("POST", &path_item.post),
                    ("PUT", &path_item.put),
                    ("DELETE", &path_item.delete),
                    ("PATCH", &path_item.patch),
                ] {
                    if let Some(op) = operation {
                        endpoints_to_process.push((path.clone(), method.to_string(), op.clone(), module_path.clone()));
                    }
                }
            }
        }

        // Process all endpoints
        for (path, method, operation, module_path) in endpoints_to_process {
            self.process_endpoint(&path, &method, &operation, &mut root_modules, &module_path);
        }

        // Generate all root modules - now IR registration happens during generation
        for (_, module) in root_modules {
            output.push_str(&format!("pub mod {} {{\n", module.name));
            output.push_str("    use super::*;\n\n");

            // Pass IR and module path to allow registration during generation
            let module_path = vec![module.name.clone()];
            output.push_str(&module.generate_code(&mut self.generated_types, 1, &mut self.ir, &module_path));
            output.push_str("}\n\n");
        }

        // Clean up endpoint metadata: set request_type/response_type to None if type doesn't exist in IR
        // (IR now only contains types that were actually generated)
        let mut cleaned_endpoints = 0;
        for endpoint in &mut self.ir.endpoints {
            if let Some(ref req_type) = endpoint.request_type {
                if !self.ir.schema_locations.values().any(|loc| loc.full_path == req_type.full_path) {
                    endpoint.request_type = None;
                    cleaned_endpoints += 1;
                }
            }
            if let Some(ref resp_type) = endpoint.response_type {
                if !self.ir.schema_locations.values().any(|loc| loc.full_path == resp_type.full_path) {
                    endpoint.response_type = None;
                    cleaned_endpoints += 1;
                }
            }
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
        &mut self,
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
        target.types.push((type_name.clone(), schema.clone()));

        // Don't register in IR here - will be done during code generation if code is actually generated
    }

    fn process_endpoint(
        &mut self,
        path: &str,
        method: &str,
        operation: &Operation,
        root_modules: &mut BTreeMap<String, NestedModule>,
        module_path: &[String],
    ) {
        let type_name = self.method_to_type_name(method, path);
        let mut request_type: Option<TypeLocation> = None;
        let mut response_type: Option<TypeLocation> = None;

        // Extract request schema
        if let Some(request_body) = &operation.request_body {
            if let ObjectOrReference::Object(body) = request_body {
                if let Some(content) = body.content.get("application/json") {
                    match &content.schema {
                        Some(ObjectOrReference::Ref { ref_path, .. }) => {
                            // Referenced schema - look it up in IR
                            if let Some(schema_name) = ref_path.split('/').last() {
                                if let Some(loc) = self.ir.schema_locations.get(schema_name) {
                                    request_type = Some(loc.clone());
                                }
                            }
                        }
                        Some(ObjectOrReference::Object(schema)) => {
                            if !schema.properties.is_empty() || !schema.enum_values.is_empty() {
                                let request_name = format!("{}Request", type_name);
                                self.add_to_module_tree(root_modules, module_path, request_name.clone(), schema.clone());

                                // Only set request_type if the type was actually registered in IR
                                // (add_to_module_tree only registers if will_generate_code returns true)
                                let full_name = format!("{}::{}", module_path.join("::"), request_name);
                                if self.ir.schema_locations.contains_key(&full_name) {
                                    request_type = Some(TypeLocation::new(module_path.to_vec(), request_name));
                                }
                            }
                        }
                        None => {}
                    }
                }
            }
        }

        // Extract response schemas
        let mut response_status = "200".to_string();
        if let Some(responses) = &operation.responses {
            for (status_code, response) in responses {
                if let ObjectOrReference::Object(resp) = response {
                    if let Some(content) = resp.content.get("application/json") {
                        match &content.schema {
                            Some(ObjectOrReference::Ref { ref_path, .. }) => {
                                // Referenced schema - look it up in IR
                                if let Some(schema_name) = ref_path.split('/').last() {
                                    if let Some(loc) = self.ir.schema_locations.get(schema_name) {
                                        response_type = Some(loc.clone());
                                        response_status = status_code.clone();
                                        break;
                                    }
                                }
                            }
                            Some(ObjectOrReference::Object(schema)) => {
                                if !schema.properties.is_empty() || !schema.enum_values.is_empty() {
                                    let status_suffix = if status_code == "200" {
                                        String::new()
                                    } else {
                                        status_code.replace("XX", "").replace("\"", "")
                                    };
                                    let response_name = format!("{}Response{}", type_name, status_suffix);
                                    self.add_to_module_tree(root_modules, module_path, response_name.clone(), schema.clone());

                                    // Only set response_type if the type was actually registered in IR
                                    let full_name = format!("{}::{}", module_path.join("::"), response_name);
                                    if self.ir.schema_locations.contains_key(&full_name) {
                                        response_type = Some(TypeLocation::new(module_path.to_vec(), response_name));
                                        response_status = status_code.clone();
                                    }
                                    break;
                                }
                            }
                            None => {}
                        }
                    }
                }
            }
        }

        // Register endpoint in IR
        let endpoint_metadata = EndpointMetadata {
            method: method.to_uppercase(),
            path: path.to_string(),
            operation_id: operation.operation_id.clone(),
            request_type,
            response_type,
            response_status,
            path_params: vec![],  // TODO: extract from path
            query_params: vec![],  // TODO: extract from operation.parameters
            summary: operation.summary.clone(),
            description: operation.description.clone(),
            tags: if operation.tags.is_empty() { None } else { Some(operation.tags.clone()) },
        };

        self.ir.register_endpoint(endpoint_metadata);
    }

    /// Get the built IR
    pub fn get_ir(&self) -> &CompleteIR {
        &self.ir
    }

    /// Consume the generator and return the IR
    pub fn into_ir(self) -> CompleteIR {
        self.ir
    }
}
