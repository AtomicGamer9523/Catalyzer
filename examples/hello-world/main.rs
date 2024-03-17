//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

use catalyzer::*;

#[main]
fn main() {
    App![index]
        .bind("0.0.0.0:3000")?
        .launch()
}

use res::Html;

#[get("/")]
fn index() {
    Html::from_file("index.html").await
}
