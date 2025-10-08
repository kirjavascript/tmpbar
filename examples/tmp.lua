-- TODO
-- click on everything
-- animated SVG for battery monitor, CPU graph
-- window()
--
-- colours / styles / layout
-- `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
--
-- input: pressing enter doesnt unfocus properly
-- make style property dynamic? make all properties?

-- API improvements: primitives for svg

local ui = require('ui')
local wm = require('wm')
local sys = require('sys')

for monitor_index, monitor in ui.monitors() do
    ui.bar({
        monitor = monitor,
        position = "top",
        height = 130,

        style = {
            display = "flex",
            justify_items = "flex_end",
            justify_content = "space_between",
            -- align_items = "flex_end",
            -- flex_direction = "column",
            -- postion = "absolute",
            size = "max",
        },
        -- max_interval = 1000

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

        items = {
            ui.label({ -- window title
                style = {
                    position = "absolute",
                    margin = "auto",
                },
                text = function() return "« " .. truncate(wm.window_title(), 80) .. " »" end,
            }),
            ui.label({ -- i3 mode
                style = {
                    position = "absolute",
                    margin_left = "auto",
                    margin_right = "auto",
                    size = "auto",
                    bottom = 3,
                },
                text = function()
                    local mode = wm.i3_mode()
                    return (mode == "default") and "" or ("« " .. mode .. " »")
                end,
                background = function(svg)
                    local is_default = wm.i3_mode() == "default"
                    local color = is_default and "transparent" or "darkred"

                    return ([[
                        <rect x="0" y="0" width="%d" height="%d" fill="%s" rx="2"/>
                    ]]):format(svg.width, svg.height, color)
                end,
            }),
            ui.image({
                path = "https://sharey.org/8ilgDQ.png",
                style = { padding = 3, size = "max", },
            }),
            ui.image({
                path = "./archlinux.svg",
                style = { size = "max" },
            }),
            ui.workspaces({
                render = function (workspace) return ui.label({
                        style = { width = "20" },
                        text = tostring(workspace.name):sub(1, 1),
                        background = function(svg)
                            local color = workspace.urgent and "red"
                                or workspace.focused and "#0A83FD"
                                or workspace.visible and "#0022CC"
                                or "black"

                            return ([[
                                <rect x="0" y="0" width="12" height="%d" fill="%s" rx="2"/>
                            ]]):format(svg.height, color)
                        end,
                        click = function()
                            wm.set_workspace(workspace.number)
                        end,
                }) end
            }),
            ui.container({
                items = {
                    ui.label({ -- clock
                        text = function() return os.date("%Y-%m-%d %a %X") end,
                    }),
                    ui.label({ -- network
                        text = function()
                            local bw = sys.bandwidth();
                            return bw.enp3s0 and bw.enp3s0.down
                                or bw.eth0 and bw.eth0.down
                                or '[no interface]'
                        end,
                    }),
                    ui.label({
                        text = function()
                            return 'MEM ' .. sys.memory().used_percent
                        end,
                    }),
                    ui.label({
                        text = function()
                            return sys.cpu_temp() .. '°C'
                        end,
                    }),
                    ui.label({
                        text = function()
                            return sys.disk()['/'].free .. ' free'
                        end,
                    }),
                    ui.button({
                        text = "shutdown",
                        size = 100,
                        click = function() return sys.spawn("~/.config/i3/scripts/powermenu") end
                    }),

                    ui.button({
                        text = "activate",
                        size = 100,
                        click = function() return sys.spawn("activate-linux") end
                    }),
                    monitor_index == 1 and ui.tray({
                        color = "#0A3A77",
                    }),
                },
            }),
        },
    })
end

-- tiny workspace switcher at the bottom
for _, monitor in ui.monitors() do
    ui.bar({
        monitor = monitor,
        position = "bottom",
        height = 1,
        -- TODO: black background

        scroll = function(delta)
            wm.set_workspace(delta > 0 and "next" or "prev")
        end,
    })
end
