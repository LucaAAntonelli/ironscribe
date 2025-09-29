# UI

This crate contains all shared components for the workspace. Any UI component that is not platform-specific goes in here.

```
ui/
├─ assets/
│  ├─ favicon.ico # The icon that is used, e.g., for the native app icon or the tab in the browser
│  └─ main.css # CSS definitions for the UI components
└─ src/
   ├─ lib.rs # The entrypoint for the ui crate
   ├─ books.rs # The book table component
   ├─ app.rs # The main app component, this is what's launched when the app starts (web and native)
   └─ path_picker.rs # The component that is launched when no library path has been set in the config
```

## Dependencies

Since this crate is shared between multiple platforms, it should not pull in any platform specific dependencies. Instead, pure frontend dependencies like `web-sys` should be added to the `web` crate as a frontend-only dependency.
