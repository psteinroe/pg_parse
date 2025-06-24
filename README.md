# pg_parse

Alternative Rust binding for libpg_query. It uses the actual Postgres server source to parse SQL queries and return the internal parse tree.

## Features
- *AST*: Parses Postgres queries into an abstract syntax tree (AST)
- *Multi-version*: Supports multiple Postgres versions at build time
- *Deparse*: Convert an AST back to the SQL string
- *Fingerprint*: Fingerprints a given SQL statement
- *Normalize*: Normalizes the given SQL statement, returning a parametized version
- *Scan*: Lexes the given SQL statement into tokens
- *Split*: Split a query into separate statements

## Why?

There already is an official Rust binding for libpg_query, so why creating a new one? We wanted a few missing features:
- *Multi-version support*: This library can be built for different Postgres versions (15, 16, 17).
- *WASM support*: You can use this library and still build your application to WASM using the `wasm32-unknown-emscripten` target. You can find a full example in `wasm_example/`. We run a build in the CI to make sure it remains compatible.
- *Macro-based iterators*: The official Rust binding implements the iterator for AST nodes manually and therefore misses a large part. This implementation uses the `.proto` definition to generate the code at build time using procedural macros.

> This is *not* a drop-in replacement for the official Rust binding. The generated iterators allows the user to implement a large part of the `ParseResult` API themselves easily. This library just exposes a minimal API meant to be extended on depending on the individual use case.

## Related

- [libpg_query](https://github.com/pganalyze/libpg_query): C library for accessing the PostgreSQL parser outside of the server.
- [pg_query.rs](https://github.com/pganalyze/pg_query.rs): Official Rust binding
- [pg-parser](https://github.com/supabase-community/pg-parser): WASM-based Node binding from which I learned a lot

