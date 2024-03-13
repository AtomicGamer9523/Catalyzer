//! Run with
//!
//! ```not_rust
//! cargo run -p example-builtin-default
//! ```

#[catalyzer::main]
async fn main() {
    catalyzer::App::default()
        .launch("0.0.0.0:3000")
}
