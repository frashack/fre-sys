use std::env;
use std::path::PathBuf;

mod var_names {
    pub const AIR_SDK: &'static str = "AIR_HOME";
    pub const ARCH: &'static str = "CARGO_CFG_TARGET_ARCH";
}

fn main() {
    if env::var("DOCS_RS").is_ok() {return;}
    let air_sdk = PathBuf::from(
        env::var(var_names::AIR_SDK)
            .expect(&format!("{} not set", var_names::AIR_SDK))
    );
    let arch = env::var(var_names::ARCH).unwrap();
    
    let relative = match arch.as_str() {
        "x86" => "win",
        "x86_64" => "win64",
        // todo!
        _ => panic!("Unsupported arch: {}", arch),
    };
    let dir = air_sdk   
        .join("lib")
        .join(relative);

    println!("cargo:rustc-link-search=native={}", dir.display());
    println!("cargo:rustc-link-lib=FlashRuntimeExtensions");
}