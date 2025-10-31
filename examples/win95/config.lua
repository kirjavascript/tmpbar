local ui = require('ui')
local wm = require('wm')
local sys = require('sys')
local util = require('util')

-- local grey = '#BDB8BE'
-- local darkgrey = '#7f787f'

local grey = '#C0C0C0'
local darkgrey = '#808080'
local lightgrey = '#DFDFDF'
local black = '#000000'
local white = '#FFFFFF'

ui.load_font('MSGothic', './fonts/msgothic-cjk.ttf')
ui.load_font('W95FA', './fonts/w95fa/W95FA.otf')

local function win95_border(raised)
    return function(svg)
        local outer_top = raised and white or darkgrey
        local outer_bottom = raised and darkgrey or white
        local inner_top = raised and lightgrey or black
        local inner_bottom = raised and black or lightgrey

        return string.format([[
            <rect width="%d" height="%d" fill="%s" />
            <rect x="0" y="0" width="%d" height="1" fill="%s" />
            <rect x="0" y="0" width="1" height="%d" fill="%s" />
            <rect x="0" y="%d" width="%d" height="1" fill="%s" />
            <rect x="%d" y="0" width="1" height="%d" fill="%s" />
            <rect x="1" y="1" width="%d" height="1" fill="%s" />
            <rect x="1" y="1" width="1" height="%d" fill="%s" />
            <rect x="1" y="%d" width="%d" height="1" fill="%s" />
            <rect x="%d" y="1" width="1" height="%d" fill="%s" />
        ]],
        svg.width, svg.height, grey,
        svg.width, outer_top,
        svg.height, outer_top,
        svg.height - 1, svg.width, outer_bottom,
        svg.width - 1, svg.height, outer_bottom,
        svg.width - 2, inner_top,
        svg.height - 2, inner_top,
        svg.height - 2, svg.width - 2, inner_bottom,
        svg.width - 2, svg.height - 2, inner_bottom)
    end
end

for _, monitor in ui.monitors() do
    ui.bar({
        monitor = monitor,
        position = 'bottom',

        style = {
            height = 28,
            color = black,
            background_color = grey,
            font_family = 'W95FA',
            font_size = 11,
            padding = 2,
            justify_content = 'space_between',
            background = win95_border(true),
        },

        items = {
            ui.container({
                items = {
                    ui.label({
                        style = {
                            height = 22,
                            width = 54,
                            padding = 0,
                            margin_right = 4,
                            font_family = 'W95FA',
                            font_size = 11,
                            font_weight = 'bold',
                            text_align = 'center',
                            text_valign = 'center',
                            background = function(svg)
                                return win95_border(not ui.local_mousedown())(svg)
                            end,
                        },
                        text = 'Start',
                        click = function()
                            -- Could integrate with rofi/dmenu here
                            sys.exec('rofi -show drun')
                        end,

                    }),

                    ui.container({
                        style = {
                            flex_direction = 'row',
                            gap = 2,
                            height = 22,
                            flex = 1,
                        },
                        items = function()
                            local windows = {}
                            for _, window in ipairs(wm.windows()) do
                                if window.title and window.title ~= '' then
                                    table.insert(windows, ui.button({
                                        style = {
                                            height = 22,
                                            min_width = 120,
                                            max_width = 160,
                                            padding_left = 4,
                                            padding_right = 4,
                                            background = win95_border(not window.focused),
                                            font_family = 'W95FA',
                                            font_size = 11,
                                            text_align = 'left',
                                            text_valign = 'center',
                                            overflow = 'hidden',
                                        },
                                        text = util.truncate(window.title, 20),
                                        click = function()
                                            wm.focus_window(window.id)
                                        end,
                                    }))
                                end
                            end
                            return windows
                        end,
                    }),
                },
            }),

            ui.container({
                style = {
                    height = 22,
                    background = win95_border(false),
                    padding = 2,
                    gap = 8,
                    align_items = 'center',
                },
                items = {
                    ui.tray({
                        style = {
                            background_color = grey,
                            height = 16,
                            padding_right = 4,
                        },
                    }),

                    ui.label({
                        style = {
                            font_family = 'W95FA',
                            font_size = 11,
                            padding_left = 4,
                            padding_right = 4,
                            text_align = 'center',
                            text_valign = 'center',
                        },
                        text = function()
                            return os.date('%I:%M %p')
                        end,
                    }),
                },
            }),
        }
    })
end
