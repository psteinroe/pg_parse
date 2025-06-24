# pg_parse WebAssembly Example

This crate provides WebAssembly bindings for the `pg_parse` library using Emscripten.

## Overview

This example demonstrates how to use `pg_parse` in WebAssembly using a hybrid approach:

1. **Rust Static Library**: This crate (`pg_parse_wasm`) uses the `pg_parse` library and compiles as a static library for the `wasm32-unknown-emscripten` target
2. **C Wrapper**: A minimal C wrapper (`wasm_wrapper.c`) provides the interface between JavaScript and Rust
3. **Emscripten Linking**: Both libraries are linked together using `emcc` to create the final WASM module

This approach avoids runtime compatibility issues while providing full access to the Rust library's functionality.

## Building for WebAssembly

### Prerequisites

- Docker and docker-compose (for the provided development environment)
- Emscripten SDK (included in the Docker environment)

### Build Commands

```bash
# Build the complete WASM module
just build-complete

# Or build step by step:
just build           # Build Rust static library
just build-complete  # Link with C wrapper
```

### Testing

```bash
# Run the WASM tests
just test
```

## Architecture

### Files

- `src/lib.rs` - Rust WASM bindings crate
- `wasm_wrapper.c` - C wrapper that exposes functions to JavaScript
- `wasm_test.js` - JavaScript test runner
- `justfile` - Build commands

### Exported Functions

The WASM module exports the following function to JavaScript:

- `is_valid_sql(query: string): number` - Validates SQL, returns 1 for valid, 0 for invalid

### Memory Management

The module uses Emscripten's memory management functions:

- `_malloc(size)` - Allocate memory
- `_free(ptr)` - Free memory
- `stringToUTF8(str, ptr, maxBytes)` - Copy JavaScript string to WASM memory
- `lengthBytesUTF8(str)` - Get byte length of UTF-8 string

## Usage Example

```javascript
const createModule = require("./target/pg_parse_wasm.js");

async function validateSQL(sql) {
    const module = await createModule();

    // Allocate memory for the SQL string
    const bytes = module.lengthBytesUTF8(sql) + 1;
    const ptr = module._malloc(bytes);
    module.stringToUTF8(sql, ptr, bytes);

    // Call the validation function
    const isValid = module._is_valid_sql(ptr);

    // Free the memory
    module._free(ptr);

    return isValid === 1;
}

// Usage
validateSQL("SELECT * FROM users").then(isValid => {
    console.log(isValid ? "Valid SQL" : "Invalid SQL");
});
```

## Crate Structure

This is a separate crate (`pg_parse_wasm`) that depends on the main `pg_parse` library:

```toml
[dependencies]
pg_parse = { path = "../crates/pg_parse" }
```

This keeps the main `pg_parse` library clean and focused on pure Rust usage, while this crate provides the WebAssembly interface.

## Technical Details

### Why the Hybrid Approach?

Direct compilation of complex Rust code to WebAssembly can hit runtime compatibility issues, particularly with:
- Complex error handling (Result types)
- String operations
- Panic machinery
- Standard library components

The hybrid approach:
- Compiles the full Rust library as a static library for emscripten
- Uses simple C code as the WASM interface (very reliable with emscripten)
- Avoids WebAssembly table index errors and other runtime issues
- Provides full access to Rust functionality

### Build Process

1. **Rust Static Library**: `cargo build --target wasm32-unknown-emscripten --features wasm --lib`
2. **C Wrapper Compilation**: `emcc wasm_wrapper.c libpg_parse.a libpg_query.a`
3. **WASM Module**: Produces `.wasm` and `.js` files for use in JavaScript

### Dependencies

The WASM build includes:
- Your Rust library (`libpg_parse.a`)
- The underlying PostgreSQL parser (`libpg_query.a`)
- All necessary dependencies compiled for emscripten

## Limitations

- Requires Emscripten environment for building
- Larger file size compared to pure C implementations due to Rust dependencies
- Memory must be manually managed when calling from JavaScript
- Currently exports only basic SQL validation (extensible)

## Extending

To add more functions to the WASM interface:

1. Add the Rust function to the `wasm` module in `lib.rs`
2. Add a corresponding C wrapper function in `wasm_wrapper.c`
3. Export the function in the emcc command in `justfile`
4. Update the JavaScript tests

Example:

```rust
// In lib.rs
#[cfg(feature = "wasm")]
pub mod wasm {
    #[no_mangle]
    pub extern "C" fn pg_parse_get_tables(query: *const c_char) -> *const c_char {
        // Implementation
    }
}
```

```c
// In wasm_wrapper.c
extern char* pg_parse_get_tables(const char* query);

EMSCRIPTEN_KEEPALIVE
char* get_tables(const char* sql) {
    return pg_parse_get_tables(sql);
}
```
