// Intermediate Representation (IR) for Rust code generation
//
// This module defines a clean, structured representation of Rust types
// that sits between OpenAPI schemas and generated code. This approach
// provides several benefits:
//
// 1. Separation of concerns: Parsing OpenAPI is separate from code generation
// 2. Testability: Each transformation step can be unit tested
// 3. Flexibility: Easy to add new output formats or modify generation logic
// 4. Type safety: Structured data instead of string concatenation

pub mod types;

pub use types::*;
