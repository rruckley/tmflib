//! Drift detection: committed generated modules in tmflib's `src/` tree
//! must match a fresh generation from the bundled OAS specs. If this test
//! fails, a spec or the generator changed without re-running:
//!
//!     cargo run -p tmf-gen -- --all --output src

use tmf_gen::{config_for_feature, generate_spec_file, SUPPORTED_MODULES};

#[test]
fn committed_modules_match_generation() {
    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("src");

    for feature in SUPPORTED_MODULES {
        let committed = src_dir.join(format!("{feature}.rs"));
        // Modules not yet onboarded (still hand-maintained or absent) are
        // skipped; they have no committed generated file to compare.
        if !committed.exists() {
            continue;
        }

        let config = config_for_feature(feature)
            .unwrap_or_else(|| panic!("configured module {feature} has no config"));
        let spec_file = config
            .spec_file
            .as_ref()
            .unwrap_or_else(|| panic!("module {feature} has no spec file"));
        let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("open_api")
            .join(spec_file);

        let fresh = generate_spec_file(&spec_path, &config)
            .unwrap_or_else(|e| panic!("generation failed for {feature}: {e}"));
        let committed_code = std::fs::read_to_string(&committed)
            .unwrap_or_else(|e| panic!("could not read {}: {e}", committed.display()));

        assert_eq!(
            committed_code, fresh,
            "committed {feature} module is stale. Re-run: cargo run -p tmf-gen -- --all --output src"
        );
    }
}
