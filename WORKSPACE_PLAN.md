# Workspace Structure Plan

## Problem: Coupled Codebase

Currently, the old string-based generator and new IR-based system share the same crate. This means:
- ❌ Breaking the old generator breaks the entire project
- ❌ Can't evolve systems independently
- ❌ Mixed responsibilities and dependencies
- ❌ Confusing for maintainers

## Solution: Cargo Workspace

Split into separate crates that can evolve independently.

## Proposed Structure

```
dfns_gen/                          (workspace root)
├── Cargo.toml                     (workspace manifest)
├── README.md
├── CLAUDE.md
├── docs/
│   ├── CODE_GENERATION_ANALYSIS.md
│   ├── BETTER_NAMING_DESIGN.md
│   └── WORKSPACE_PLAN.md
│
├── dfns-gen-core/                 (NEW IR-based generator)
│   ├── Cargo.toml
│   ├── README.md
│   ├── src/
│   │   ├── lib.rs
│   │   ├── ir/
│   │   │   ├── mod.rs
│   │   │   └── types.rs
│   │   └── codegen/
│   │       ├── mod.rs
│   │       ├── ir_to_ast.rs
│   │       └── openapi_to_ir.rs (when fixed)
│   ├── examples/
│   │   ├── ir_demo.rs
│   │   ├── ir_codegen_demo.rs
│   │   └── better_naming_demo.rs
│   └── tests/
│
├── dfns-gen-legacy/               (OLD string-based generator)
│   ├── Cargo.toml
│   ├── README.md
│   ├── src/
│   │   ├── lib.rs
│   │   └── codegen/
│   │       ├── mod.rs
│   │       ├── schema_generator.rs
│   │       ├── endpoint_generator.rs
│   │       ├── type_mapper.rs
│   │       └── utils.rs
│   └── README.md  (deprecation notice)
│
└── dfns-gen/                      (CLI tool)
    ├── Cargo.toml
    ├── README.md
    └── src/
        ├── main.rs
        └── commands/
            ├── generate.rs
            ├── convert.rs
            └── downgrade.rs
```

## Workspace Cargo.toml

```toml
[workspace]
members = [
    "dfns-gen-core",
    "dfns-gen-legacy",
    "dfns-gen",
]
resolver = "2"

[workspace.package]
version = "0.2.0"
edition = "2021"
authors = ["Your Name"]
license = "MIT"

[workspace.dependencies]
# Shared dependencies
oas3 = { version = "0.19.0", features = ["yaml-spec"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
tokio = { version = "1.48", features = ["full"] }
reqwest = "0.12"

# Code generation
quote = "1"
syn = { version = "2", features = ["full", "parsing"] }
proc-macro2 = "1"
prettyplease = "0.2"

# OpenAPI support
utoipa = "5.4"
axum = "0.8"
```

## Individual Crate Configs

### dfns-gen-core/Cargo.toml

```toml
[package]
name = "dfns-gen-core"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
description = "Modern IR-based OpenAPI to Rust code generator"

[dependencies]
oas3 = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
quote = { workspace = true }
syn = { workspace = true }
proc-macro2 = { workspace = true }
prettyplease = { workspace = true }
utoipa = { workspace = true }
smart-default = "0.7"

[dev-dependencies]
tokio = { workspace = true }
```

### dfns-gen-legacy/Cargo.toml

```toml
[package]
name = "dfns-gen-legacy"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
description = "Legacy string-based OpenAPI to Rust code generator (deprecated)"

[dependencies]
oas3 = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
smart-default = "0.7"
```

### dfns-gen/Cargo.toml

```toml
[package]
name = "dfns-gen"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
description = "CLI tool for generating Rust code from OpenAPI specs"

[[bin]]
name = "dfns-gen"
path = "src/main.rs"

[dependencies]
dfns-gen-core = { path = "../dfns-gen-core" }
# Optional: dfns-gen-legacy = { path = "../dfns-gen-legacy", optional = true }

anyhow = { workspace = true }
tokio = { workspace = true }
reqwest = { workspace = true }
clap = { version = "4.5", features = ["derive"] }

[features]
default = []
legacy = ["dep:dfns-gen-legacy"]  # Enable legacy generator
```

## Migration Steps

### Step 1: Create Workspace Root

```bash
# Create workspace Cargo.toml at root
cat > Cargo.toml << 'EOF'
[workspace]
members = [
    "dfns-gen-core",
    "dfns-gen-legacy",
    "dfns-gen",
]
resolver = "2"
EOF
```

### Step 2: Create dfns-gen-core

```bash
# Create new crate for IR-based system
mkdir -p dfns-gen-core/src
cd dfns-gen-core

# Move IR and new codegen
cp -r ../src/ir src/
cp ../src/codegen/ir_to_ast.rs src/codegen/
# (openapi_to_ir.rs when fixed)

# Move examples
cp -r ../examples .

# Create Cargo.toml for core
cat > Cargo.toml << 'EOF'
[package]
name = "dfns-gen-core"
version = "0.2.0"
edition = "2021"
# ... rest of dependencies
EOF
```

### Step 3: Create dfns-gen-legacy

```bash
# Create crate for old generator
mkdir -p dfns-gen-legacy/src
cd dfns-gen-legacy

# Move old codegen
cp -r ../src/codegen src/
# Remove ir_to_ast.rs from this copy

# Create Cargo.toml
cat > Cargo.toml << 'EOF'
[package]
name = "dfns-gen-legacy"
version = "0.2.0"
edition = "2021"
# ... rest of dependencies
EOF

# Add deprecation notice
cat > README.md << 'EOF'
# dfns-gen-legacy

⚠️ **DEPRECATED**: This crate contains the legacy string-based code generator.

Please use `dfns-gen-core` for new projects. It provides:
- Better type safety
- Cleaner code generation
- Nested module support
- Better naming conventions

This crate is maintained for compatibility only.
EOF
```

### Step 4: Create CLI Tool

```bash
# Create main CLI crate
mkdir -p dfns-gen/src
cd dfns-gen

# Create main.rs with clap
cat > src/main.rs << 'EOF'
use clap::{Parser, Subcommand};
use dfns_gen_core::codegen::ir_to_ast;
use dfns_gen_core::ir::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate Rust code from OpenAPI spec (new IR-based)
    Generate {
        /// Path to OpenAPI spec
        #[arg(short, long)]
        input: String,

        /// Output file path
        #[arg(short, long)]
        output: String,
    },
    /// Convert using legacy generator (deprecated)
    #[cfg(feature = "legacy")]
    Legacy {
        #[arg(short, long)]
        input: String,

        #[arg(short, long)]
        output: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { input, output } => {
            // Use new IR-based generator
            println!("Generating with dfns-gen-core...");
            // Implementation...
        }
        #[cfg(feature = "legacy")]
        Commands::Legacy { input, output } => {
            println!("⚠️  Using deprecated legacy generator");
            // Use dfns-gen-legacy
        }
    }

    Ok(())
}
EOF
```

## Benefits

### ✅ Independence
- Core and legacy can evolve separately
- Breaking changes in one don't affect the other
- Can deprecate legacy without removing it

### ✅ Clarity
- Clear separation of responsibilities
- Easy to see which is which
- Obvious migration path

### ✅ Flexibility
- Users can choose which generator to use
- Can run both side-by-side for comparison
- Easy to test new features in core

### ✅ Maintenance
- Can mark legacy as deprecated
- Reduce maintenance burden over time
- Focus efforts on core

## Usage After Migration

```bash
# Build everything
cargo build --workspace

# Build just core
cargo build -p dfns-gen-core

# Build CLI without legacy
cargo build -p dfns-gen

# Build CLI with legacy support
cargo build -p dfns-gen --features legacy

# Run tests for specific crate
cargo test -p dfns-gen-core

# Run example from core
cargo run -p dfns-gen-core --example better_naming_demo
```

## Timeline

1. **Immediate (< 1 hour)**: Create workspace structure, move files
2. **Short-term (1-2 hours)**: Fix imports, test everything compiles
3. **Medium-term (when ready)**: Finish openapi_to_ir, make core fully functional
4. **Long-term**: Deprecate legacy, eventually remove it

## Documentation Updates

After migration, update:
- Root README.md (explain workspace structure)
- CLAUDE.md (point to correct crates)
- Each crate gets its own README.md
- Add MIGRATION.md guide for users

This structure makes both issues you raised much better!
