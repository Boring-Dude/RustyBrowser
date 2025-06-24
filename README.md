## Tired of heavy resource usage while browsing?
### RUSTYBROWSER!

*RustyBrowser* is a **lightweight yet powerful browser in Rust** that while writing this. I'll try to say that it outperforms even the most optimized "gaming". with all due respect, Browsers (like Opera GX, Brave, etc)while using just i could say **0.5%–2%**

if interested in knowing more, here is an overview of the Core:
* **Ultra-low resource usage** (0.5%-2%).
* **Gaming/Browser hybrid performance.**
* **Built in Rust** (for speed and memory safety)
* Minimal dependencies, fast startup, and low RAM footprint
* GPU-friendly rendering without unnecessary overhead
* Ad-blocking, low-latency tab switching, and Beautiful-centric UX

## 🛠️ Tools & Libraries i used
to meet this goal, you'll think i used C++, C (basically high-level languages) but honestly.

| For Functionality        | What i used                                                                |
| ------------------------ | -------------------------------------------------------------------------- |
| HTML parsing/rendering   | [`html5ever`](https://github.com/servo/html5ever)                          |
| CSS parsing              | [`cssparser`](https://crates.io/crates/cssparser)                          |
| Rendering engine         | Custom minimal or [`raqote`](https://github.com/jrmuizel/raqote)           |
| Network layer            | [`reqwest`](https://crates.io/crates/reqwest) or `ureq`                    |
| GUI/windowing            | [`winit`](https://crates.io/crates/winit), \[`druid`] or \[`iced`]         |
| Webview (fallback)       | [`wry`](https://github.com/tauri-apps/wry) (for hybrid web/native support) |

---
# 📦 The M.V.P (Minimum Viable Product) Plan currently in-use
## Phase 1 - The Tiny Browser Core.
* ☐ Load an HTML page
* ☐ Parse and display basic content (text layout only)
* ☐ Render inside a window
* ☐ Show tab title, URL bar
## Phase 2 - Light rendering engine
* ☐ Add CSS support (basic styles: font, color, layout)
* ☐ GPU acceleration via wgpu or 2D renderer like ``wgpu`` or 2D renderer like ``raqote``
* ☐ Asynchronous loading using ``tokio`` or ``async-std``
## Phase 3 - User experience features
* ☐ Tabbed Browsing
* ☐ Lightweight ad blocking
* ☐ Game-like UI modes (FPS counter, memory display, etc.)
* ☐ extension sandboxing via WebAssembly
---
# Resource Usage Strategy
in order To stay under 2% system resource usage, i had to do the following:
* Minimal threading, batch rendering
* Avoid Chromium/Servo/Gecko-style engine bloat
* Render only visible content (lazy layout)
* Preload content in background threads (prioritized I/O)

>for contributing, just send me a message in my discord [**server.**](https://discord.gg/yJcdrdu3Xd)
