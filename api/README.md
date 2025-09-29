# API

This crate contains all shared fullstack server functions. Server functions use procedural macros to compile the server functions so that the frontend only sees the stubs (function signatures). That way, only the input and output types of the server functions need to compile to WASM.

This crate will be built twice:

1. Once for the server build with the `dioxus/server` feature enabled
2. Once for the client build with the client feature disabled

During the server build, the server functions will be collected and hosted on a public API for the client to call. During the client build, the server functions will be compiled into the client build.

## Dependencies

Most server dependencies (like sqlx and tokio) will not compile on client platforms like WASM. To avoid building server dependencies on the client, this project uses a separate server-only crate with [`backend`](../backend/) so the split between backend and frontend is clearer and only the `backend` crate needs to be conditionally included as a dependency.
