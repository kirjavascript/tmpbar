-- TODO
-- exec('xwinwrap') + windows-live.mp4 + netscape
-- animated SVG for battery monitor, CPU graph
--
-- input: pressing enter doesnt unfocus properly
-- button/input style / behaviour
-- click on everything
--
-- volume
-- battery sys
--  https://github.com/svartalf/rust-battery
-- cpu% sys

local ui = require('ui')
local wm = require('wm')
local sys = require('sys')
local util = require('util')

for monitor_index, monitor in ui.monitors() do
    ui.bar({
        monitor = monitor,
        position = 'top',

        style = {
            height = 130,
            justify_items = 'flex_end',
            justify_content = 'space_between',
            -- align_items = 'flex_end',
            -- flex_direction = 'column',
            -- postion = 'absolute',
            color = 'pink',
            background = function(svg) return string.format([[
                <rect
                    x="0"
                    y="0"
                    width="%d"
                    height="%d"
                    rx="5"
                    fill="none"
                    stroke="black"
                    stroke-width="5"
                />
            ]], svg.width, svg.height) end,
        },
        -- min_interval = 1000

        items = {
            ui.label({
                style = {
                    background = 'yellow',
                    color = 'black',
                    width = 150,
                    text_align = 'max',
                    text_valign = 'max',
                    font_size = 55,
                },
                text = 'JS',
            }),
            ui.label({ -- window title
                style = {
                    position = 'absolute',
                    margin = 'auto',
                },
                text = function() return '« ' .. util.truncate(wm.window_title(), 80) .. ' »' end,
            }),
            ui.label({ -- i3 mode
                style = {
                    position = 'absolute',
                    margin_left = 'auto',
                    margin_right = 'auto',
                    size = 'auto',
                    bottom = 3,
                    background_color = function()
                        local is_default = wm.i3_mode() == 'default'
                        return is_default and 'transparent' or 'darkred'
                    end,
                },
                text = function()
                    local mode = wm.i3_mode()
                    return (mode == 'default') and '' or (' « ' .. mode .. ' » ')
                end,
            }),
            ui.container({
                style = {
                    flex_direction = 'column',
                },
                items = {
                    ui.image({
                        path = 'https://sharey.org/8ilgDQ.png',
                        style = { padding = 3, size = 60, },
                    }),
                    ui.image({
                        path = './assets/archlinux.svg',
                        style = { size = 100 },
                    }),
                },
            }),
            ui.workspaces({
                render = function (workspace) return ui.label({
                        style = {
                            width = '20',
                            background_color = function()
                                return workspace.urgent and 'red'
                                    or workspace.focused and '#0A83FD'
                                    or workspace.visible and '#0022CC'
                                    or 'black'
                            end,
                        },
                        text = tostring(workspace.name):sub(1, 1),
                        click = function()
                            wm.set_workspace(workspace.number)
                        end,
                }) end
            }),
            ui.container({
                style = {
                },
                items = {
                    ui.label({ -- clock
                        text = function() return os.date('%Y-%m-%d %a %X') end,
                    }),
                    ui.button({
                        text = 'shutdown',
                        size = 100,
                        click = function() return sys.spawn('~/.config/i3/scripts/powermenu') end
                    }),

                    ui.button({
                        text = 'activate',
                        size = 100,
                        click = function() return sys.spawn('activate-linux') end
                    }),
                    monitor_index == 1 and ui.tray({
                        style = {
                            background_color = '#0A3A77',
                        },
                    }),
                },
            }),
        },
    })
end
