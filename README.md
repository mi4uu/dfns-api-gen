# DFNS Generator & Client

A complete Cargo workspace for generating and using the DFNS API in Rust.

## 🏗️ Architecture

**Two separate crates in one workspace:**

```
dfns_gen/                    (workspace root)
├── Cargo.toml              (workspace manifest)
├── dfns-gen/               (Generator - creates types)
│   ├── src/
│   │   ├── main.rs        → CLI: cargo run -p dfns-gen
│   │   ├── ir/            → Intermediate Representation
│   │   └── codegen/       → Code generation logic
│   └── examples/
│
└── dfns-client/            (Client - uses types)
    ├── src/
    │   ├── generated.rs   → Generated types (DO NOT EDIT)
    │   ├── generated_api.rs → Generated API (DO NOT EDIT)
    │   ├── client/        → HTTP client implementation
    │   └── api_server.rs  → Mock API server binary
    └── examples/
```

## 🚀 Quick Start

### 1. Generate Types

```bash
cd dfns-gen
cargo run
```

This fetches the DFNS OpenAPI spec and generates:
- Types in `../dfns-client/src/generated.rs`
- API server in `../dfns-client/src/generated_api.rs`

### 2. Use the Client

```rust
use dfns_client::{DfnsClient, DfnsConfig};

let config = DfnsConfig::new(
    "https://api.dfns.ninja",
    "service-account-token",
    "-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----"
);

let client = DfnsClient::new(config)?;

// Service account request
let wallets = client.service_account().get("/wallets").await?;

// User operations
let user_client = client.as_user("user@example.com").authenticate().await?;
let wallet = user_client.get("/wallets/wa-123").await?;
```

## 📦 Commands

```bash
# Generate types from DFNS OpenAPI
cargo run -p dfns-gen

# Check workspace compiles
cargo check --workspace

# Build everything
cargo build --workspace

# Run examples
cargo run -p dfns-gen --example better_naming_demo
```

## 🎯 Key Features

### Clean Module Structure

API paths → nested Rust modules:

| API Path | Generated Module |
|----------|------------------|
| `GET /wallets` | `wallets::ListResponse` |
| `GET /wallets/{id}` | `wallets::wallet_id::GetResponse` |
| `POST /wallets/{id}/transfers` | `wallets::wallet_id::transfers::CreateRequest` |

### Complete HTTP Client

- ✅ Service account authentication
- ✅ User impersonation (delegated login)
- ✅ 3-step challenge/signing flow
- ✅ Ed25519 request signing
- ✅ Retry middleware
- ✅ Type-safe builder pattern

## 🔄 Regeneration Workflow

When DFNS updates their API:

```bash
# 1. Regenerate
cd dfns-gen && cargo run

# 2. Verify
cd .. && cargo check -p dfns-client

# 3. Commit
git add dfns-client/src/generated*.rs
git commit -m "Update DFNS API types"
```

## ✅ Status

**Working:**
- ✅ Generator fetches and parses OpenAPI
- ✅ Nested modules with clean naming
- ✅ Hyphens → underscores (`latest-unaccepted` → `latest_unaccepted`)
- ✅ HTTP client (auth, signing, challenges)
- ✅ Workspace compiles

**TODO:**
- 🚧 Update endpoint generator for nested modules
- 🚧 Add more examples

## 📚 Documentation

- [CLIENT_README.md](CLIENT_README.md) - Detailed client usage
- [BETTER_NAMING_DESIGN.md](BETTER_NAMING_DESIGN.md) - Design rationale

## License

MIT
