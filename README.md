# tmpbar

customizable statusbar for your windowmanager

* lua config
* flexbox layout
* image, vector, or shader graphics
* system tray integration
* hot config reloading
* windowmanager neutral config
* multibar/multimonitor support
* more rice than feudal japan

tmpbar is built with egui and XCB

## quickstart

```bash
git clone https://github.com/kirjavascript/tmpbar.git
cd tmpbar
cargo run --release -- -c examples/darkblue/config.lua
```

view the [examples](./examples)

### notes

this project is a reimagining of [another bar](https://github.com/kirjavascript/cakeybar). tmpbar has much stronger primitives

wayland support is missing but not a huge undertaking to add
