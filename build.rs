use std::env;
use std::path::PathBuf;


const VAR_AIR_SDK: &str = "AIR_HOME";
const SETUP_GUIDE: &str = "For more information, see: https://airsdk.dev/docs/basics/getting-started\n";


fn main() {
    if env::var("DOCS_RS").is_ok() {return;}

    
    let air_sdk = PathBuf::from(
        env::var(VAR_AIR_SDK)
            .expect(&format!(
                "`{VAR_AIR_SDK}` is not set. Set it to the AIR SDK root directory.\n\
                {SETUP_GUIDE}",
            ))
    );
    let dir = air_sdk.join("lib");
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let dir = match os.as_str() {
        "windows" => {
            let dir = match arch.as_str() {
                "x86" => dir.join("win"),
                "x86_64" => dir.join("win64"),
                _ => panic!("Unsupported target architecture: {arch}"),
                    
            };
            check(dir.join("FlashRuntimeExtensions.lib"));
            dir
        },
        //
        // TODO: how to do?
        //
        _ => panic!("Unsupported target operating system: {os}"),
    };


    println!("cargo:rustc-link-search=native={}", dir.display());
    println!("cargo:rustc-link-lib=FlashRuntimeExtensions");
}


fn check (file: PathBuf) {
    assert!(
        file.is_file(),

        "Required file not found: `{}`\n\
        Ensure AIR SDK is correctly installed.\n\
        {SETUP_GUIDE}",

        file.display()
    );
}