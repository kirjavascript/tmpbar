# tmpbar

customizable statusbar for your windowmanager

* expressive configuration with Lua
* hot config reloading
* windowmanager neutral config
* system tray integration
* multibar/multimonitor support
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
