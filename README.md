# treelight

[![Version](https://img.shields.io/crates/v/treelight)](https://crates.io/crates/treelight)
[![Downloads](https://img.shields.io/crates/d/treelight)](https://crates.io/crates/treelight)
[![License](https://img.shields.io/crates/l/treelight)](https://crates.io/crates/treelight)
[![Docs](https://docs.rs/treelight/badge.svg)](https://docs.rs/treelight)
[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/treelight.svg)](https://web.crev.dev/rust-reviews/crate/treelight/)

A syntax highlighter for the web using [tree-sitter](https://github.com/tree-sitter/tree-sitter).

```rust
use treelight::*;

let code = r#"
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResponseError {
    #[error("api error {0}")]
    ApiError(#[from] PaypalError),
    #[error("http error {0}")]
    HttpError(#[from] reqwest::Error)
}
"#;

let result = highlight_to_html(Language::Rust, code);
println!("{}", result);
```

License: MIT
