local ui = require('ui')

xcake_load_font("Hack", "./Hack-Regular.ttf")
xcake_load_font("CyberBlast", "./CyberBlast.otf")
xcake_load_font("ByteBounce", "./ByteBounce.ttf")



for _, monitor in ui.monitors() do
    ui.bar({
        monitor = monitor,
        position = "top",
        height = 60,

        style = {
            size = "max",
            justify_content = "space_around",
            background_color = "black",
            padding = 10,
        },

        items = {
            ui.label({
                text = "Hello with Hack font!",
                style = {
                    font_family = "Hack",
                    font_size = 16,
                    color = "red"
                }
            }),

            ui.label({
                text = "Hello with Hack font!",
                style = {
                    font_family = "Hack",
                    font_size = 16,
                    color = "red"
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
