# Better Naming and Module Structure Design

## Current Problems

### Problem 1: Monolithic Names
❌ **Current:** `ServiceAccountsServiceAccountIdGETResponseUserInfoKind`
- Too long
- Hard to read
- No organization
- All types in flat namespace or single-level modules

### Problem 2: Shared Crate
❌ **Current:** Old generator and new IR system in same crate
- Breaking old code prevents new system from working
- Can't evolve independently
- Coupling between two different systems

## Proposed Solutions

### Solution 1: Nested Module Hierarchy

✅ **Better Approach:**
```
/service_accounts/{service_account_id} GET response

Current (BAD):
- ServiceAccountsServiceAccountIdGETResponse

New (GOOD):
- service_accounts::service_account_id::GetResponse
- service_accounts::service_account_id::UserInfo
- service_accounts::service_account_id::UserInfoKind
```

#### Module Structure:
```rust
pub mod service_accounts {
    pub mod service_account_id {
        // GET /service_accounts/{service_account_id}
        pub struct GetResponse { ... }
        pub struct UserInfo { ... }
        pub enum UserInfoKind { ... }

        // PUT /service_accounts/{service_account_id}
        pub struct PutRequest { ... }
        pub struct PutResponse { ... }
    }

    // POST /service_accounts
    pub struct PostRequest { ... }
    pub struct PostResponse { ... }

    // GET /service_accounts (list)
    pub struct ListResponse { ... }
}
```

### Solution 2: Workspace with Separate Crates

```
dfns_gen/               (workspace root)
├── Cargo.toml         (workspace definition)
├── dfns-gen-core/     (new IR-based generator)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── ir/
│   │   └── codegen/
│   └── examples/
├── dfns-gen-legacy/   (old string-based generator)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   └── codegen/
│   └── README.md
└── dfns-gen/          (CLI tool that uses core)
    ├── Cargo.toml
    └── src/
        └── main.rs
```

## Implementation Plan

### Phase 1: Fix Naming (Immediate)

Update `path_to_mod_and_name()` logic:

**Current:**
```rust
/service_accounts/{service_account_id}/actions/{action_id}
→ module: "service_accounts"
→ type prefix: "ServiceAccountIdActionsActionId"
→ final: ServiceAccountIdActionsActionIdGETResponse
```

**New:**
```rust
/service_accounts/{service_account_id}/actions/{action_id}
→ modules: ["service_accounts", "service_account_id", "actions", "action_id"]
→ type: "GetResponse" (or "ListResponse", "CreateRequest", etc.)
→ usage: service_accounts::service_account_id::actions::action_id::GetResponse
```

### Phase 2: Workspace Structure (Next)

1. Create workspace Cargo.toml
2. Move old generator to `dfns-gen-legacy/`
3. Move new IR system to `dfns-gen-core/`
4. Create main CLI in `dfns-gen/` that uses core

## Naming Conventions

### Module Names
- Path segments become module names
- Parameters become module names (snake_case)
- Nested based on URL structure

```
/wallets                 → wallets
/wallets/{walletId}      → wallets::wallet_id
/auth/action/init        → auth::action::init
```

### Type Names
- Based on HTTP method + purpose
- Short and clear
- Context from module path

```
GET    → GetResponse, ListResponse
POST   → CreateRequest, CreateResponse
PUT    → UpdateRequest, UpdateResponse
DELETE → DeleteResponse
PATCH  → PatchRequest, PatchResponse
```

### Special Cases

**List endpoints:**
```
GET /wallets → wallets::ListResponse
```

**Single resource:**
```
GET /wallets/{id} → wallets::wallet_id::GetResponse
```

**Actions/Operations:**
```
POST /wallets/{id}/export → wallets::wallet_id::export::CreateRequest
```

## Examples of Improvement

### Before:
```rust
// Flat namespace or single module
pub mod wallets {
    pub struct WalletIdExportPostRequest { ... }
    pub struct WalletIdExportPostResponse { ... }
    pub struct WalletIdSignaturesGetResponse { ... }
    pub struct WalletIdSignaturesSignatureIdGetResponse { ... }
}
```

### After:
```rust
pub mod wallets {
    // GET /wallets
    pub struct ListResponse { ... }

    // POST /wallets
    pub struct CreateRequest { ... }
    pub struct CreateResponse { ... }

    pub mod wallet_id {
        // GET /wallets/{walletId}
        pub struct GetResponse { ... }

        // PUT /wallets/{walletId}
        pub struct UpdateRequest { ... }
        pub struct UpdateResponse { ... }

        pub mod export {
            // POST /wallets/{walletId}/export
            pub struct CreateRequest { ... }
            pub struct CreateResponse { ... }
        }

        pub mod signatures {
            // GET /wallets/{walletId}/signatures
            pub struct ListResponse { ... }

            pub mod signature_id {
                // GET /wallets/{walletId}/signatures/{signatureId}
                pub struct GetResponse { ... }
            }
        }
    }
}
```

## Benefits

### ✅ Readability
- Clear hierarchy
- Shorter names
- Context from module path

### ✅ Organization
- Related types grouped together
- Natural navigation
- IDE-friendly

### ✅ Maintainability
- Easy to find types
- Clear structure
- Logical grouping

### ✅ Scalability
- Works with any API size
- No naming collisions
- Clean namespaces

## Migration Strategy

1. **Immediate:** Implement new naming in IR-based generator
2. **Soon:** Create workspace structure
3. **Eventually:** Deprecate old generator once new one is feature-complete

## Code Generation Changes Needed

### Update `path_to_modules()`
```rust
fn path_to_modules(path: &str) -> Vec<String> {
    path.split('/')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let cleaned = s.replace('{', "").replace('}', "");
            to_snake_case(&cleaned)
        })
        .collect()
}
```

### Update `method_to_type_name()`
```rust
fn method_to_type_name(method: &str, path: &str) -> String {
    let is_list = !path.contains('{'); // No parameters = list endpoint

    match method.to_uppercase().as_str() {
        "GET" if is_list => "ListResponse".to_string(),
        "GET" => "GetResponse".to_string(),
        "POST" => "CreateRequest".to_string(), // or CreateResponse
        "PUT" => "UpdateRequest".to_string(),
        "DELETE" => "DeleteResponse".to_string(),
        "PATCH" => "PatchRequest".to_string(),
        _ => format!("{}Response", method),
    }
}
```

### Nested ModuleDef Generation
```rust
fn build_nested_modules(paths: &Paths) -> ModuleDef {
    let mut root = ModuleDef::new("root");

    for (path, path_item) in paths {
        let modules = path_to_modules(path);
        let mut current = &mut root;

        // Navigate/create nested module structure
        for module_name in modules {
            current = current.get_or_create_submodule(&module_name);
        }

        // Add types to the deepest module
        for (method, operation) in path_item.operations() {
            let type_name = method_to_type_name(method, path);
            let type_def = operation_to_type_def(operation, &type_name);
            current.add_type(type_def);
        }
    }

    root
}
```

This design gives you clean, organized, maintainable code generation!
