# Code Generation Architecture Analysis

## Current State Problems

Your current implementation works but has quality issues:
- **String concatenation approach**: Building code via `output.push_str()` is error-prone and hard to maintain
- **Lack of structure**: No intermediate representation (IR) between OpenAPI and Rust code
- **Limited validation**: Hard to validate correctness of generated code structure
- **Formatting concerns**: Manual indentation management throughout the codebase
- **Testing difficulties**: Can't easily unit test parts of the generation pipeline

## Proposed Solutions

### Option 1: **quote! + syn + prettyplease** (RECOMMENDED)

#### Overview
Use Rust's official proc-macro ecosystem to generate code as an Abstract Syntax Tree (AST), then pretty-print it.

#### Architecture
```
OpenAPI Schema → IR (your structs) → AST (syn types) → TokenStream (quote!) → Formatted Code (prettyplease)
```

#### Implementation Flow
1. **Parse OpenAPI** with `oas3` (current)
2. **Create IR**: Define your own serde-serializable structs that represent ONLY what you need:
   ```rust
   #[derive(Debug, Clone, Serialize, Deserialize)]
   struct StructDef {
       name: String,
       fields: Vec<FieldDef>,
       derives: Vec<String>,
       doc_comment: Option<String>,
   }

   #[derive(Debug, Clone, Serialize, Deserialize)]
   struct FieldDef {
       name: String,
       rust_type: RustType,
       serde_rename: Option<String>,
       optional: bool,
       doc_comment: Option<String>,
   }
   ```

3. **Transform IR to AST**: Use `quote!` and `syn` to build proper Rust syntax trees:
   ```rust
   fn generate_struct(def: &StructDef) -> proc_macro2::TokenStream {
       let name = syn::Ident::new(&def.name, proc_macro2::Span::call_site());
       let fields = def.fields.iter().map(|f| generate_field(f));

       quote! {
           #[derive(Debug, Clone, Serialize, Deserialize)]
           pub struct #name {
               #(#fields),*
           }
       }
   }
   ```

4. **Format output**: Convert TokenStream → AST → Pretty-printed code:
   ```rust
   let tokens = generate_struct(&struct_def);
   let ast: syn::File = syn::parse2(tokens).unwrap();
   let code = prettyplease::unparse(&ast);
   ```

#### Pros
- ✅ **Industry standard**: Used by progenitor, typify, and many other successful projects
- ✅ **Type-safe**: Generate Rust code using Rust's own type system
- ✅ **Validation**: Syntax errors caught at AST construction time, not runtime
- ✅ **Clean separation**: Clear pipeline stages (OpenAPI → IR → AST → Code)
- ✅ **Testable**: Each transformation step can be unit tested independently
- ✅ **Automatic formatting**: No manual indentation or string manipulation
- ✅ **Maintainable**: Functions that work with structured data, not string fragments
- ✅ **No new dependencies on templates**: Pure Rust solution

#### Cons
- ⚠️ Learning curve for `quote!` macro and `syn` types
- ⚠️ More upfront work to restructure current code
- ⚠️ TokenStream debugging can be tricky initially

#### Key Crates
```toml
[dependencies]
syn = { version = "2", features = ["full"] }
quote = "1"
proc-macro2 = "1"
prettyplease = "0.2"
```

---

### Option 2: **typify / progenitor** (Use Existing Tool)

#### Overview
Instead of building your own, use Oxide Computer's battle-tested tools that already solve this problem.

#### How It Works
- **typify**: Converts JSON Schema (OpenAPI component schemas) → Rust types
- **progenitor**: Full OpenAPI → Rust client generator (uses typify internally)

#### Architecture
```
OpenAPI Spec → typify (types) + progenitor (client) → Complete Rust SDK
```

#### Implementation
```rust
// In build.rs or as a macro
use progenitor::GenerationSettings;

let spec = oas3::from_path("openapi.json")?;
let settings = GenerationSettings::default()
    .with_interface_style(InterfaceStyle::Builder)
    .with_derives(vec!["utoipa::ToSchema"]);

let code = progenitor::Generator::new(&settings)
    .generate_tokens(&spec)?;
```

#### Pros
- ✅ **Zero implementation**: Already handles all edge cases
- ✅ **Production-ready**: Used by Oxide Computer in production
- ✅ **Full client generation**: Includes HTTP client, pagination, error handling
- ✅ **Modern patterns**: Async/await, streams, builder pattern
- ✅ **Well-tested**: Years of real-world usage
- ✅ **Active maintenance**: Regular updates and bug fixes

#### Cons
- ⚠️ **Less control**: Harder to customize generation logic
- ⚠️ **OpenAPI 3.0 only**: May not support all OpenAPI 3.1 features yet
- ⚠️ **Opinionated**: Designed for Dropshot APIs, may not match your exact needs
- ⚠️ **Learning overhead**: Need to understand their API surface

#### When to Choose This
- You want a complete client SDK, not just types
- You're okay with some opinionated choices
- You want to focus on business logic, not code generation

---

### Option 3: **MiniJinja Templates**

#### Overview
Use a template engine to generate code from templates, similar to how Django/Jinja2 work.

#### Architecture
```
OpenAPI Schema → IR (your structs) → MiniJinja Templates → Generated Code → rustfmt
```

#### Implementation Flow
1. **Create IR** (same as Option 1)
2. **Define templates** for each code structure:
   ```jinja
   {# struct.jinja #}
   {% if doc_comment %}
   /// {{ doc_comment }}
   {% endif %}
   #[derive({{ derives|join(", ") }})]
   pub struct {{ name }} {
       {% for field in fields %}
       {% if field.doc_comment %}
       /// {{ field.doc_comment }}
       {% endif %}
       {% if field.serde_rename %}
       #[serde(rename = "{{ field.serde_rename }}")]
       {% endif %}
       pub {{ field.name }}: {{ field.rust_type }},
       {% endfor %}
   }
   ```

3. **Render templates**:
   ```rust
   let mut env = minijinja::Environment::new();
   env.add_template("struct", include_str!("templates/struct.jinja"))?;

   let code = env.get_template("struct")?.render(context! {
       name => "MyStruct",
       fields => fields,
       derives => vec!["Debug", "Clone"],
   })?;
   ```

4. **Format with rustfmt** or prettyplease

#### Pros
- ✅ **Visual clarity**: Easy to see what generated code looks like
- ✅ **Designer-friendly**: Non-Rust devs can modify templates
- ✅ **Flexible**: Can generate any text format (not just Rust)
- ✅ **Fast iteration**: Change template, regenerate immediately
- ✅ **Familiar**: Many devs know Jinja2 syntax

#### Cons
- ⚠️ **No compile-time validation**: Template errors found at runtime
- ⚠️ **String-based**: Still fundamentally text manipulation
- ⚠️ **Requires formatting pass**: Need rustfmt or prettyplease after generation
- ⚠️ **Harder to test**: Template logic not as testable as functions
- ⚠️ **Indentation management**: Still manual in templates
- ⚠️ **Type safety**: No guarantee generated code compiles

#### When to Choose This
- You need to generate multiple languages/formats
- Your team prefers declarative template-driven approaches
- You want non-Rust developers to contribute to code generation

---

### Option 4: **serde-reflection**

#### Overview
Use serde-reflection to extract format specifications from Rust types, then use serde-generate for code generation.

#### Architecture
```
OpenAPI Schema → Rust IR Types → serde-reflection (extract formats) → serde-generate (output)
```

#### How It Works
```rust
// 1. Define your IR types with serde
#[derive(Serialize, Deserialize)]
struct WalletRequest {
    network: String,
    name: Option<String>,
}

// 2. Extract format
use serde_reflection::{Tracer, Samples};
let mut tracer = Tracer::new();
let samples = Samples::new();
tracer.trace_type::<WalletRequest>(&samples)?;
let registry = tracer.registry()?;

// 3. Generate code (supports Rust, Python, C++, Java, etc.)
use serde_generate::{rust, CodeGeneratorConfig};
let config = CodeGeneratorConfig::new("my_crate".to_string());
let generator = rust::CodeGenerator::new(&config);
generator.output(&mut std::io::stdout(), &registry)?;
```

#### Pros
- ✅ **Cross-language**: Generate types for Python, C++, Java, TypeScript, etc.
- ✅ **Format validation**: Can version-control schemas and detect breaking changes
- ✅ **Protocol buffers-like**: Similar to protobuf workflow but using Serde
- ✅ **Well-tested**: Built by Meta (formerly Facebook) for Diem/Libra

#### Cons
- ⚠️ **Wrong direction**: Works from Rust → other languages, not OpenAPI → Rust
- ⚠️ **Indirect**: Would need to define IR types manually, then extract formats
- ⚠️ **Limited customization**: Generated code style is opinionated
- ⚠️ **Overhead**: Extra step (Rust types → format → Rust types) is circular
- ⚠️ **Maintained by fork**: Original repo abandoned by Meta, now maintained by Zefchain

#### When to Choose This
- You need cross-language type compatibility (Rust ↔ Python/Java/etc.)
- You want to version-control serialization formats
- You're building an RPC system or protocol

#### Why NOT for Your Use Case
❌ This tool is designed for **extracting** formats from existing Rust types and generating **other languages** from them. You want to go from OpenAPI → Rust, which is the opposite direction. This would add unnecessary complexity.

---

## Recommendation: **Option 1 (quote + syn + prettyplease)**

### Why This is Best for You

1. **Solves your core problem**: "code is generated but not highest quality and messy"
   - Structured approach replaces string concatenation
   - Compile-time validation ensures correctness
   - Automatic formatting eliminates manual indentation

2. **Aligns with your goals**:
   - ✅ Build intermediate representation with only what you need
   - ✅ Transform IR in organized manner with dedicated functions
   - ✅ Better code quality and maintainability
   - ✅ Eventually build full client with request signing, session handling

3. **Industry proven**:
   - Used by `progenitor`, `typify`, `cargo-expand`, and many others
   - Standard approach for Rust code generation
   - Large community and resources

4. **Incremental migration**:
   - Can refactor module by module
   - Keep existing `oas3` parsing
   - Introduce IR types gradually
   - Replace string building with `quote!` incrementally

### Recommended Implementation Plan

#### Phase 1: Define IR Types (Week 1)
```rust
// src/ir/mod.rs
pub mod types;
pub mod schema;
pub mod endpoint;

// src/ir/types.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeDef {
    Struct(StructDef),
    Enum(EnumDef),
    TypeAlias(TypeAliasDef),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructDef {
    pub name: String,
    pub doc: Option<String>,
    pub derives: Vec<String>,
    pub fields: Vec<FieldDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDef {
    pub name: String,
    pub ty: RustType,
    pub doc: Option<String>,
    pub serde_rename: Option<String>,
    pub optional: bool,
    pub skip_serializing_if_none: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RustType {
    String,
    I32, I64,
    F32, F64,
    Bool,
    Vec(Box<RustType>),
    Option(Box<RustType>),
    HashMap(Box<RustType>, Box<RustType>),
    Named(String), // For references and custom types
    JsonValue,
}
```

#### Phase 2: Transform OpenAPI → IR (Week 1-2)
```rust
// src/codegen/openapi_to_ir.rs
pub struct OpenApiToIr {
    spec: Spec,
}

impl OpenApiToIr {
    pub fn convert(&self) -> Vec<TypeDef> {
        let mut types = Vec::new();

        // Convert component schemas
        if let Some(components) = &self.spec.components {
            for (name, schema) in &components.schemas {
                types.push(self.schema_to_type_def(name, schema));
            }
        }

        // Convert inline schemas from paths
        types.extend(self.extract_path_types());

        types
    }

    fn schema_to_type_def(&self, name: &str, schema: &ObjectOrReference<ObjectSchema>) -> TypeDef {
        // Your existing logic, but output IR instead of strings
    }
}
```

#### Phase 3: IR → AST with quote! (Week 2)
```rust
// src/codegen/ir_to_ast.rs
use quote::quote;
use syn::{Ident, Type};
use proc_macro2::{TokenStream, Span};

pub fn generate_type_def(def: &TypeDef) -> TokenStream {
    match def {
        TypeDef::Struct(s) => generate_struct(s),
        TypeDef::Enum(e) => generate_enum(e),
        TypeDef::TypeAlias(t) => generate_type_alias(t),
    }
}

fn generate_struct(def: &StructDef) -> TokenStream {
    let name = Ident::new(&def.name, Span::call_site());
    let doc = def.doc.as_ref().map(|d| quote! { #[doc = #d] });

    let derives = def.derives.iter().map(|d| {
        let ident = Ident::new(d, Span::call_site());
        quote! { #ident }
    });

    let fields = def.fields.iter().map(|f| generate_field(f));

    quote! {
        #doc
        #[derive(#(#derives),*)]
        pub struct #name {
            #(#fields),*
        }
    }
}

fn generate_field(field: &FieldDef) -> TokenStream {
    let name = Ident::new(&field.name, Span::call_site());
    let ty = rust_type_to_syn(&field.ty);
    let doc = field.doc.as_ref().map(|d| quote! { #[doc = #d] });
    let rename = field.serde_rename.as_ref().map(|r| {
        quote! { #[serde(rename = #r)] }
    });
    let skip = if field.skip_serializing_if_none {
        Some(quote! { #[serde(skip_serializing_if = "Option::is_none")] })
    } else {
        None
    };

    quote! {
        #doc
        #rename
        #skip
        pub #name: #ty
    }
}

fn rust_type_to_syn(ty: &RustType) -> TokenStream {
    match ty {
        RustType::String => quote! { String },
        RustType::I32 => quote! { i32 },
        RustType::Vec(inner) => {
            let inner_ty = rust_type_to_syn(inner);
            quote! { Vec<#inner_ty> }
        },
        RustType::Option(inner) => {
            let inner_ty = rust_type_to_syn(inner);
            quote! { Option<#inner_ty> }
        },
        RustType::Named(name) => {
            let ident = Ident::new(name, Span::call_site());
            quote! { #ident }
        },
        // ... handle other types
    }
}
```

#### Phase 4: Format and Output (Week 2)
```rust
// src/codegen/mod.rs
use prettyplease;
use syn;

pub fn generate_code(spec: Spec) -> Result<String, Error> {
    // 1. OpenAPI → IR
    let converter = OpenApiToIr::new(spec);
    let types = converter.convert();

    // 2. IR → TokenStream
    let mut tokens = quote! {
        use serde::{Deserialize, Serialize};
    };

    for type_def in types {
        let type_tokens = generate_type_def(&type_def);
        tokens.extend(type_tokens);
    }

    // 3. TokenStream → AST → Formatted Code
    let ast: syn::File = syn::parse2(tokens)?;
    let code = prettyplease::unparse(&ast);

    Ok(code)
}
```

#### Phase 5: Testing (Ongoing)
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_generation() {
        let def = StructDef {
            name: "User".to_string(),
            doc: Some("A user account".to_string()),
            derives: vec!["Debug".to_string(), "Clone".to_string()],
            fields: vec![
                FieldDef {
                    name: "id".to_string(),
                    ty: RustType::String,
                    optional: false,
                    // ...
                },
            ],
        };

        let tokens = generate_struct(&def);
        let code = prettyplease::unparse(&syn::parse2(tokens).unwrap());

        assert!(code.contains("pub struct User"));
        assert!(code.contains("pub id: String"));
    }
}
```

### Benefits You'll See

1. **Immediate**: Cleaner, more maintainable code
2. **Short-term**: Easier to add features (like client generation with reqwest)
3. **Long-term**: Foundation for advanced features (auth, signing, sessions)

### Migration Strategy

You don't have to rewrite everything at once:

1. **Start small**: Convert just the enum generator first
2. **Parallel systems**: Keep old system while building new one
3. **Compare outputs**: Ensure new system generates equivalent code
4. **Switch over**: Once confident, remove old string-based system
5. **Iterate**: Add client generation, request signing, etc.

---

## Alternative: If You Want Quick Results

If you need a **working client ASAP** and don't care about learning/customization:

### Use Progenitor + Customize Later

```bash
cargo install progenitor-client
```

```rust
// build.rs
fn main() {
    let spec = std::fs::read_to_string("openapi.json").unwrap();
    let output = progenitor_client::generate(
        &spec,
        progenitor_client::GenerationSettings::default()
            .with_interface_style(InterfaceStyle::Builder)
    ).unwrap();

    std::fs::write(
        format!("{}/generated.rs", std::env::var("OUT_DIR").unwrap()),
        output
    ).unwrap();
}
```

Then add your custom auth/signing logic on top of the generated client.

---

## Conclusion

**For your specific needs** (better code quality, maintainability, eventual full client):

🏆 **Winner: Option 1 (quote + syn + prettyplease)**

This gives you:
- Clean architecture with IR
- High-quality generated code
- Full control for future features (auth, signing, sessions)
- Industry-standard approach
- Testable, maintainable codebase

**Start with**: Phase 1 (IR types) this week, then gradually migrate your existing string-based generation to the AST-based approach.
