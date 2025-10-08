local ui = require('ui')

for _, monitor in ui.monitors() do
    ui.bar({
        monitor = monitor,
        position = "top",
        height = 80,

        style = {
            justify_content = "space_between",
            size = "max",
        },

        background = function(svg) return string.format([[
            <rect width="%d" height="%d" fill="green" />
        ]], svg.width, svg.height) end,

        items = {
            ui.label({
                style = {
                    position = "absolute",
                    margin = "auto",
                },
                background = function(svg) return string.format([[
                    <rect width="%d" height="%d" fill="pink" />
                ]], svg.width, svg.height) end,
                text = "centre"
            }),
            ui.container({
                style = { gap = 10 },
                background = function(svg) return string.format([[
                    <rect width="%d" height="%d" fill="red" />
                ]], svg.width, svg.height) end,
                items = {
                    ui.label({ text = "left" }),
                    ui.label({ text = "left2" }),
                    ui.label({ text = "left3" }),
                },
            }),
            ui.container({
                style = {
                    flex_direction = "column",
                    justify_content = "space_around",
                    size = "max",
                },
                background = function(svg) return string.format([[
                    <rect width="%d" height="%d" fill="blue" />
                ]], svg.width, svg.height) end,
                items = {
                    ui.label({ text = "right" }),
                    ui.label({ text = "right2" }),
                    ui.label({ text = "right3" }),
                },
            }),
        },
    })
end
