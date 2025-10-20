use crate::codegen::utils::{sanitize_field_name, to_pascal_case, to_snake_case};
use oas3::spec::{MediaTypeExamples, ObjectOrReference, Operation, Spec};
use std::collections::BTreeMap;

pub struct EndpointGenerator {
    spec: Spec,
}

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub path: String,
    pub method: String,
    pub operation_id: Option<String>,
    pub handler_name: String,
    pub request_type: Option<String>,
    pub response_type: Option<String>,
    pub response_status: String,
    pub response_example: Option<serde_json::Value>,
    pub module_name: String,
}

impl EndpointGenerator {
    pub fn new(spec: Spec) -> Self {
        Self { spec }
    }

    pub fn generate(&self) -> String {
        let endpoints = self.extract_endpoints();
        let mut output = String::new();

        // Generate header
        output.push_str("// Auto-generated API endpoints\n");
        output.push_str("// Do not edit manually\n\n");
        output.push_str("use axum::{extract::Path as AxumPath, http::StatusCode, Json};\n");
        output.push_str("use crate::generated;\n");
        output.push_str("use utoipa::OpenApi;\n");
        output.push_str("use utoipa_axum::{router::OpenApiRouter, routes};\n\n");

        // Generate OpenAPI struct
        output.push_str("#[derive(OpenApi)]\n");
        output.push_str("#[openapi(\n");
        output.push_str("    info(title = \"Dfns API\", version = \"1.0.0\"),\n");
        output.push_str("    paths(\n");

        for (i, endpoint) in endpoints.iter().enumerate() {
            output.push_str(&format!("        {}", endpoint.handler_name));
            if i < endpoints.len() - 1 {
                output.push_str(",\n");
            } else {
                output.push_str("\n");
            }
        }

        output.push_str("    )\n");
        output.push_str(")]\n");
        output.push_str("pub struct ApiDoc;\n\n");

        // Generate router setup
        output.push_str("impl ApiDoc {\n");
        output.push_str("    pub fn router() -> OpenApiRouter {\n");
        output.push_str("        OpenApiRouter::with_openapi(ApiDoc::openapi())\n");

        for endpoint in &endpoints {
            let method_lower = endpoint.method.to_lowercase();
            output.push_str(&format!(
                "            .routes(routes!({}))\n",
                endpoint.handler_name
            ));
        }

        output.push_str("    }\n");
        output.push_str("}\n\n");

        // Generate handler functions
        for endpoint in &endpoints {
            output.push_str(&self.generate_handler(&endpoint));
            output.push_str("\n\n");
        }

        output
    }

    fn extract_endpoints(&self) -> Vec<Endpoint> {
        let mut endpoints = Vec::new();

        if let Some(paths) = &self.spec.paths {
            for (path, path_item) in paths {
                // Process each HTTP method
                if let Some(op) = &path_item.get {
                    endpoints.push(self.create_endpoint(path, "GET", op));
                }
                if let Some(op) = &path_item.post {
                    endpoints.push(self.create_endpoint(path, "POST", op));
                }
                if let Some(op) = &path_item.put {
                    endpoints.push(self.create_endpoint(path, "PUT", op));
                }
                if let Some(op) = &path_item.delete {
                    endpoints.push(self.create_endpoint(path, "DELETE", op));
                }
                if let Some(op) = &path_item.patch {
                    endpoints.push(self.create_endpoint(path, "PATCH", op));
                }
            }
        }

        endpoints
    }

    fn create_endpoint(&self, path: &str, method: &str, operation: &Operation) -> Endpoint {
        let (module_name, path_name) = self.path_to_mod_and_name(path);

        // Always include module name to avoid collisions, then add path_name if present
        let handler_name = if path_name.is_empty() {
            format!("{}_{}", method.to_lowercase(), module_name)
        } else {
            format!(
                "{}_{}_{}",
                method.to_lowercase(),
                module_name,
                to_snake_case(&path_name)
            )
        };

        // Get request type
        let request_type = self.get_request_type(path, &path_name, method, operation);

        // Get response type and example
        let (response_type, response_status, response_example) =
            self.get_response_info(path, &path_name, method, operation);

        Endpoint {
            path: path.to_string(),
            method: method.to_string(),
            operation_id: operation.operation_id.clone(),
            handler_name,
            request_type,
            response_type,
            response_status,
            response_example,
            module_name,
        }
    }

    fn get_request_type(
        &self,
        path: &str,
        path_name: &str,
        method: &str,
        operation: &Operation,
    ) -> Option<String> {
        if let Some(request_body) = &operation.request_body {
            match request_body {
                ObjectOrReference::Object(body) => {
                    if let Some(content) = body.content.get("application/json") {
                        if let Some(schema) = &content.schema {
                            match schema {
                                ObjectOrReference::Ref { ref_path, .. } => {
                                    // Use the referenced component schema directly
                                    if let Some(schema_name) = ref_path.split('/').last() {
                                        return Some(format!("generated::{}", schema_name));
                                    } else {
                                        return Some("serde_json::Value".to_string());
                                    }
                                }
                                ObjectOrReference::Object(obj_schema) => {
                                    // Check if this is a simple type or complex type that won't be generated
                                    if obj_schema.properties.is_empty()
                                        && obj_schema.enum_values.is_empty()
                                    {
                                        // Simple type (string, array, etc.) or complex type (oneOf, allOf)
                                        // Use serde_json::Value as fallback
                                        return Some("serde_json::Value".to_string());
                                    } else {
                                        // Inline schema with properties - should have a generated type
                                        let module = self.path_to_mod_and_name(path).0;
                                        let type_name = if path_name.is_empty() {
                                            // Use module name when path_name is empty
                                            format!(
                                                "{}{}Request",
                                                to_pascal_case(&module),
                                                to_pascal_case(method)
                                            )
                                        } else {
                                            format!(
                                                "{}{}Request",
                                                path_name,
                                                to_pascal_case(method)
                                            )
                                        };
                                        return Some(format!(
                                            "generated::{}::{}",
                                            module, type_name
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
                ObjectOrReference::Ref { ref_path, .. } => {
                    let type_name = ref_path.split('/').last().unwrap_or("Unknown");
                    return Some(format!("generated::{}", type_name));
                }
            }
        }
        None
    }

    fn get_response_info(
        &self,
        path: &str,
        path_name: &str,
        method: &str,
        operation: &Operation,
    ) -> (Option<String>, String, Option<serde_json::Value>) {
        if let Some(responses) = &operation.responses {
            // Try to find 200, 201, or first success response
            for status in ["200", "201", "204", "2XX"] {
                if let Some(response) = responses.get(status) {
                    match response {
                        ObjectOrReference::Object(resp) => {
                            if let Some(content) = resp.content.get("application/json") {
                                // First try to get example from media type examples
                                let mut example =
                                    content.examples.as_ref().and_then(|media_examples| {
                                        match media_examples {
                                            MediaTypeExamples::Examples { examples } => {
                                                examples.values().next().and_then(|ex| match ex {
                                                    ObjectOrReference::Object(ex_obj) => {
                                                        ex_obj.value.clone()
                                                    }
                                                    _ => None,
                                                })
                                            }
                                            MediaTypeExamples::Example { example } => {
                                                Some(example.clone())
                                            }
                                        }
                                    });

                                // If no media type example, try to get example from schema
                                if example.is_none() {
                                    example = self.extract_example_from_schema(&content.schema);
                                }

                                let status_name = status.replace("XX", "").replace("\"", "");
                                let status_name = match status_name.as_str() {
                                    "200" => String::new(),
                                    _ => status_name,
                                };

                                // Determine the type to use
                                let full_type = if let Some(schema) = &content.schema {
                                    match schema {
                                        ObjectOrReference::Ref { ref_path, .. } => {
                                            // Use the referenced component schema directly
                                            if let Some(schema_name) = ref_path.split('/').last() {
                                                format!("generated::{}", schema_name)
                                            } else {
                                                // Fallback to serde_json::Value
                                                "serde_json::Value".to_string()
                                            }
                                        }
                                        ObjectOrReference::Object(obj_schema) => {
                                            // Check if this is a simple type or complex type that won't be generated
                                            if obj_schema.properties.is_empty()
                                                && obj_schema.enum_values.is_empty()
                                            {
                                                // Simple type (string, array, etc.) or complex type (oneOf, allOf)
                                                // Use serde_json::Value as fallback
                                                "serde_json::Value".to_string()
                                            } else {
                                                // Inline schema with properties - should have a generated type
                                                let module = self.path_to_mod_and_name(path).0;
                                                let type_name = if path_name.is_empty() {
                                                    // Use module name when path_name is empty
                                                    format!(
                                                        "{}{}Response{}",
                                                        to_pascal_case(&module),
                                                        to_pascal_case(method),
                                                        status_name
                                                    )
                                                } else {
                                                    format!(
                                                        "{}{}Response{}",
                                                        path_name,
                                                        to_pascal_case(method),
                                                        status_name
                                                    )
                                                };
                                                format!("generated::{}::{}", module, type_name)
                                            }
                                        }
                                    }
                                } else {
                                    // No schema - use serde_json::Value
                                    "serde_json::Value".to_string()
                                };

                                return (Some(full_type), status.to_string(), example);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        (None, "200".to_string(), None)
    }

    fn extract_example_from_schema(
        &self,
        schema_ref: &Option<ObjectOrReference<oas3::spec::ObjectSchema>>,
    ) -> Option<serde_json::Value> {
        if let Some(schema) = schema_ref {
            match schema {
                ObjectOrReference::Ref { ref_path, .. } => {
                    // Extract schema name from ref path like "#/components/schemas/Wallet"
                    if let Some(schema_name) = ref_path.split('/').last() {
                        // Look up the schema in components
                        if let Some(components) = &self.spec.components {
                            if let Some(schema_obj) = components.schemas.get(schema_name) {
                                match schema_obj {
                                    ObjectOrReference::Object(s) => {
                                        return s.example.clone();
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                ObjectOrReference::Object(s) => {
                    return s.example.clone();
                }
            }
        }
        None
    }

    fn generate_handler(&self, endpoint: &Endpoint) -> String {
        let mut output = String::new();

        // Extract path parameters
        let path_params: Vec<String> = endpoint
            .path
            .split('/')
            .filter(|s| s.starts_with('{') && s.ends_with('}'))
            .map(|s| s.trim_start_matches('{').trim_end_matches('}').to_string())
            .collect();

        // Build utoipa path annotation
        output.push_str("#[utoipa::path(\n");
        output.push_str(&format!("    {},\n", endpoint.method.to_lowercase()));
        output.push_str(&format!("    path = \"{}\",\n", endpoint.path));

        // Add path parameters
        if !path_params.is_empty() {
            output.push_str("    params(\n");
            for (i, param) in path_params.iter().enumerate() {
                output.push_str(&format!(
                    "        (\"{}\" = String, Path, description = \"{}\")",
                    param, param
                ));
                if i < path_params.len() - 1 {
                    output.push_str(",\n");
                } else {
                    output.push_str("\n");
                }
            }
            output.push_str("    ),\n");
        }

        // Add request body - use actual type if available
        if let Some(ref req_type) = endpoint.request_type {
            output.push_str(&format!("    request_body = {},\n", req_type));
        }

        // Add responses - use actual type if available
        output.push_str("    responses(\n");
        if let Some(ref resp_type) = endpoint.response_type {
            output.push_str(&format!(
                "        (status = {}, body = {})\n",
                endpoint.response_status, resp_type
            ));
        } else {
            output.push_str(&format!(
                "        (status = {})\n",
                endpoint.response_status
            ));
        }
        output.push_str("    )\n");
        output.push_str(")]\n");

        // Generate function signature
        output.push_str(&format!("pub async fn {}(\n", endpoint.handler_name));

        // Add path parameters
        if !path_params.is_empty() {
            let param_tuple = if path_params.len() == 1 {
                path_params[0].clone()
            } else {
                format!("({})", path_params.join(", "))
            };
            output.push_str(&format!("    AxumPath({}): AxumPath<", param_tuple));
            if path_params.len() == 1 {
                output.push_str("String");
            } else {
                output.push_str(&format!(
                    "({})",
                    vec!["String"; path_params.len()].join(", ")
                ));
            }
            output.push_str(">,\n");
        }

        // Add request body parameter - use actual type if available
        if let Some(ref req_type) = endpoint.request_type {
            output.push_str(&format!("    Json(request): Json<{}>,\n", req_type));
        }

        output.push_str(")");

        // Return type - use actual type if available
        if let Some(ref resp_type) = endpoint.response_type {
            output.push_str(&format!(" -> Json<{}>", resp_type));
        } else {
            output.push_str(" -> StatusCode");
        }

        output.push_str(" {\n");

        // Generate mock response body
        if let Some(ref resp_type) = endpoint.response_type {
            output.push_str("    // TODO: Replace with actual implementation\n");

            if resp_type == "serde_json::Value" {
                // For serde_json::Value, just return the JSON directly
                if let Some(ref example) = endpoint.response_example {
                    let example_json =
                        serde_json::to_string_pretty(example).unwrap_or_else(|_| "{}".to_string());
                    output.push_str("    Json(serde_json::json!(\n");
                    output.push_str(
                        &example_json
                            .lines()
                            .map(|l| format!("        {}", l))
                            .collect::<Vec<_>>()
                            .join("\n"),
                    );
                    output.push_str("\n    ))\n");
                } else {
                    output.push_str("    Json(serde_json::json!({}))\n");
                }
            } else {
                // For typed responses, deserialize from example or use default
                if let Some(ref example) = endpoint.response_example {
                    let example_json =
                        serde_json::to_string_pretty(example).unwrap_or_else(|_| "{}".to_string());
                    output.push_str("    let example_json = serde_json::json!(\n");
                    output.push_str(
                        &example_json
                            .lines()
                            .map(|l| format!("        {}", l))
                            .collect::<Vec<_>>()
                            .join("\n"),
                    );
                    output.push_str("\n    );\n");
                    output.push_str(&format!(
                        "    let response: {} = serde_json::from_value(example_json)\n",
                        resp_type
                    ));
                    output.push_str(
                        "        .expect(\"Failed to deserialize example into response type\");\n",
                    );
                    output.push_str("    Json(response)\n");
                } else {
                    // Use Default trait to create an instance
                    output.push_str(&format!("    Json({}::default())\n", resp_type));
                }
            }
        } else {
            output.push_str("    // TODO: Replace with actual implementation\n");
            output.push_str(&format!(
                "    StatusCode::{}\n",
                if endpoint.response_status == "204" {
                    "NO_CONTENT"
                } else {
                    "OK"
                }
            ));
        }

        output.push_str("}");
        output
    }

    fn path_to_mod_and_name(&self, path: &str) -> (String, String) {
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

        let mod_name = parts[0].to_lowercase();
        let type_name_prefix = parts[1..].join("");

        (mod_name, type_name_prefix)
    }
}
