use std::{fs::{File, read_to_string}, io::{Read, Write}, path::PathBuf};

fn main() {
    println!("cargo::rerun-if-changed=Cargo.toml");
    println!("cargo::rerun-if-changed=src/lib.rs");

    let new_text = cargo_readme::generate_readme(
        &PathBuf::from("."),
        &mut File::open("src/lib.rs").unwrap(),
        None,
        true, true, true, true,
    ).expect("Failed to generate new readme text");

    let old_text = read_to_string("README.md").expect("Failed to open old readme");

    if std::env::var("DOCS_RS").is_err() && new_text != old_text {
        File::create("README.md").expect("Failed to create new readme")
            .write_all(new_text.as_bytes()).expect("Failed to write new readme");
    }
}
