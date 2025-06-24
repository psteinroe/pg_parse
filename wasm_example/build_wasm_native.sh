#!/bin/bash
set -e

mkdir -p target

# Find libpg_query.a
PG_QUERY_LIB=$(find target/wasm32-unknown-emscripten/debug/build -name "libpg_query.a" | head -1)
PG_QUERY_DIR=$(dirname $PG_QUERY_LIB)

echo "Using libpg_query.a from: $PG_QUERY_LIB"

# Build WASM module
emcc wasm_wrapper.c \
    target/wasm32-unknown-emscripten/debug/libpg_parse_wasm.a \
    $PG_QUERY_LIB \
    -I$PG_QUERY_DIR \
    -s EXPORTED_FUNCTIONS="[_is_valid_sql,_malloc,_free]" \
    -s EXPORTED_RUNTIME_METHODS="[ccall,cwrap,lengthBytesUTF8,stringToUTF8,UTF8ToString]" \
    -s MODULARIZE=1 \
    -s EXPORT_NAME=createModule \
    -s ALLOW_MEMORY_GROWTH=1 \
    -O0 \
    -o target/pg_parse_wasm.js