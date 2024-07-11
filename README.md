# [bug] WebviewWindow freezes/loads infinite on creation inside js if tauri-plugin-devtools are inside Cargo.toml

[Issue #10256](https://github.com/tauri-apps/tauri/issues/10256)

i try to create a new WebviewWindow from JS Code but every time it just loads infinite and i need to force quit the Program.

I figured out that it has something to do with the `tauri-plugin-devtools`. If i remove it from the Cargo.toml it works.

Cargo.toml
```toml
[package]
name = "app"
version = "0.1.0"
description = "A Tauri App"
authors = ["you"]
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[build-dependencies]
tauri-build = { version = "2.0.0-beta.18", features = [] }

[dependencies]
tauri = { version = "2.0.0-beta", features = ["devtools"] }
tauri-plugin-devtools = "2.0.0-beta" # <- Removing this makes it work.
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```
main.rs
```rust 
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let builder = tauri::Builder::default();

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

```js
    import {WebviewWindow} from "@tauri-apps/api/webviewWindow"

    function openWindow() {
        const pileWindow = new WebviewWindow('my-label', {
            url: 'https://github.com/tauri-apps/tauri'
        });

        pileWindow.once('tauri://created', function () {
            console.log('Success');
        });
        pileWindow.once('tauri://error', function (e) {
            console.log(e);
        });
    }
```
