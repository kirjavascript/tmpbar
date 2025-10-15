# tmpbar

customizable statusbar for your windowmanager

* dynamic Lua configuration
* expressive flexbox layout
* hot config reloading
* system tray integration
* multibar/multimonitor support
* windowmanager neutral config
* more rice than feudal japan

tmpbar is written in Rust using egui and XCB

[request a feature or file a bug](https://github.com/kirjavascript/tmpbar/issues)

## quickstart

```bash
git clone https://github.com/kirjavascript/tmpbar.git
cd tmpbar
cargo run --release -- -c examples/darkblue/config.lua
```

view the [examples](./examples)

### notes

this project is a full rewrite of [cakeybar](https://github.com/kirjavascript/cakeybar)
