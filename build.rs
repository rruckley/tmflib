//! Build script for tmflib.
//!
//! Generates Rust model code at build time from the TMF OAS specifications
//! in `open_api/`, using the `tmf-gen` generator. Only modules whose cargo
//! feature is enabled (and which have a configuration in `tmf-gen`) are
//! generated into `OUT_DIR`; `src/lib.rs` includes them with `include!`.

use std::env;
use std::path::Path;

use tmf_gen::{config_for_feature, generate_spec_file, write_output, SUPPORTED_MODULES};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");

    // Opt into precise re-build tracking: re-run when the build script,
    // the specs, or the generator change.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=open_api/");
    println!("cargo:rerun-if-changed=tmf-gen/src/");

    let open_api_dir = Path::new(&manifest_dir).join("open_api");
    let out_dir = Path::new(&out_dir);

    for feature in SUPPORTED_MODULES {
        let feature_env = format!("CARGO_FEATURE_{}", feature.to_uppercase());
        if env::var_os(&feature_env).is_none() {
            continue;
        }
        let config = config_for_feature(feature)
            .unwrap_or_else(|| panic!("feature {feature} enabled but no generator config"));
        let spec_file = config.spec_file.as_ref().unwrap_or_else(|| {
            panic!("feature {feature} enabled but config has no spec file")
        });
        let spec_path = open_api_dir.join(spec_file);
        println!("cargo:rerun-if-changed={}", spec_path.display());

        match generate_spec_file(&spec_path, &config) {
            Ok(code) => {
                if let Err(e) = write_output(out_dir, &config, &code) {
                    panic!("failed to write generated {feature}: {e}");
                }
            }
            Err(e) => panic!("failed to generate {feature} from {}: {e}", spec_path.display()),
        }
    }
}
