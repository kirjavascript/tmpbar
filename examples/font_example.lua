local ui = require('ui')

ui.add_font("CyberBlast", "./CyberBlast.otf")
ui.add_font("Hack", "./Hack-Regular.ttf")

for _, monitor in ui.monitors() do
    ui.bar({
        monitor = monitor,
        position = "top",

        style = {
            height = 60,
            justify_content = "space_around",
            background_color = "black",
            color = "red",
            padding = 10,
        },

        items = {
            ui.label({
                text = "Hello with CyberBlast",
                style = {
                    font_family = "CyberBlast",
                    font_size = 40,
                    color = "red"
                }
            }),

            ui.label({
                text = "Hello with Hack",
                style = {
                    font_family = "Hack",
                    font_size = 16,
                    color = "rebeccapurple"
                }
            }),
            ui.label({
                text = "Default font with default color",
                style = {
                    font_size = 14
                }
            }),
            ui.label({
                text = "Monospace font",
                style = {
                    font_family = "monospace",
                    color = "darkred",
                    font_size = 18
                }
            }),
            ui.label({
                text = "Large custom font",
                style = {
                    font_family = "Hack",
                    color = "#00FF00",
                    font_size = 24
                }
            })
        },
    })
end
