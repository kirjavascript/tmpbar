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
-- import other lua functions (!), also import a standard api
--
-- setmetatable(_G, {
    -- __index = function(_, key)
    --     return function(props)
    --         return component(key, props)
    --     end
    -- end
-- })
-- image {}
--function make_component(name)
    -- return function(props) return component(name, props) end
-- end

-- label = make_component("label")
-- image = make_component("image")
-- button = make_component("button")
--
-- main.lua
-- local M = {}

-- function M.hello()
--     print("Hello from my inline module!")
-- end

-- -- register it as if it were a required module
-- package.loaded["mymodule"] = M

-- -- now other scripts can require it:
-- local mod = require("mymodule")
-- mod.hello()

for monitor_index, monitor in monitors() do
    bar({
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
            component("label", { -- window title
                style = {
                    position = "absolute",
                    margin = "auto",
                },
                text = function() return "« " .. truncate(window_title(), 80) .. " »" end,
            }),
            component("label", { -- i3 mode
                style = {
                    position = "absolute",
                    margin_left = "auto",
                    margin_right = "auto",
                    size = "auto",
                    bottom = 3,
                },
                text = function() return i3_mode() == "default" and "" or "« "..i3_mode().." »" end,
                background = function(svg)
                    local is_default = i3_mode() == "default"
                    local color = is_default and "transparent" or "darkred"

                    return ([[
                        <rect x="0" y="0" width="%d" height="%d" fill="%s" rx="2"/>
                    ]]):format(svg.width, svg.height, color)
                end,
            }),
            component("image", {
                path = "https://sharey.org/8ilgDQ.png",
                style = { padding = 3, size = "max", },
            }),
            component("image", {
                path = "./archlinux.svg",
                style = { size = "max" },
            }),
            component("workspaces", {
                render = function (workspace) return component("label", {
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
                            set_workspace(workspace.number)
                        end,
                }) end
            }),
            component("container", {
                items = {
                    component("label", { -- clock
                        text = function() return os.date("%Y-%m-%d %a %X") end,
                    }),
                    component("label", { -- network
                        text = function()
                            local bw = bandwidth();
                            return bw.enp3s0 and bw.enp3s0.down
                                or bw.eth0 and bw.eth0.down
                                or '[no interface]'
                        end,
                    }),
                    component("label", {
                        text = function()
                            return 'MEM ' .. memory().used_percent
                        end,
                    }),
                    component("label", {
                        text = function()
                            return cpu_temp() .. '°C'
                        end,
                    }),
                    component("label", {
                        text = function()
                            return disk()['/'].free .. ' free'
                        end,
                    }),
                    component("button", {
                        text = "shutdown",
                        size = 100,
                        click = function() return spawn("~/.config/i3/scripts/powermenu") end
                    }),

                    component("button", {
                        text = "activate",
                        size = 100,
                        click = function() return spawn("activate-linux") end
                    }),
                    monitor_index == 1 and component("tray", {
                        color = "#0A3A77",
                    }),
                },
            }),
        },
    })
end

-- tiny workspace switcher at the bottom
for _, monitor in monitors() do
    bar({
        monitor = monitor,
        position = "bottom",
        height = 1,
        -- TODO: black background

        scroll = function(delta)
            set_workspace(delta > 0 and "next" or "prev")
        end,
    })
end
