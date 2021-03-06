use config::Config;
use std::collections::HashMap;
use std::io::prelude::*;
use std::path::PathBuf;
use std::env;

fn main() {
    // Grab OPTIX_ROOT and CUDA_ROOT from build-settings.toml
    // or from the environment
    let mut settings = Config::default();

    settings
        .merge(config::File::with_name("build-settings").required(false))
        .unwrap();
    settings.merge(config::Environment::new()).ok();

    let settings_map = settings
        .try_into::<HashMap<String, String>>()
        .unwrap_or(HashMap::new());

    let optix_root = settings_map
        .get("optix_root")
        .expect("OPTIX_ROOT not found. You must set OPTIX_ROOT either as an environment variable, or in build-settings.toml to point to the root of your OptiX installation.");

    let cuda_root = settings_map.get("cuda_root")
        .expect("CUDA_ROOT not found. You must set CUDA_ROOT either as an environment variable, or in build-settings.toml to point to the root of your CUDA installation.");

    let dst = cmake::Config::new("cuda")
        .define("OPTIX_ROOT", optix_root)
        .define("CUDA_ROOT", cuda_root)
        .build();

    // create a temporary file to tell tests and examples where the cmake-generated ptx
    // has ended up
    let ptx_path = format!("ptx_path = \"{}/ptx\"", dst.display());
    let mut file = std::fs::File::create(std::path::Path::new("ptx_path.toml")).unwrap();
    file.write_all(ptx_path.as_bytes()).unwrap();

    println!("cargo:rustc-link-lib=dylib=optix");
    println!("cargo:rustc-link-search=native={}/lib64", optix_root);
}
