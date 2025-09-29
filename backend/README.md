# Development

This crate defines a library of utility functions that are mainly used in server functions defined in `api`. This ensures a clean split between the server functions, posing as the interface between the frontend and the backend, and any logic that needs to happen in the backend. This is not strictly necessary, but it allows more fine-grained structuring, and it should hopefully make it easier in the future to change implementation details without affecting the interface. 

```
backend/
├─ src/
│  ├─ config.rs # handles creating, reading and modifying the config file of this app
│  ├─ database.rs # handles access to the database containing all the metadata and more
│  ├─ lib.rs # The entrypoint for the library, defines modules
│  ├─ schema.sql # contains the database schema, used in database.rs to avoid having all queries in multi-line strings
├─ Cargo.toml # The backend crate's Cargo.toml

```
