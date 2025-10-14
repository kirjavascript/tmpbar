local ui = require('ui')

for index, monitor in ui.monitors() do
    ui.bar({
        monitor = monitor,
        position = 'top',

        style = {
            height = 80,
            justify_content = 'space_between',
            background = function(svg) return string.format([[
                <defs>
                    <linearGradient id="grad" x1="0%%" y1="0%%" x2="100%%" y2="0%%">
                        <stop offset="2%%" stop-color="green" />
                        <stop offset="70%%" stop-color="orange" />
                    </linearGradient>
                </defs>

                <rect width="%d" height="%d" fill="url(#grad)" />
            ]], svg.width, svg.height) end,
        },

        items = {
            ui.label({
                style = {
                    background = 'rebeccapurple',
                    position = 'absolute',
                    margin = 'auto',
                },
                text = 'centre'
            }),
            ui.container({
                style = {
                    background = 'darkred',
                    gap = 10,
                    padding = 10,
                },
                items = {
                    ui.label({ text = 'left1' }),
                    ui.label({ text = 'left2' }),
                    ui.label({ text = 'left3' }),
                },
            }),
            ui.container({
                style = {
                    background = 'steelblue',
                    flex_direction = 'column',
                    justify_content = 'space_around',
                    height = 'max',
                    width = 200,
                },
                items = {
                    ui.label({ text = 'right2' }),
                    ui.label({ text = 'right', style = { background_color = 'pink' }, }),
                    ui.label({ text = 'right3' }),
                },
            }),
        },
    })
end
