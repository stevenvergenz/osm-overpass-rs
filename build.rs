use std::{fs::File, io::Write, path::PathBuf};

fn main() {
    println!("cargo::rerun-if-changed=Cargo.toml");
    println!("cargo::rerun-if-changed=src/lib.rs");

    let r = cargo_readme::generate_readme(
        &PathBuf::from("."),
        &mut File::open("src/lib.rs").unwrap(),
        None,
        true, false, true, true,
    );
    match r {
        Ok(text) => {
            let mut f = File::create("./README.md").expect("Failed to create README.md");
            f.write_all(text.as_bytes()).expect("Failed to write README.md");
        },
        Err(e) => eprintln!("{e}"),
    }
}