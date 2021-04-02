# wry macOS bug

When winit is launched, the webview is not created till we resize the window.

Works fine on Windows.

### how to reproduce`
- Clone this repo
- `cargo run` on macOS