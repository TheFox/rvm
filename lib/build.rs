
use std::env::var_os;
use std::fs::write;
use std::path::Path;

fn main() {
    let content = format!(
r#"
const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
"#);

    let out_dir = var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("config.rs");
    write(&dest_path, content).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
