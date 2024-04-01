//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

use catalyzer::*;

catalyze![index];

#[get("/")]
fn index() {
    res::Html::from_file("index.html").await
}
