local ui = require('ui')

local grey = '#BDB8BE'
local darkgrey = '#7f787f'

ui.load_font('MSGothic', './fonts/msgothic-cjk.ttf')
ui.load_font('W95FA', './fonts/w95fa/W95FA.otf')

for _, monitor in ui.monitors() do
    ui.bar({
        monitor = monitor,
        position = 'bottom',

        style = {
            height = 28,
            background_color = grey,
            font_family = 'W95FA',
            font_size = 14,
            color = 'black',
        },

        items = {
            ui.label({
                text = 'Start',
            })
        }
    })
end
