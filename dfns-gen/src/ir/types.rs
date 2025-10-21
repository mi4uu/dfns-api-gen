use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Top-level type definition that can be generated
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeDef {
    /// A Rust struct
    Struct(StructDef),
    /// A Rust enum
    Enum(EnumDef),
    /// A type alias (type Foo = Bar;)
    TypeAlias(TypeAliasDef),
}

impl TypeDef {
    pub fn name(&self) -> &str {
        match self {
            TypeDef::Struct(s) => &s.name,
            TypeDef::Enum(e) => &e.name,
            TypeDef::TypeAlias(t) => &t.name,
        }
    }

    pub fn set_doc(&mut self, doc: Option<String>) {
        match self {
            TypeDef::Struct(s) => s.doc = doc,
            TypeDef::Enum(e) => e.doc = doc,
            TypeDef::TypeAlias(t) => t.doc = doc,
        }
    }
}

/// Definition of a Rust struct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructDef {
    /// The struct name (PascalCase)
    pub name: String,

    /// Optional documentation comment
    pub doc: Option<String>,

    /// Derive macros to apply (e.g., ["Debug", "Clone", "Serialize"])
    pub derives: Vec<String>,

    /// Struct fields
    pub fields: Vec<FieldDef>,

    /// Additional attributes (e.g., #[serde(rename_all = "camelCase")])
    pub attributes: Vec<String>,

    /// Whether this is a tuple struct
    pub is_tuple: bool,
}

impl StructDef {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            doc: None,
            derives: vec![
                "Debug".to_string(),
                "Clone".to_string(),
                "PartialEq".to_string(),
                "Serialize".to_string(),
                "Deserialize".to_string(),
            ],
            fields: Vec::new(),
            attributes: Vec::new(),
            is_tuple: false,
        }
    }

    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc = Some(doc.into());
        self
    }

    pub fn with_derives(mut self, derives: Vec<String>) -> Self {
        self.derives = derives;
        self
    }

    pub fn add_derive(mut self, derive: impl Into<String>) -> Self {
        self.derives.push(derive.into());
        self
    }

    pub fn with_field(mut self, field: FieldDef) -> Self {
        self.fields.push(field);
        self
    }

    pub fn with_fields(mut self, fields: Vec<FieldDef>) -> Self {
        self.fields = fields;
        self
    }

    pub fn with_attribute(mut self, attr: impl Into<String>) -> Self {
        self.attributes.push(attr.into());
        self
    }

    pub fn as_tuple(mut self) -> Self {
        self.is_tuple = true;
        self
    }
}

/// Definition of a struct field
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldDef {
    /// The field name (snake_case)
    pub name: String,

    /// The Rust type
    pub ty: RustType,

    /// Optional documentation comment
    pub doc: Option<String>,

    /// Serde rename attribute value (if different from field name)
    pub serde_rename: Option<String>,

    /// Whether the field is optional (affects whether to use Option<T>)
    pub optional: bool,

    /// Whether to add skip_serializing_if = "Option::is_none"
    pub skip_serializing_if_none: bool,

    /// Additional field attributes
    pub attributes: Vec<String>,

    /// Visibility (pub, pub(crate), etc.) - defaults to "pub"
    pub visibility: Visibility,
}

impl FieldDef {
    pub fn new(name: impl Into<String>, ty: RustType) -> Self {
        Self {
            name: name.into(),
            ty,
            doc: None,
            serde_rename: None,
            optional: false,
            skip_serializing_if_none: false,
            attributes: Vec::new(),
            visibility: Visibility::Public,
        }
    }

    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc = Some(doc.into());
        self
    }

    pub fn with_serde_rename(mut self, rename: impl Into<String>) -> Self {
        self.serde_rename = Some(rename.into());
        self
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self.skip_serializing_if_none = true;
        self
    }

    pub fn required(mut self) -> Self {
        self.optional = false;
        self.skip_serializing_if_none = false;
        self
    }

    pub fn with_attribute(mut self, attr: impl Into<String>) -> Self {
        self.attributes.push(attr.into());
        self
    }

    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    /// Get the actual type to use in code generation
    /// (wraps in Option<T> if optional)
    pub fn effective_type(&self) -> RustType {
        if self.optional && !matches!(self.ty, RustType::Option(_)) {
            RustType::Option(Box::new(self.ty.clone()))
        } else {
            self.ty.clone()
        }
    }
}

/// Definition of a Rust enum
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumDef {
    /// The enum name (PascalCase)
    pub name: String,

    /// Optional documentation comment
    pub doc: Option<String>,

    /// Derive macros to apply
    pub derives: Vec<String>,

    /// Enum variants
    pub variants: Vec<VariantDef>,

    /// Additional attributes
    pub attributes: Vec<String>,

    /// Representation (e.g., for #[serde(untagged)])
    pub representation: EnumRepresentation,
}

impl EnumDef {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            doc: None,
            derives: vec![
                "Debug".to_string(),
                "Clone".to_string(),
                "PartialEq".to_string(),
                "Serialize".to_string(),
                "Deserialize".to_string(),
            ],
            variants: Vec::new(),
            attributes: Vec::new(),
            representation: EnumRepresentation::External,
        }
    }

    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc = Some(doc.into());
        self
    }

    pub fn with_derives(mut self, derives: Vec<String>) -> Self {
        self.derives = derives;
        self
    }

    pub fn add_derive(mut self, derive: impl Into<String>) -> Self {
        self.derives.push(derive.into());
        self
    }

    pub fn with_variant(mut self, variant: VariantDef) -> Self {
        self.variants.push(variant);
        self
    }

    pub fn with_variants(mut self, variants: Vec<VariantDef>) -> Self {
        self.variants = variants;
        self
    }

    pub fn with_representation(mut self, repr: EnumRepresentation) -> Self {
        self.representation = repr;
        self
    }

    pub fn with_attribute(mut self, attr: impl Into<String>) -> Self {
        self.attributes.push(attr.into());
        self
    }
}

/// Enum variant definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariantDef {
    /// The variant name (PascalCase)
    pub name: String,

    /// Optional documentation comment
    pub doc: Option<String>,

    /// Variant data
    pub data: VariantData,

    /// Serde rename attribute value (if different from variant name)
    pub serde_rename: Option<String>,

    /// Additional variant attributes
    pub attributes: Vec<String>,

    /// Whether this is the default variant
    pub is_default: bool,
}

impl VariantDef {
    pub fn unit(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            doc: None,
            data: VariantData::Unit,
            serde_rename: None,
            attributes: Vec::new(),
            is_default: false,
        }
    }

    pub fn tuple(name: impl Into<String>, ty: RustType) -> Self {
        Self {
            name: name.into(),
            doc: None,
            data: VariantData::Tuple(vec![ty]),
            serde_rename: None,
            attributes: Vec::new(),
            is_default: false,
        }
    }

    pub fn struct_variant(name: impl Into<String>, fields: Vec<FieldDef>) -> Self {
        Self {
            name: name.into(),
            doc: None,
            data: VariantData::Struct(fields),
            serde_rename: None,
            attributes: Vec::new(),
            is_default: false,
        }
    }

    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc = Some(doc.into());
        self
    }

    pub fn with_serde_rename(mut self, rename: impl Into<String>) -> Self {
        self.serde_rename = Some(rename.into());
        self
    }

    pub fn with_attribute(mut self, attr: impl Into<String>) -> Self {
        self.attributes.push(attr.into());
        self
    }

    pub fn as_default(mut self) -> Self {
        self.is_default = true;
        self
    }
}

/// Data associated with an enum variant
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VariantData {
    /// Unit variant (e.g., `Foo`)
    Unit,

    /// Tuple variant (e.g., `Foo(String, i32)`)
    Tuple(Vec<RustType>),

    /// Struct variant (e.g., `Foo { bar: String }`)
    Struct(Vec<FieldDef>),
}

/// Enum representation style for serde
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnumRepresentation {
    /// Default serde representation
    External,

    /// #[serde(untagged)]
    Untagged,

    /// #[serde(tag = "type")]
    InternallyTagged { tag: String },

    /// #[serde(tag = "type", content = "data")]
    AdjacentlyTagged { tag: String, content: String },
}

/// Type alias definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeAliasDef {
    /// The alias name
    pub name: String,

    /// Optional documentation comment
    pub doc: Option<String>,

    /// The target type
    pub target: RustType,

    /// Visibility
    pub visibility: Visibility,
}

impl TypeAliasDef {
    pub fn new(name: impl Into<String>, target: RustType) -> Self {
        Self {
            name: name.into(),
            doc: None,
            target,
            visibility: Visibility::Public,
        }
    }

    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc = Some(doc.into());
        self
    }

    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }
}

/// Rust type representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RustType {
    /// String type
    String,

    /// i32
    I32,

    /// i64
    I64,

    /// u32
    U32,

    /// u64
    U64,

    /// f32
    F32,

    /// f64
    F64,

    /// bool
    Bool,

    /// Vec<T>
    Vec(Box<RustType>),

    /// Option<T>
    Option(Box<RustType>),

    /// HashMap<K, V>
    HashMap(Box<RustType>, Box<RustType>),

    /// HashSet<T>
    HashSet(Box<RustType>),

    /// Box<T>
    Box(Box<RustType>),

    /// A named type (custom struct, enum, or external type)
    /// Can include module path (e.g., "std::path::PathBuf" or "MyType")
    Named(String),

    /// serde_json::Value
    JsonValue,

    /// A tuple type: (T1, T2, ...)
    Tuple(Vec<RustType>),

    /// Unit type ()
    Unit,

    /// A reference type: &T or &mut T
    Reference {
        mutable: bool,
        inner: Box<RustType>,
    },
}

impl RustType {
    /// Create a named type
    pub fn named(name: impl Into<String>) -> Self {
        RustType::Named(name.into())
    }

    /// Wrap this type in Option<T>
    pub fn optional(self) -> Self {
        RustType::Option(Box::new(self))
    }

    /// Wrap this type in Vec<T>
    pub fn vec(self) -> Self {
        RustType::Vec(Box::new(self))
    }

    /// Wrap this type in Box<T>
    pub fn boxed(self) -> Self {
        RustType::Box(Box::new(self))
    }

    /// Create a HashMap<String, V>
    pub fn string_map(value_type: RustType) -> Self {
        RustType::HashMap(Box::new(RustType::String), Box::new(value_type))
    }

    /// Check if this type is already Optional
    pub fn is_optional(&self) -> bool {
        matches!(self, RustType::Option(_))
    }
}

/// Visibility modifier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    PublicCrate,
    Private,
}

impl Visibility {
    pub fn as_str(&self) -> &str {
        match self {
            Visibility::Public => "pub",
            Visibility::PublicCrate => "pub(crate)",
            Visibility::Private => "",
        }
    }
}

/// Module definition (for organizing generated types)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModuleDef {
    /// Module name
    pub name: String,

    /// Optional documentation
    pub doc: Option<String>,

    /// Types defined in this module
    pub types: Vec<TypeDef>,

    /// Nested submodules
    pub submodules: Vec<ModuleDef>,

    /// Visibility
    pub visibility: Visibility,
}

impl ModuleDef {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            doc: None,
            types: Vec::new(),
            submodules: Vec::new(),
            visibility: Visibility::Public,
        }
    }

    pub fn with_doc(mut self, doc: impl Into<String>) -> Self {
        self.doc = Some(doc.into());
        self
    }

    pub fn with_type(mut self, type_def: TypeDef) -> Self {
        self.types.push(type_def);
        self
    }

    pub fn with_types(mut self, types: Vec<TypeDef>) -> Self {
        self.types = types;
        self
    }

    pub fn with_submodule(mut self, module: ModuleDef) -> Self {
        self.submodules.push(module);
        self
    }

    pub fn with_submodules(mut self, modules: Vec<ModuleDef>) -> Self {
        self.submodules = modules;
        self
    }

    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    /// Add a type to this module
    pub fn add_type(&mut self, type_def: TypeDef) {
        self.types.push(type_def);
    }

    /// Get or create a submodule by name
    pub fn get_or_create_submodule(&mut self, name: &str) -> &mut ModuleDef {
        // Find existing submodule
        if let Some(pos) = self.submodules.iter().position(|m| m.name == name) {
            return &mut self.submodules[pos];
        }

        // Create new submodule
        self.submodules.push(ModuleDef::new(name));
        self.submodules.last_mut().unwrap()
    }

    /// Navigate to a nested module by path, creating modules as needed
    /// Example: ["service_accounts", "service_account_id", "actions"]
    pub fn navigate_to_module(&mut self, path: &[String]) -> &mut ModuleDef {
        if path.is_empty() {
            return self;
        }

        let mut current = self;
        for segment in path {
            current = current.get_or_create_submodule(segment);
        }
        current
    }
}

/// Complete code generation output
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeGenOutput {
    /// Top-level types (not in any module)
    pub types: Vec<TypeDef>,

    /// Modules containing types
    pub modules: Vec<ModuleDef>,

    /// Imports/use statements
    pub imports: Vec<String>,

    /// Top-level attributes and pragmas
    pub file_attributes: Vec<String>,
}

impl CodeGenOutput {
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            modules: Vec::new(),
            imports: vec![
                "use serde::{Deserialize, Serialize};".to_string(),
            ],
            file_attributes: vec![
                "// Auto-generated from OpenAPI schema".to_string(),
                "// Do not edit manually".to_string(),
                "#![allow(unused_imports)]".to_string(),
            ],
        }
    }

    pub fn with_type(mut self, type_def: TypeDef) -> Self {
        self.types.push(type_def);
        self
    }

    pub fn with_module(mut self, module: ModuleDef) -> Self {
        self.modules.push(module);
        self
    }

    pub fn with_import(mut self, import: impl Into<String>) -> Self {
        self.imports.push(import.into());
        self
    }

    pub fn with_attribute(mut self, attr: impl Into<String>) -> Self {
        self.file_attributes.push(attr.into());
        self
    }
}

impl Default for CodeGenOutput {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct_builder() {
        let struct_def = StructDef::new("User")
            .with_doc("A user account")
            .with_field(
                FieldDef::new("id", RustType::String)
                    .with_doc("User ID")
                    .with_serde_rename("userId")
            )
            .with_field(
                FieldDef::new("name", RustType::String)
                    .optional()
            );

        assert_eq!(struct_def.name, "User");
        assert_eq!(struct_def.fields.len(), 2);
        assert_eq!(struct_def.fields[0].name, "id");
        assert!(!struct_def.fields[0].optional);
        assert_eq!(struct_def.fields[1].name, "name");
        assert!(struct_def.fields[1].optional);
    }

    #[test]
    fn test_enum_builder() {
        let enum_def = EnumDef::new("Status")
            .with_doc("User status")
            .with_variant(VariantDef::unit("Active"))
            .with_variant(VariantDef::unit("Inactive"))
            .with_variant(VariantDef::tuple("Suspended", RustType::String));

        assert_eq!(enum_def.name, "Status");
        assert_eq!(enum_def.variants.len(), 3);
        assert!(matches!(enum_def.variants[0].data, VariantData::Unit));
        assert!(matches!(enum_def.variants[2].data, VariantData::Tuple(_)));
    }

    #[test]
    fn test_rust_type_helpers() {
        let opt_string = RustType::String.optional();
        assert!(matches!(opt_string, RustType::Option(_)));

        let vec_int = RustType::I32.vec();
        assert!(matches!(vec_int, RustType::Vec(_)));

        let map = RustType::string_map(RustType::I64);
        assert!(matches!(map, RustType::HashMap(_, _)));
    }

    #[test]
    fn test_field_effective_type() {
        let required_field = FieldDef::new("id", RustType::String).required();
        assert_eq!(required_field.effective_type(), RustType::String);

        let optional_field = FieldDef::new("name", RustType::String).optional();
        assert!(matches!(optional_field.effective_type(), RustType::Option(_)));
    }

    #[test]
    fn test_codegen_output_builder() {
        let output = CodeGenOutput::new()
            .with_type(TypeDef::Struct(StructDef::new("Foo")))
            .with_module(
                ModuleDef::new("api")
                    .with_type(TypeDef::Enum(EnumDef::new("Status")))
            );

        assert_eq!(output.types.len(), 1);
        assert_eq!(output.modules.len(), 1);
        assert!(output.imports.len() > 0);
    }
}
