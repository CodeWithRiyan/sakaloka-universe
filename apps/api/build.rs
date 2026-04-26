use std::path::Path;

fn main() {
    let version_file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("VERSION");

    if version_file.exists() {
        let version = std::fs::read_to_string(&version_file)
            .expect("failed to read VERSION")
            .trim()
            .to_string();
        println!("cargo:rustc-env=SAKALOKA_VERSION={version}");
    } else {
        println!("cargo:rustc-env=SAKALOKA_VERSION=0.0.0.0");
    }

    println!("cargo:rerun-if-changed={}", version_file.display());
}
