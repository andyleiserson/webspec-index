use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=data/w3c_specs.json");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("w3c_specs.rs");

    let data_path = Path::new("data/w3c_specs.json");
    if !data_path.exists() {
        println!("cargo:warning=data/w3c_specs.json not found; W3C specs will be empty.");
        println!("cargo:warning=Run: python3 tools/gen_w3c_specs.py --clone");
        println!("cargo:warning=  or: webspec-index update-spec-list");
        fs::write(dest, "pub const W3C_SPECS: &[SpecInfo] = &[];\n").unwrap();
        return;
    }

    let data = fs::read_to_string(data_path).expect("Failed to read data/w3c_specs.json");
    let specs: Vec<serde_json::Value> =
        serde_json::from_str(&data).expect("Failed to parse data/w3c_specs.json");

    let mut code = String::from("pub const W3C_SPECS: &[SpecInfo] = &[\n");
    for spec in &specs {
        let name = spec["name"].as_str().unwrap_or("");
        let base_url = spec["base_url"].as_str().unwrap_or("");
        let provider = spec["provider"].as_str().unwrap_or("w3c");
        let github_repo = spec["github_repo"].as_str().unwrap_or("");
        code.push_str(&format!(
            "    SpecInfo {{ name: {name:?}, base_url: {base_url:?}, provider: {provider:?}, github_repo: {github_repo:?} }},\n"
        ));
    }
    code.push_str("];\n");

    fs::write(dest, code).expect("Failed to write w3c_specs.rs");
}
