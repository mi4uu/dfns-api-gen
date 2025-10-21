// Converts IR (Intermediate Representation) to Rust AST using quote! and syn
//
// This module takes structured IR types and converts them to proc_macro2::TokenStream
// using the quote! macro, then formats them with prettyplease for clean output.

use crate::ir::*;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

/// Convert a TypeDef to a TokenStream
pub fn generate_type_def(type_def: &TypeDef) -> TokenStream {
    match type_def {
        TypeDef::Struct(s) => generate_struct(s),
        TypeDef::Enum(e) => generate_enum(e),
        TypeDef::TypeAlias(t) => generate_type_alias(t),
    }
}

/// Generate a struct definition
pub fn generate_struct(def: &StructDef) -> TokenStream {
    let name = ident(&def.name);
    let doc = generate_doc_comment(&def.doc);
    let derives = generate_derives(&def.derives);
    let attributes = generate_attributes(&def.attributes);

    if def.is_tuple {
        // Tuple struct: pub struct Foo(String, i32);
        let field_types: Vec<_> = def.fields.iter().map(|f| {
            let ty = rust_type_to_tokens(&f.effective_type());
            let vis = visibility_to_tokens(&f.visibility);
            quote! { #vis #ty }
        }).collect();

        quote! {
            #doc
            #(#attributes)*
            #derives
            pub struct #name(#(#field_types),*);
        }
    } else {
        // Regular struct with named fields
        let fields = def.fields.iter().map(|f| generate_field(f));

        quote! {
            #doc
            #(#attributes)*
            #derives
            pub struct #name {
                #(#fields),*
            }
        }
    }
}

/// Generate a field definition
fn generate_field(field: &FieldDef) -> TokenStream {
    let name = ident(&field.name);
    let ty = rust_type_to_tokens(&field.effective_type());
    let doc = generate_doc_comment(&field.doc);
    let vis = visibility_to_tokens(&field.visibility);

    let mut attrs = Vec::new();

    // Add serde rename if needed
    if let Some(rename) = &field.serde_rename {
        attrs.push(quote! { #[serde(rename = #rename)] });
    }

    // Add skip_serializing_if for optional fields
    if field.skip_serializing_if_none && field.optional {
        attrs.push(quote! { #[serde(skip_serializing_if = "Option::is_none")] });
    }

    // Add custom attributes (as raw tokens for now)
    for attr in &field.attributes {
        if let Ok(tokens) = syn::parse_str::<TokenStream>(attr) {
            attrs.push(tokens);
        }
    }

    quote! {
        #doc
        #(#attrs)*
        #vis #name: #ty
    }
}

/// Generate an enum definition
pub fn generate_enum(def: &EnumDef) -> TokenStream {
    let name = ident(&def.name);
    let doc = generate_doc_comment(&def.doc);
    let derives = generate_derives(&def.derives);
    let attributes = generate_attributes(&def.attributes);
    let representation = generate_enum_representation(&def.representation);

    let variants = def.variants.iter().map(|v| generate_variant(v));

    quote! {
        #doc
        #(#attributes)*
        #derives
        #representation
        pub enum #name {
            #(#variants),*
        }
    }
}

/// Generate an enum variant
fn generate_variant(variant: &VariantDef) -> TokenStream {
    let name = ident(&variant.name);
    let doc = generate_doc_comment(&variant.doc);

    let mut attrs = Vec::new();

    // Add serde rename if needed
    if let Some(rename) = &variant.serde_rename {
        attrs.push(quote! { #[serde(rename = #rename)] });
    }

    // Add default marker if needed
    if variant.is_default {
        attrs.push(quote! { #[default] });
    }

    // Add custom attributes (as raw tokens for now)
    for attr in &variant.attributes {
        if let Ok(tokens) = syn::parse_str::<TokenStream>(attr) {
            attrs.push(tokens);
        }
    }

    let variant_data = match &variant.data {
        VariantData::Unit => {
            quote! {}
        }
        VariantData::Tuple(types) => {
            let type_tokens: Vec<_> = types.iter().map(rust_type_to_tokens).collect();
            quote! { (#(#type_tokens),*) }
        }
        VariantData::Struct(fields) => {
            let field_tokens = fields.iter().map(generate_field);
            quote! { { #(#field_tokens),* } }
        }
    };

    quote! {
        #doc
        #(#attrs)*
        #name #variant_data
    }
}

/// Generate enum representation attribute
fn generate_enum_representation(repr: &EnumRepresentation) -> TokenStream {
    match repr {
        EnumRepresentation::External => quote! {},
        EnumRepresentation::Untagged => quote! { #[serde(untagged)] },
        EnumRepresentation::InternallyTagged { tag } => {
            quote! { #[serde(tag = #tag)] }
        }
        EnumRepresentation::AdjacentlyTagged { tag, content } => {
            quote! { #[serde(tag = #tag, content = #content)] }
        }
    }
}

/// Generate a type alias
pub fn generate_type_alias(def: &TypeAliasDef) -> TokenStream {
    let name = ident(&def.name);
    let doc = generate_doc_comment(&def.doc);
    let target = rust_type_to_tokens(&def.target);
    let vis = visibility_to_tokens(&def.visibility);

    quote! {
        #doc
        #vis type #name = #target;
    }
}

/// Convert RustType to TokenStream
pub fn rust_type_to_tokens(ty: &RustType) -> TokenStream {
    match ty {
        RustType::String => quote! { String },
        RustType::I32 => quote! { i32 },
        RustType::I64 => quote! { i64 },
        RustType::U32 => quote! { u32 },
        RustType::U64 => quote! { u64 },
        RustType::F32 => quote! { f32 },
        RustType::F64 => quote! { f64 },
        RustType::Bool => quote! { bool },
        RustType::Vec(inner) => {
            let inner_ty = rust_type_to_tokens(inner);
            quote! { Vec<#inner_ty> }
        }
        RustType::Option(inner) => {
            let inner_ty = rust_type_to_tokens(inner);
            quote! { Option<#inner_ty> }
        }
        RustType::HashMap(key, value) => {
            let key_ty = rust_type_to_tokens(key);
            let value_ty = rust_type_to_tokens(value);
            quote! { std::collections::HashMap<#key_ty, #value_ty> }
        }
        RustType::HashSet(inner) => {
            let inner_ty = rust_type_to_tokens(inner);
            quote! { std::collections::HashSet<#inner_ty> }
        }
        RustType::Box(inner) => {
            let inner_ty = rust_type_to_tokens(inner);
            quote! { Box<#inner_ty> }
        }
        RustType::Named(name) => {
            // Parse the name as a path (handles "std::path::PathBuf" or "MyType")
            if let Ok(path) = syn::parse_str::<syn::Path>(name) {
                quote! { #path }
            } else {
                // Fallback to simple ident if parsing fails
                let ident = ident(name);
                quote! { #ident }
            }
        }
        RustType::JsonValue => quote! { serde_json::Value },
        RustType::Tuple(types) => {
            let type_tokens: Vec<_> = types.iter().map(rust_type_to_tokens).collect();
            quote! { (#(#type_tokens),*) }
        }
        RustType::Unit => quote! { () },
        RustType::Reference { mutable, inner } => {
            let inner_ty = rust_type_to_tokens(inner);
            if *mutable {
                quote! { &mut #inner_ty }
            } else {
                quote! { &#inner_ty }
            }
        }
    }
}

/// Generate derives
fn generate_derives(derives: &[String]) -> TokenStream {
    if derives.is_empty() {
        return quote! {};
    }

    let derive_idents: Vec<_> = derives
        .iter()
        .map(|d| {
            // Handle derives with paths like "utoipa::ToSchema"
            if let Ok(path) = syn::parse_str::<syn::Path>(d) {
                quote! { #path }
            } else {
                let ident = ident(d);
                quote! { #ident }
            }
        })
        .collect();

    quote! {
        #[derive(#(#derive_idents),*)]
    }
}

/// Generate attributes
fn generate_attributes(attributes: &[String]) -> Vec<TokenStream> {
    attributes
        .iter()
        .filter_map(|attr| syn::parse_str::<TokenStream>(attr).ok())
        .collect()
}

/// Generate documentation comment
fn generate_doc_comment(doc: &Option<String>) -> TokenStream {
    match doc {
        Some(text) => {
            // Split multi-line docs
            let lines: Vec<_> = text.lines().collect();
            quote! {
                #(#[doc = #lines])*
            }
        }
        None => quote! {},
    }
}

/// Convert Visibility to TokenStream
fn visibility_to_tokens(vis: &Visibility) -> TokenStream {
    match vis {
        Visibility::Public => quote! { pub },
        Visibility::PublicCrate => quote! { pub(crate) },
        Visibility::Private => quote! {},
    }
}

/// Create an identifier from a string
fn ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}

/// Generate a complete module (with nested submodules)
pub fn generate_module(module: &ModuleDef) -> TokenStream {
    let name = ident(&module.name);
    let doc = generate_doc_comment(&module.doc);
    let vis = visibility_to_tokens(&module.visibility);

    let types = module.types.iter().map(generate_type_def);
    let submodules = module.submodules.iter().map(generate_module);

    quote! {
        #doc
        #vis mod #name {
            use super::*;

            #(#types)*

            #(#submodules)*
        }
    }
}

/// Generate complete code output
pub fn generate_code_output(output: &CodeGenOutput) -> String {
    let mut tokens = TokenStream::new();

    // Add file attributes (as comments or pragmas)
    for attr in &output.file_attributes {
        if attr.starts_with("//") || attr.starts_with("#!") {
            // Parse as raw tokens
            if let Ok(parsed) = syn::parse_str::<TokenStream>(attr) {
                tokens.extend(parsed);
            }
        }
    }

    // Add imports
    for import in &output.imports {
        if let Ok(parsed) = syn::parse_str::<syn::ItemUse>(import) {
            tokens.extend(quote! { #parsed });
        }
    }

    // Add top-level types
    for type_def in &output.types {
        tokens.extend(generate_type_def(type_def));
    }

    // Add modules
    for module in &output.modules {
        tokens.extend(generate_module(module));
    }

    // Format with prettyplease
    format_tokens(tokens)
}

/// Format TokenStream using prettyplease
pub fn format_tokens(tokens: TokenStream) -> String {
    match syn::parse2::<syn::File>(tokens.clone()) {
        Ok(ast) => prettyplease::unparse(&ast),
        Err(e) => {
            // If parsing fails, return the unformatted tokens with error message
            format!(
                "// Error formatting code: {}\n// Raw tokens:\n{}",
                e,
                tokens
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_simple_struct() {
        let struct_def = StructDef::new("User")
            .with_field(FieldDef::new("id", RustType::String))
            .with_field(FieldDef::new("name", RustType::String).optional());

        let tokens = generate_struct(&struct_def);
        let code = format_tokens(tokens);

        assert!(code.contains("pub struct User"));
        assert!(code.contains("pub id: String"));
        assert!(code.contains("pub name: Option<String>"));
    }

    #[test]
    fn test_generate_enum() {
        let enum_def = EnumDef::new("Status")
            .with_variant(VariantDef::unit("Active").as_default())
            .with_variant(VariantDef::unit("Inactive"))
            .with_variant(VariantDef::tuple("Suspended", RustType::String));

        let tokens = generate_enum(&enum_def);
        let code = format_tokens(tokens);

        assert!(code.contains("pub enum Status"));
        assert!(code.contains("Active"));
        assert!(code.contains("Suspended(String)"));
    }

    #[test]
    fn test_rust_type_to_tokens() {
        let ty = RustType::Vec(Box::new(RustType::String));
        let tokens = rust_type_to_tokens(&ty);
        let code = tokens.to_string();
        assert!(code.contains("Vec"));
        assert!(code.contains("String"));
    }

    #[test]
    fn test_generate_with_serde_rename() {
        let struct_def = StructDef::new("User")
            .with_field(
                FieldDef::new("user_id", RustType::String)
                    .with_serde_rename("userId")
            );

        let tokens = generate_struct(&struct_def);
        let code = format_tokens(tokens);

        assert!(code.contains("serde(rename = \"userId\")"));
        assert!(code.contains("pub user_id: String"));
    }

    #[test]
    fn test_complete_code_output() {
        let output = CodeGenOutput::new()
            .with_type(TypeDef::Struct(
                StructDef::new("Foo")
                    .with_field(FieldDef::new("bar", RustType::I32))
            ));

        let code = generate_code_output(&output);

        assert!(code.contains("use serde"));
        assert!(code.contains("pub struct Foo"));
        assert!(code.contains("pub bar: i32"));
    }
}
