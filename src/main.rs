use rust_project_sandbox::parse_config;
use std::path::PathBuf;

fn main() {
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(".github")
        .join("workflows")
        .join("rust_project_matrix.json");

    match parse_config(&config_path, false) {
        Some(config) => {
            println!("{}", serde_json::to_string_pretty(&config).unwrap());
        }
        None => {
            std::process::exit(1);
        }
    }
}
