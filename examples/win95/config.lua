local ui = require('ui')

local grey = '#BDB8BE'
local darkgrey = '#7f787f'

ui.load_font('MSGothic', './fonts/msgothic-cjk.ttf')
ui.load_font('W95FA', './fonts/w95fa/W95FA.otf')

for _, monitor in ui.monitors() do
    ui.bar({
        monitor = monitor,
        position = 'top',

        style = {
            height = 24,
            color = 'black',
            background_color = grey,
            font_family = 'W95FA',
            font_size = 14,
            gap = 10,
            -- justify_content = 'space_between',
        },

        items = {
            ui.label({
                style = {
                    height = 20,
                    width = 100,
                    background_color = 'blue',
                },
            }),
            ui.image({
                path = './old/assets/start.png',
                style = {
                    height = 20,
                },
            }),
            ui.image({
                path = './old/assets/start.png',
                style = {
                    height = 22,
                    -- width = 100,
                    background_color = 'pink',
                },
            }),

            ui.label({
                style = {
                    -- background_image = './old/assets/start.png',
                    font_family = 'MSGothic',
                    font_size = 14,
                },
                text = 'スタート',
            }),

            ui.label({
                text = 'Windows 95 testing...',
            }),
        }
    })
end
