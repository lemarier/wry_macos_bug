# wry macOS bug

When winit is launched, the webview is not created till we resize the window.

Works fine on Windows.

### how to reproduce`
- Clone this repo
- `cargo run --bin webview` on macOS to see blocking webview
- `cargo run --bin window_only` is a winit window showing that it's not blocking