use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

// This should match the version used by pg_parse crate
// You can configure this via environment variable PG_QUERY_VERSION if needed
static LIBPG_QUERY_TAG: &str = "17-6.1.0";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Allow version override via environment variable
    let version = env::var("PG_QUERY_VERSION").unwrap_or_else(|_| LIBPG_QUERY_TAG.to_string());

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let vendor_dir = out_dir.join("vendor");
    let proto_dir = vendor_dir.join("libpg_query").join(&version);
    let proto_path = proto_dir.join("pg_query.proto");
    let stamp_file = proto_dir.join(".stamp");

    // Download proto file if not already present
    if !stamp_file.exists() {
        println!(
            "cargo:warning=Downloading pg_query.proto for libpg_query {}",
            version
        );

        // Create directories
        fs::create_dir_all(&proto_dir)?;

        // Download the proto file
        let proto_url = format!(
            "https://raw.githubusercontent.com/pganalyze/libpg_query/{}/protobuf/pg_query.proto",
            version
        );

        let response = ureq::get(&proto_url).call()?;
        let proto_content = response.into_string()?;

        // Write proto file
        let mut file = fs::File::create(&proto_path)?;
        file.write_all(proto_content.as_bytes())?;

        // Create stamp file
        fs::File::create(&stamp_file)?;

        println!("cargo:warning=Successfully downloaded pg_query.proto");
    }

    // Set environment variable for the proc macro
    println!(
        "cargo:rustc-env=PG_QUERY_PROTO_PATH={}",
        proto_path.display()
    );

    // Tell cargo to rerun if the stamp file changes
    println!("cargo:rerun-if-changed={}", stamp_file.display());

    Ok(())
}

