#![allow(missing_docs)]

// #!/bin/sh
// 
// # Apply patches
// cp .github/patches/sidebar-items.js.patch target/doc/catalyzer/sidebar-items.js
// cp .github/patches/index.html.patch target/doc/catalyzer/index.html

use std::fmt::Display;
use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    println!("Patching sidebar-items.js");
    patch("sidebar-items.js")?;
    println!("Patching index.html");
    patch("index.html")?;
    Ok(())
}

fn patch<F: Display>(file_name: F) -> io::Result<()> {
    use io::*;
    let cwd = std::env::current_dir()?;
    println!("Current directory: {:?}", cwd);

    let patches_dir = cwd.join(".github/patches");
    let doc_dir = cwd.join("target/doc/catalyzer");
    let patch_file_name = patches_dir.join(format!("{file_name}.patch"));
    let unpatched_file_name = doc_dir.join(format!("{file_name}"));


    let patch_file = File::options()
        .read(true)
        .write(false)
        .open(patch_file_name)?;
    let unpatched_file = File::options()
        .read(false)
        .write(true)
        .create(true)
        .open(unpatched_file_name)?;
    let mut patch = io::BufReader::new(patch_file);
    let mut temp = Vec::with_capacity(patch.buffer().len());
    let mut unpatched = io::BufWriter::new(unpatched_file);
    patch.read_to_end(&mut temp)?;
    unpatched.write_all(&temp)?;
    Ok(())
}
