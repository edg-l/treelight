# treelight
A syntax highlighter for the web using tree-sitter.

Work in progress.

```rust
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
