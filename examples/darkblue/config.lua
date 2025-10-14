local ui = require('ui')
local wm = require('wm')
local sys = require('sys')

local blue = "#0A83FD"
local darkblue = "#0022CC"

for index, monitor in ui.monitors() do
    ui.bar({
        monitor = monitor,
        position = "top",

        style = {
            height = 27,
            color = "#DDD",
            justify_content = "space_between",
            background = function(svg) return string.format([[
                <rect width="%d" height="%d" fill="black" />
                <rect x="0" y="%d" width="%d" height="2" fill="%s" />
            ]], svg.width, svg.height, svg.height - 2, svg.width, blue) end,
        },

        items = {
            ui.label({
                style = {
                    position = "absolute",
                    margin = "auto",
                    padding_bottom = 4,
                },
                text = function() return "« " .. truncate(wm.window_title(), 80) .. " »" end,
            }),
            ui.container({
                items = {
                    ui.image({ path = "./assets/manjaro.svg", style = { size = 26, padding_right = 3, } }),
                    ui.workspaces({
                        style = { gap = 3, height = 23 },
                        render = function (workspace) return ui.label({
                            style = {
                                width = 20,
                                background_color = function()
                                    return workspace.urgent and "red"
                                        or workspace.focused and blue
                                        or workspace.visible and darkblue
                                        or "black"
                                end,
                            },
                            text = tostring(workspace.name):sub(1, 1),
                            click = function()
                                wm.set_workspace(workspace.number)
                            end,
                        }) end
                    }),
                    ui.label({ -- i3 mode
                        style = {
                            background_color = function()
                                local is_default = wm.i3_mode() == "default"
                                return is_default and "transparent" or blue
                            end,
                        },
                        text = function()
                            local mode = wm.i3_mode()
                            return (mode == "default") and "" or (" « " .. mode .. " » ")
                        end,
                    }),
                },
            }),
            ui.container({
                style = {
                },
                items = {
                    ui.label({ -- memory
                        style = { align_self = "center", },
                        text = function() return "MEM " .. sys.memory().used_percent end,
                    }),
                    ui.image({
                        style = {
                            margin_left = 10,
                            align_self = "center",
                            size = 18,
                        },
                        path = "./assets/disk.svg",
                    }),
                    ui.label({ -- disk
                        style = { margin_left = 5, align_self = "center" },
                        text = function()
                            return sys.disk()['/'].free .. ' free'
                        end,
                    }),
                    -- battery
                    ui.image({
                        style = {
                            margin_left = 10,
                            align_self = "center",
                            size = 15,
                        },
                        path = "./assets/ip.svg",
                    }),
                    ui.label({ -- ip
                        style = { margin_left = 5,align_self = "center" },
                        text = function()
                            return sys.ip()
                        end,
                    }),

                    ui.image({
                        style = {
                            margin_left = 10,
                            align_self = "center",
                            size = 15,
                        },
                        path = "./assets/bandwidth.svg",
                    }),
                    ui.label({ -- network
                        style = { margin_left = 5,align_self = "center" },
                        text = function()
                            local bw = sys.bandwidth();
                            return bw.enp3s0 and bw.enp3s0.down
                                or bw.eth0 and bw.eth0.down
                                or '[no interface]'
                        end,
                    }),
                    -- cpu %
                    ui.image({
                        style = {
                            margin_left = 10,
                            align_self = "center",
                            size = 15,
                        },
                        path = "./assets/cpu.svg",
                    }),
                    ui.label({
                        style = { margin_left = 5, align_self = "center" },
                        text = function()
                            return sys.cpu_temp() .. '°C'
                        end,
                    }),
                    ui.image({
                        style = {
                            margin_left = 10,
                            align_self = "center",
                            size = 15,
                        },
                        path = "./assets/clock.svg",
                    }),
                    ui.label({ -- clock
                        style = { margin_left = 5, align_self = "center" },
                        text = function() return os.date("%Y-%m-%d %a %X") end,
                    }),
                    index == 1 and ui.tray({
                        style = {
                            background_color = "black",
                            height = 20,
                            margin_left = 10,
                            align_self = "center",
                        },
                    }),
                    ui.image({
                        style = { margin_left = 5, align_self = "center", size = 18, },
                        path = "./assets/tux.png",
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

        style = {
            height = 1,
            background_color = "black",
        },

        scroll = function(delta)
            wm.set_workspace(delta > 0 and "next" or "prev")
        end,
    })
end
