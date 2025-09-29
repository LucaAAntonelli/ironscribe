# Development

The desktop crate defines the entrypoint for the desktop app along with any assets, components and dependencies that are specific to desktop builds.

```
desktop/
├─ assets/ # Assets used by the desktop app
├─ src/
│  ├─ main.rs # The entrypoint for the desktop app.It also defines the routes for the desktop platform
│  ├─ views/ # The views each route will render in the desktop version of the app
│  │  ├─ mod.rs # Defines the module for the views route and re-exports the components for each route
│  │  ├─ blog.rs # The component that will render at the /blog/:id route
│  │  ├─ home.rs # The component that will render at the / route
├─ Cargo.toml # The desktop crate's Cargo.toml - This should include all desktop specific dependencies
```

## Dependencies

Since this is a fullstack project, the desktop crate will be built two times:

1. Once for the server build with the `server` feature enabled
2. Once for the client build with the `desktop` feature enabled

### Serving Your Desktop App

During development, the app is launched natively with

```bash
dx serve --package desktop

when running from the top-level directory of the project.
