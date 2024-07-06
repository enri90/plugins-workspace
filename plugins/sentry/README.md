![plugin-upload](https://github.com/tauri-apps/plugins-workspace/raw/v2/plugins/upload/banner.png)

A Tauri Plugin for improved Sentry support.

It's perfectly reasonable to use Sentry's Rust and browser SDKs separately in a
Tauri app. However, this plugin passes browser breadcrumbs and events through 
the Rust backend which has a number of advantages:

- Browser events are enriched with Rust, OS and device context
  - Events from both Rust and browser will have the same context for filtering
- Breadcrumbs are merged from both the Rust and browser SDKs
  - See what was happening in the Rust backend and the browser frontend at the
    time of the event

## Installation

This example also shows usage of
[`sentry_rust_minidump`](https://github.com/timfish/sentry-rust-minidump) which
allows you to capture minidumps for native crashes from a separate crash
reporting process.

Add the required dependencies in `Cargo.toml`:

```toml
[dependencies]
tauri-plugin-sentry = "2.0.0-beta"
# alternatively with Git:
tauri-plugin-sentry = { git = "https://github.com/tauri-apps/plugins-workspace", branch = "v2" }
```

`sentry` and `sentry-rust-minidump` are re-exported by `sentry-tauri` so you
don't need to add them as dependencies.

Run one of these commands to add the capabilities:
- npm: `npm run tauri add autostart`
- yarn: `yarn run tauri add autostart`
- pnpm: `pnpm tauri add autostart`
- cargo: `cargo tauri add autostart`

however, make sure that you have `sentry:default` in your capabilities:

###### src-tauri/capabilities/*.json
```json
{
  "$schema": "./../gen/schemas/windows-schema.json",
  "identifier": "main",
  "local": true,
  "windows": [
    "main"
  ],
  "permissions": [
    "sentry:default" // <- important
  ]
}
```

## Usage
First you need to register the core plugin with Tauri:

`src-tauri/src/main.rs`

```rust
fn main() {
    let dsn = std::env::var("SENTRY_DSN").unwrap_or_else(|_| String::new());

    let client = tauri_plugin_sentry::sentry::init((
      dsn.clone(),
      tauri_plugin_sentry::sentry::ClientOptions {
        debug: true,
        release: tauri_plugin_sentry::sentry::release_name!(),
        ..Default::default()
      },
    ));
    let _guard = tauri_plugin_sentry::minidump::init(&client);

    tauri::Builder::default()
        .plugin(tauri_plugin_upload::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Afterwards all the plugin's APIs are available through the JavaScript guest bindings:

```javascript

    window.Sentry.captureException("error execution", {
      tags: {
        post: 'error',
      },
      extra: { 'test' },
    });

```

## The Plugin

- Injects and initialises `@sentry/browser` in every web-view
- Includes `beforeSend` and `beforeBreadcrumb` hooks that intercept events and breadcrumbs and passes
  them to the Rust SDK via the Tauri `invoke` API
- Tauri + `serde` + existing Sentry Rust types = Deserialisation mostly Just Works™️

## Contributing

PRs accepted. Please make sure to read the Contributing Guide before making a pull request.