# Development

The web crate defines the entry point for the web app. It may use arbitrary dependencies, but since the crate will be built once for the native target (for the backend) and for WASM (for the frontend), it is absolutely vital to make sure non-compatible crates are only included for the backend via `#[cfg(feature="server")]`. This includes indirect dependencies, e.g., we have a dependency `web`->`api` and `api`->`backend`, the last of which contains dependencies for SQLite. As such, the `api` crate uses the `server` feature flag when being compiled for the native OS.

The structure of the crate is as follows:

```
web/
├─ src/
│  └─ main.rs # The entrypoint for the web app.It also defines the routes for the web platform
└─ Cargo.toml # The web crate's Cargo.toml - This should include all web specific dependencies
```

Because there is currently no UI element that is specific to any target, every component is defined in the `ui` crate and retrieved via the dependency.

## Dependencies

Since this is a fullstack app, the web crate will be built two times:

1. Once for the server build with the `server` feature enabled
2. Once for the client build with the `web` feature enabled

### Serving the Web App

The web app is started with the following command, ran from the workspace level:

```bash
dx serve --package web
