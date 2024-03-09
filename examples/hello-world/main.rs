//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

use catalyzer::*;
use res::Html;

#[main]
async fn main() {
    App::new()
        .get("/", index)
        .launch("127.0.0.1:3000")
}

async fn index() -> Result<Html> {
    Html::from_file("index.html").await
}
