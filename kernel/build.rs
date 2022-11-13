// Import Dependencies
use std::path::Path;
// Define Build Script
fn main() {
    // Define linker script path
    let target = std::env::var("TARGET")
        .ok()
        .and_then(|target| Some(target.split("-").into_iter().take(1).next()?.to_string()))
        .unwrap();
    let linker_script = Path::new("link").join(target.clone()).with_extension("ld");
    if !linker_script.exists() {
        panic!("Invalid architecture: {}", target)
    }
    // Apply linker
    println!(
        "cargo:rustc-link-arg-bins=-T{}",
        linker_script
            .canonicalize()
            .ok()
            .and_then(|path| Some(path.to_str()?.to_string()))
            .unwrap()
    );
    println!("cargo:rerun-if-changed=build.rs");
}
