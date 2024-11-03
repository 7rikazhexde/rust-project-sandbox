use rust_project_sandbox::parse_config;
use std::path::PathBuf;

#[test]
fn test_parse_valid_config() {
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(".github")
        .join("workflows")
        .join("rust_project_matrix.json");

    let config = parse_config(&config_path, false).expect("Failed to parse config");

    assert!(config.os.contains(&"ubuntu-latest".to_string()));
    assert!(config.os.contains(&"windows-latest".to_string()));
    assert!(config.os.contains(&"macos-latest".to_string()));

    let rust_versions = config.versions.get("rust").expect("No Rust versions found");
    assert!(rust_versions.contains(&"1.80.0".to_string()));
    assert!(rust_versions.contains(&"1.81.0".to_string()));
    assert!(rust_versions.contains(&"1.82.0".to_string()));
    assert!(rust_versions.contains(&"stable".to_string()));

    assert_eq!(config.ghpages_branch, "ghgapes");
}
