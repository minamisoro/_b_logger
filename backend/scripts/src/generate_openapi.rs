use blogger_api::openapi::ApiDoc;
use std::fs;
use std::path::PathBuf;
use utoipa::OpenApi;

fn main() {
    // Generate OpenAPI specification
    let openapi = ApiDoc::openapi();

    // Convert to JSON
    let json = serde_json::to_string_pretty(&openapi)
        .expect("Failed to serialize OpenAPI spec to JSON");

    // Determine output path: frontend/lib/swagger.json
    let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("frontend")
        .join("lib");

    // Create the lib directory if it doesn't exist
    fs::create_dir_all(&output_dir)
        .expect("Failed to create frontend/lib directory");

    let output_path = output_dir.join("swagger.json");

    // Write the JSON to file
    fs::write(&output_path, json)
        .expect("Failed to write swagger.json");

    println!("✅ OpenAPI specification generated successfully!");
    println!("📄 Output: {}", output_path.display());
}
