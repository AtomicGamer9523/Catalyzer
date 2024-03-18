// #!/bin/sh
// 
// # Apply patches
// cp .github/patches/sidebar-items.js.patch target/doc/catalyzer/sidebar-items.js
// cp .github/patches/index.html.patch target/doc/catalyzer/index.html

use std::env::current_dir as cwd;
use std::{fs, io};

fn main() -> io::Result<()> {
    patch_sidebar_items_js()?;
    patch_index_html()?;
    Ok(())
}

fn patch_sidebar_items_js() -> io::Result<()> {
    println!("Patching sidebar-items.js");
    let patch_file = cwd()?.join(".github/patches/sidebar-items.js.patch");
    let unpatched_file = cwd()?.join("target/doc/catalyzer/sidebar-items.js");
    fs::copy(patch_file, unpatched_file)?;
    Ok(())
}

fn patch_index_html() -> io::Result<()> {
    println!("Patching index.html");
    use io::*;
    let patch_file = cwd()?.join(".github/patches/index.html.patch");
    let unpatched_file_path = cwd()?.join("target/doc/catalyzer/index.html");
    let patch_data = fs::read_to_string(patch_file)?;
    let mut unpatched_file_data = String::new();
    let mut patched_file_data = String::new();
    let mut unpatched_file = fs::OpenOptions::new()
        .read(true)
        .open(&unpatched_file_path)?;
    unpatched_file.read_to_string(&mut unpatched_file_data)?;
    for (i, line) in unpatched_file_data.lines().enumerate() {
        if i == 2 {
            patched_file_data.push_str(&patch_data);
        } else {
            patched_file_data.push_str(line);
            patched_file_data.push('\n');
        }
    }
    unpatched_file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(unpatched_file_path)?;
    unpatched_file.write_all(patched_file_data.as_bytes())?;
    Ok(())
}

// fn patch<F: Display>(file_name: F, l: Option<usize>) -> io::Result<()> {
//     use io::*;
//     let cwd = std::env::current_dir()?;
//     println!("Current directory: {:?}", cwd);
// 
//     let patches_dir = cwd.join(".github/patches");
//     let doc_dir = cwd.join("target/doc/catalyzer");
//     let patch_file_name = patches_dir.join(format!("{file_name}.patch"));
//     let unpatched_file_name = doc_dir.join(format!("{file_name}"));
// 
// 
//     let patch_file = File::options()
//         .read(true)
//         .write(false)
//         .open(patch_file_name)?;
//     let unpatched_file = File::options()
//         .read(false)
//         .write(true)
//         .create(true)
//         .open(unpatched_file_name)?;
//     let mut patch = io::BufReader::new(patch_file);
//     let mut temp = Vec::new();
//     let mut unpatched = io::BufWriter::new(unpatched_file);
//     for line in 0..patch.buffer().len() {
//         patch.read_until(b'\n', &mut temp)?;
//         if let Some(l) = l {
//             if line == l {
//                 unpatched.write_all(&temp)?;
//             }
//         } else {
//             unpatched.write_all(&temp)?;
//             unpatched.flush()?;
//         }
//     }
//     Ok(())
// }
