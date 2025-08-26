# Resemble

**Resemble** is a Rust crate for comparing Rust source files by analyzing their Abstract Syntax Tree (AST). It computes a similarity score based on AST node counts and cosine similarity, allowing developers to identify how structurally similar two pieces of Rust code are.

[![crates.io](https://img.shields.io/crates/v/resemble)](https://crates.io/crates/resemble)

---

## Features

- Parse Rust source files into AST using [`syn`](https://crates.io/crates/syn)
- Count statements, expressions, types, patterns, macros, and blocks
- Represent node counts as embeddings
- Compute cosine similarity between embeddings
- CLI tool for quick Rust file comparison
- Can be used as a Rust library for programmatic similarity analysis

---

## Installation

Add `resemble` to your `Cargo.toml`:

```toml
[dependencies]
resemble = "0.1.0"
````

Or build from source:

```bash
git clone https://github.com/8ria/resemble.git
cd resemble
cargo build --release
```

---

## CLI Usage

Compare two Rust files using the command line:

```bash
cargo run -- file_a.rs file_b.rs
```

Example output:

```
Cosine similarity = 0.872341
```

> Use `--` to separate Cargo arguments from the binary arguments.

---

## Library Usage

You can also use `resemble` as a Rust library:

```rust
use resemble::{parse_and_count, similarity::similarity_from_counts};
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let counts_a: HashMap<String, f32> = parse_and_count("file_a.rs")?;
    let counts_b: HashMap<String, f32> = parse_and_count("file_b.rs")?;
    let similarity = similarity_from_counts(counts_a, counts_b);

    println!("Cosine similarity: {:.6}", similarity);
    Ok(())
}
```

---

## Project Structure

```
resemble/
├── Cargo.toml
├── README.md
├── tests/
│   └── integration_test.rs
└── src/
    ├── lib.rs           # Library entry point
    ├── ast.rs           # AST parsing and node counting
    ├── embeddings.rs    # Embedding representation
    └── similarity.rs    # Cosine similarity calculations
    └── bin/
        └── resemble.rs      # CLI tool
```

---

## Running Tests

Add sample Rust files under `tests/` (e.g., `file_a.rs` and `file_b.rs`) and a test like this:

```rust
use resemble::{parse_and_count, similarity::similarity_from_counts};
use std::collections::HashMap;

#[test]
fn test_similarity() {
    let counts_a = parse_and_count("tests/file_a.rs").unwrap();
    let counts_b = parse_and_count("tests/file_b.rs").unwrap();

    let sim = similarity_from_counts(counts_a, counts_b);
    assert!(sim >= 0.0 && sim <= 1.0);
}
```

Run all tests:

```bash
cargo test
```

---

## Contributing

Contributions are welcome! You can:

* Improve similarity algorithms
* Add support for more AST node types
* Add unit or integration tests
* Enhance the CLI tool

Open issues or pull requests on [GitHub](https://github.com/8ria/resemble).

---

## License

MIT License © [AndriaK](mailto:hey@andriaK.com)

---

## Crates.io

Version: 0.1.0
[https://crates.io/crates/resemble](https://crates.io/crates/resemble)
