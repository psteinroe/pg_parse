#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

use fs_extra::dir::CopyOptions;
use glob::glob;
use std::env;
use std::path::PathBuf;
use std::process::Command;

static LIBRARY_NAME: &str = "pg_query";
static LIBPG_QUERY_REPO: &str = "https://github.com/pganalyze/libpg_query.git";
static LIBPG_QUERY_TAG: &str = "17-6.1.0"; // You can make this configurable later

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let vendor_dir = out_dir.join("vendor");
    let libpg_query_dir = vendor_dir.join("libpg_query").join(LIBPG_QUERY_TAG);
    let stamp_file = libpg_query_dir.join(".stamp");

    let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("src");
    let target = env::var("TARGET").unwrap();
    let is_emscripten = target.contains("emscripten");

    // Configure cargo through stdout
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static={LIBRARY_NAME}");

    // Clone libpg_query if not already present
    if !stamp_file.exists() {
        println!("cargo:warning=Cloning libpg_query {}", LIBPG_QUERY_TAG);

        // Create vendor directory
        std::fs::create_dir_all(&vendor_dir)?;

        // Clone the repository
        let status = Command::new("git")
            .args([
                "clone",
                "--depth",
                "1",
                "--branch",
                LIBPG_QUERY_TAG,
                LIBPG_QUERY_REPO,
                libpg_query_dir.to_str().unwrap(),
            ])
            .status()?;

        if !status.success() {
            return Err("Failed to clone libpg_query".into());
        }

        // Create stamp file
        std::fs::File::create(&stamp_file)?;
    }

    // Tell cargo to rerun if the stamp file is deleted
    println!("cargo:rerun-if-changed={}", stamp_file.display());

    // Copy necessary files to OUT_DIR for compilation
    let out_header_path = out_dir.join(LIBRARY_NAME).with_extension("h");
    let out_protobuf_path = out_dir.join("protobuf");

    let source_paths = vec![
        libpg_query_dir.join(LIBRARY_NAME).with_extension("h"),
        libpg_query_dir.join("Makefile"),
        libpg_query_dir.join("src"),
        libpg_query_dir.join("protobuf"),
        libpg_query_dir.join("vendor"),
    ];

    let copy_options = CopyOptions {
        overwrite: true,
        ..CopyOptions::default()
    };

    fs_extra::copy_items(&source_paths, &out_dir, &copy_options)?;

    // Compile the C library.
    let mut build = cc::Build::new();

    // Configure for Emscripten if needed
    if is_emscripten {
        // Use emcc as the compiler instead of gcc/clang
        build.compiler("emcc");
        // Use emar as the archiver instead of ar
        build.archiver("emar");
        // Note: We don't add WASM-specific flags here as this creates a static library
        // The final linking flags should be added when building the final WASM module
    }

    build
        .files(
            glob(out_dir.join("src/*.c").to_str().unwrap())
                .unwrap()
                .map(|p| p.unwrap()),
        )
        .files(
            glob(out_dir.join("src/postgres/*.c").to_str().unwrap())
                .unwrap()
                .map(|p| p.unwrap()),
        )
        .file(out_dir.join("vendor/protobuf-c/protobuf-c.c"))
        .file(out_dir.join("vendor/xxhash/xxhash.c"))
        .file(out_dir.join("protobuf/pg_query.pb-c.c"))
        .include(out_dir.join("."))
        .include(out_dir.join("./vendor"))
        .include(out_dir.join("./src/postgres/include"))
        .include(out_dir.join("./src/include"))
        .warnings(false); // Avoid unnecessary warnings, as they are already considered as part of libpg_query development
    if env::var("PROFILE").unwrap() == "debug" || env::var("DEBUG").unwrap() == "1" {
        build.define("USE_ASSERT_CHECKING", None);
    }
    if target.contains("windows") && !is_emscripten {
        build.include(out_dir.join("./src/postgres/include/port/win32"));
        if target.contains("msvc") {
            build.include(out_dir.join("./src/postgres/include/port/win32_msvc"));
        }
    }
    build.compile(LIBRARY_NAME);

    // Generate bindings for Rust
    let mut bindgen_builder = bindgen::Builder::default()
        .header(out_header_path.to_str().ok_or("Invalid header path")?)
        // Disable layout tests as they can fail with different pointer sizes
        .layout_tests(false)
        // Whitelist only the functions we need
        .allowlist_function("pg_query_parse_protobuf")
        .allowlist_function("pg_query_scan")
        .allowlist_function("pg_query_free_protobuf_parse_result")
        .allowlist_function("pg_query_free_scan_result")
        // Whitelist the types used by these functions
        .allowlist_type("PgQueryProtobufParseResult")
        .allowlist_type("PgQueryScanResult")
        .allowlist_type("PgQueryError")
        .allowlist_type("PgQueryProtobuf")
        // Also generate bindings for size_t since it's used in PgQueryProtobuf
        .allowlist_type("size_t");

    // Configure bindgen for Emscripten target
    if is_emscripten {
        // Tell bindgen to generate bindings for the wasm32 target
        bindgen_builder = bindgen_builder.clang_arg("--target=wasm32-unknown-emscripten");

        // Use emcc to get the proper include paths
        let emcc_output = Command::new("emcc").args(["-print-search-dirs"]).output();

        if let Ok(output) = emcc_output {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.starts_with("libraries: =") {
                    let lib_path = line.strip_prefix("libraries: =").unwrap_or("");
                    // Add system includes relative to emcc
                    bindgen_builder = bindgen_builder.clang_arg(format!("-I{}/include", lib_path));
                }
            }
        }

        // Also try the homebrew installation path
        let homebrew_emscripten = PathBuf::from("/opt/homebrew/Cellar/emscripten")
            .join("4.0.10")
            .join("libexec")
            .join("cache")
            .join("sysroot")
            .join("include");
        if homebrew_emscripten.exists() {
            bindgen_builder =
                bindgen_builder.clang_arg(format!("-I{}", homebrew_emscripten.display()));
        }
    }

    bindgen_builder
        .generate()
        .map_err(|_| "Unable to generate bindings")?
        .write_to_file(src_dir.join("bindings.rs"))?;

    let protoc_exists = Command::new("protoc").arg("--version").status().is_ok();
    if protoc_exists {
        println!("generating protobuf bindings");
        // HACK: Set OUT_DIR to src/ so that the generated protobuf file is copied to src/protobuf.rs
        unsafe {
            env::set_var("OUT_DIR", &src_dir);
        }

        prost_build::compile_protos(
            &[&out_protobuf_path.join(LIBRARY_NAME).with_extension("proto")],
            &[&out_protobuf_path],
        )?;

        std::fs::rename(src_dir.join("pg_query.rs"), src_dir.join("protobuf.rs"))?;

        // Reset OUT_DIR to the original value
        unsafe {
            env::set_var("OUT_DIR", &out_dir);
        }
    } else {
        println!("skipping protobuf generation");
    }

    Ok(())
}
