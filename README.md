# rust-project-sandbox

An experimental rust project to test out various tools.

## Setup

```bash
# Clone the repository
git clone https://github.com/7rikazhexde/rust-project-sandbox.git

# Rust toolchain will be automatically installed based on rust-toolchain.toml
# Build the project
cargo build
```

## Usage

Run the JSON parser

```bash
cargo run
```

Run tests

```bash
cargo test

# Run tests with output
cargo test -- --nocapture
```

## JSON Configuration Format

Plaese create `.github/workflows/rust_project_matrix.json`

```json
{
    "os": [
        "ubuntu-latest",
        "windows-latest",
        "macos-latest"
    ],
    "versions": {
        "rust": [
            "1.79.0",
            "1.80.0",
            "1.81.0",
            "1.82.0",
            "stable"
        ]
    },
    "ghpages_branch": "ghgapes"
}
```
