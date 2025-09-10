# Rust Project Organization & Packaging

#packaging #cargo #modules #crates #workspaces

> Managing code organization, dependencies, and project structure in Rust

## 📦 The Big Picture

Rust organizes code through a hierarchical system:
- **Packages** - Top-level unit that contains crates
- **Crates** - Compilation unit (binary or library)
- **Modules** - Organize code within a crate
- **Paths** - Navigate the module tree

## 🚀 Cargo Fundamentals

### Project Creation
```bash
# Create new binary project
cargo new my_project
cargo new --bin my_project

# Create new library project
cargo new --lib my_library

# Initialize in existing directory
cargo init
cargo init --lib
```

### Basic Commands
```bash
# Build project
cargo build
cargo build --release

# Run project (binaries only)
cargo run
cargo run --release

# Check compilation without building
cargo check

# Run tests
cargo test

# Generate documentation
cargo doc --open

# Clean build artifacts
cargo clean

# Update dependencies
cargo update
```

### Cargo.toml Structure
```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <email@example.com>"]
license = "MIT"
description = "A brief description"
homepage = "https://github.com/user/repo"
repository = "https://github.com/user/repo"
readme = "README.md"
keywords = ["cli", "utility"]
categories = ["command-line-utilities"]

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
rand = { version = "0.8", optional = true }

[dev-dependencies]
criterion = "0.4"

[build-dependencies]
cc = "1.0"

[[bin]]
name = "my_binary"
path = "src/bin/my_binary.rs"

[features]
default = ["json"]
json = ["serde/derive"]
random = ["rand"]

[profile.release]
opt-level = 3
lto = true
```

## 📂 Module System

### Module Declaration Patterns

#### Inline Modules
```rust
// src/lib.rs or src/main.rs
mod network {
    fn connect() {
        // Function implementation
    }

    mod server {
        fn bind() {
            // Server binding logic
        }
    }
}
```

#### File-based Modules
```rust
// src/lib.rs
mod network;     // Looks for src/network.rs
mod database;    // Looks for src/database.rs

// src/network.rs
pub fn connect() {
    println!("Connecting to network...");
}

pub mod server {  // Creates network::server module
    pub fn bind() {
        println!("Binding server...");
    }
}
```

#### Directory-based Modules
```
src/
├── lib.rs
├── network/
│   ├── mod.rs      // Module root file
│   ├── server.rs   // network::server
│   └── client.rs   // network::client
└── database/
    ├── mod.rs
    └── connection.rs
```

```rust
// src/lib.rs
mod network;
mod database;

// src/network/mod.rs
pub mod server;
pub mod client;

pub fn connect() {
    println!("Network connecting");
}

// src/network/server.rs
pub fn bind() {
    println!("Server binding");
}
```

### Visibility and Privacy

#### Public vs Private
```rust
mod my_module {
    fn private_function() {}        // Only accessible within my_module
    pub fn public_function() {}     // Accessible from parent module

    pub struct PublicStruct {
        pub public_field: i32,      // Public field
        private_field: String,      // Private field
    }

    impl PublicStruct {
        pub fn new(value: i32) -> Self {
            Self {
                public_field: value,
                private_field: String::from("hidden"),
            }
        }
    }
}
```

#### Restricted Visibility
```rust
pub(crate) fn crate_visible() {}      // Visible within current crate
pub(super) fn parent_visible() {}     // Visible in parent module
pub(self) fn self_visible() {}        // Same as private
pub(in crate::my_module) fn restricted() {} // Visible in specific path
```

### Path Navigation

#### Absolute Paths
```rust
use crate::network::server::bind;     // From crate root
use std::collections::HashMap;        // From standard library

fn main() {
    crate::network::connect();        // Absolute path usage
    bind();                           // Using imported function
}
```

#### Relative Paths
```rust
mod network {
    pub mod server {
        pub fn bind() {}
    }

    pub fn connect() {
        server::bind();               // Relative to current module
        self::server::bind();         // Explicit self
        super::other_function();      // Parent module
    }
}

fn other_function() {}
```

#### Use Declarations
```rust
// Bringing items into scope
use std::collections::HashMap;
use std::collections::{BTreeMap, HashSet};
use std::io::*;  // Glob import (use sparingly)

// Renaming imports
use std::collections::HashMap as Map;
use std::result::Result as MyResult;

// Re-exporting
pub use self::network::connect;       // Re-export from submodule
```

## 📚 Crates and Libraries

### Library Crate Structure
```
my_library/
├── Cargo.toml
├── src/
│   ├── lib.rs          // Library root
│   ├── algorithms/     // Module directory
│   │   ├── mod.rs
│   │   ├── sort.rs
│   │   └── search.rs
│   └── utils.rs        // Module file
├── examples/           // Example usage
│   └── basic.rs
├── tests/             // Integration tests
│   └── integration_test.rs
├── benches/          // Benchmarks
│   └── benchmark.rs
└── README.md
```

```rust
// src/lib.rs - Library API
pub mod algorithms;
pub mod utils;

// Re-export commonly used items
pub use algorithms::sort::quick_sort;
pub use utils::*;

// Library-level documentation
//! # My Library
//!
//! This library provides efficient algorithms and utilities.
//!
//! ## Examples
//!
//! ```rust
//! use my_library::quick_sort;
//!
//! let mut data = vec![3, 1, 4, 1, 5];
//! quick_sort(&mut data);
//! assert_eq!(data, vec![1, 1, 3, 4, 5]);
//! ```
```

### Binary Crate Structure
```
my_app/
├── Cargo.toml
├── src/
│   ├── main.rs         // Primary binary
│   ├── bin/            // Additional binaries
│   │   └── helper.rs
│   ├── config.rs       // App modules
│   └── commands/
│       ├── mod.rs
│       ├── serve.rs
│       └── process.rs
└── README.md
```

### Publishing to crates.io
```bash
# Create account and login
cargo login [token]

# Prepare for publishing
cargo package          # Create distributable package
cargo package --list   # Show packaged files

# Publish
cargo publish          # Upload to crates.io
cargo publish --dry-run # Test publishing
```

## 🏗️ Workspaces

### Workspace Configuration
```toml
# Workspace root Cargo.toml
[workspace]
members = [
    "cli",
    "core",
    "server",
    "shared",
]

exclude = [
    "old_projects/*",
]

[workspace.dependencies]
serde = "1.0"
tokio = "1.0"

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Your Name <email@example.com>"]
```

### Workspace Structure
```
my_workspace/
├── Cargo.toml          # Workspace root
├── Cargo.lock          # Shared lock file
├── target/             # Shared build directory
├── cli/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── core/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── server/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── shared/
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

### Member Crate Dependencies
```toml
# cli/Cargo.toml
[package]
name = "my_cli"
version.workspace = true
edition.workspace = true

[dependencies]
# Workspace member dependency
core = { path = "../core" }
shared = { path = "../shared" }

# External dependencies
serde.workspace = true
clap = "4.0"
```

### Workspace Commands
```bash
# Build all workspace members
cargo build

# Build specific member
cargo build -p cli
cargo build --package server

# Run specific binary
cargo run -p cli
cargo run --bin server

# Test all members
cargo test

# Test specific member
cargo test -p core
```

## 🔧 Advanced Features

### Conditional Compilation
```rust
#[cfg(target_os = "windows")]
fn platform_specific() {
    println!("Running on Windows");
}

#[cfg(target_os = "linux")]
fn platform_specific() {
    println!("Running on Linux");
}

#[cfg(feature = "networking")]
mod network {
    pub fn connect() {}
}

#[cfg(debug_assertions)]
fn debug_only() {
    println!("Debug mode");
}
```

### Build Scripts
```rust
// build.rs
use std::env;
use std::path::Path;

fn main() {
    // Set environment variables for compilation
    println!("cargo:rustc-env=BUILD_TIME={}", chrono::Utc::now());

    // Link to system libraries
    println!("cargo:rustc-link-lib=ssl");

    // Rerun if files change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/proto/");

    // Generate code (protobuf, etc.)
    // protoc_rust::Codegen::new()...
}
```

### Custom Profiles
```toml
[profile.dev]
opt-level = 0
debug = true
overflow-checks = true

[profile.release]
opt-level = 3
debug = false
lto = "fat"
codegen-units = 1
panic = "abort"

[profile.test]
opt-level = 2

# Custom profile
[profile.profiling]
inherits = "release"
debug = 1
```

## 🎯 Best Practices

### Module Organization
```rust
// Prefer flat module structure when possible
mod config;
mod database;
mod handlers;

// Group related functionality
mod web {
    pub mod handlers;
    pub mod middleware;
    pub mod routes;
}

// Use descriptive module names
mod user_management;  // Not just "user"
mod order_processing; // Not just "orders"
```

### API Design
```rust
// Clear public API
pub struct Config {
    // Public fields when appropriate
    pub port: u16,
    pub host: String,

    // Keep internal fields private
    internal_state: HashMap<String, String>,
}

impl Config {
    pub fn new(port: u16, host: String) -> Self {
        Self {
            port,
            host,
            internal_state: HashMap::new(),
        }
    }

    // Provide controlled access to private data
    pub fn get_internal(&self, key: &str) -> Option<&String> {
        self.internal_state.get(key)
    }
}

// Re-export commonly used items
pub use config::Config;
pub use database::{Connection, Transaction};
```

### Dependency Management
```toml
[dependencies]
# Pin major versions for stability
serde = "1.0"
tokio = "1"

# Use specific features to reduce compile time
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
serde = { version = "1.0", features = ["derive"] }

# Optional dependencies for features
rand = { version = "0.8", optional = true }

[features]
default = []
random = ["rand"]
full = ["random"]
```

### Documentation
```rust
//! # My Crate
//!
//! Brief description of the crate's purpose.
//!
//! # Examples
//!
//! ```rust
//! use my_crate::MyStruct;
//!
//! let instance = MyStruct::new();
//! ```

/// Process data with optional configuration.
///
/// # Arguments
///
/// * `data` - Input data to process
/// * `config` - Optional configuration parameters
///
/// # Examples
///
/// ```rust
/// # use my_crate::process_data;
/// let result = process_data("hello", None);
/// assert_eq!(result, "processed: hello");
/// ```
pub fn process_data(data: &str, config: Option<&Config>) -> String {
    // Implementation
}
```

## 🔗 Integration with Other Tools

### Testing Structure
```
tests/
├── integration_test.rs     # Integration tests
├── common/                 # Shared test utilities
│   └── mod.rs
└── fixtures/               # Test data
    └── sample_data.json
```

### CI/CD Integration
```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: cargo test --all-features
      - name: Check formatting
        run: cargo fmt -- --check
      - name: Run clippy
        run: cargo clippy -- -D warnings
```

## 📋 Common Patterns

### Error Handling in Libraries
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Parse error at line {line}: {message}")]
    Parse { line: usize, message: String },
}

pub type Result<T> = std::result::Result<T, MyError>;
```

### Configuration Management
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    #[serde(default = "default_workers")]
    pub workers: usize,
}

fn default_workers() -> usize { 4 }

impl Config {
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}
```

## 🎓 Learning Path

### Prerequisites
- ✅ Understand [[ownership]] and borrowing
- ✅ Comfortable with [[traits]] and generics
- ✅ Know [[errors]] handling patterns

### Next Steps
1. Practice creating simple crates
2. Experiment with module organization
3. Build a small workspace project
4. Publish a library to crates.io
5. Learn about [[testing]] and CI/CD integration

---
