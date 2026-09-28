use tmf_gen::{config_for_feature, generate_spec_file};

#[test]
fn generates_tmf628() {
    let config = config_for_feature("tmf628").expect("tmf628 config missing");
    let spec_file = config.spec_file.as_ref().expect("tmf628 spec file missing");
    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("open_api")
        .join(spec_file);
    let code = generate_spec_file(&spec_path, &config)
        .unwrap_or_else(|e| panic!("generation failed: {e}"));

    // Shared base types must not be generated locally.
    assert!(!code.contains("pub struct Extensible {"));
    assert!(!code.contains("pub struct Entity {"));
    assert!(!code.contains("pub struct TimePeriod {"));

    // Shared types must be referenced via crate paths.
    assert!(code.contains("crate::common::extensible::Extensible"));
    assert!(code.contains("crate::TimePeriod"));

    // Composed types must flatten the shared base.
    assert!(code
        .contains("#[serde(flatten)]\n    pub extensible: crate::common::extensible::Extensible"));

    // Managed type wiring.
    assert!(code.contains("const TMF_MODULE: &str = \"performanceManagement\";"));
    assert!(code.contains("pub const CLASS_PATH: &str = \"measurement\";"));
    assert!(code.contains("tmflib_derive::HasId"));
    assert!(code.contains("impl IsAddressable for PerformanceMeasurement"));

    // Doc schema dumps are stripped.
    assert!(!code.contains("<details><summary>JSON schema"));

    // Display and Deref impls are generated.
    assert!(code.contains("impl ::std::fmt::Display for Characteristic"));
    assert!(code.contains("impl ::std::ops::Deref for Characteristic"));

    let out_path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target/tmf628-generated.rs");
    std::fs::create_dir_all(out_path.parent().unwrap()).unwrap();
    std::fs::write(out_path, &code).unwrap();
}
