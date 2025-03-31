-- TODO
-- click on everything
-- animated SVG for battery monitor, CPU graph

for monitor_index, monitor in monitors() do
    bar({
        monitor = monitor,
        position = "top",
        height = 30,
        flex = true,
        -- orientation = "v",

        -- wrap = true,
        -- direction = "top-down",
        -- justify = true,
        -- crossJustify = true,
        -- align = "end",
        -- crossAlign = "end",
        -- margin = { bottom = 5 },

        -- background = function(svg) return string.format([[
        --     <rect
        --         x="0"
        --         y="0"
        --         width="%d"
        --         height="%d"
        --         rx="5"
        --         fill="none"
        --         stroke="black"
        --         stroke-width="5"
        --     />
        -- ]], svg.width, svg.height) end,

        items = {
            component("image", {
                size = 40,
                path = "./archlinux.svg",
            }),
            component("workspaces", {
                render = function (workspace) return component("label", {
                        text = tostring(workspace.name):sub(1, 1),
                        size = 100, -- TODO
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
            component("i3mode", {
                render = function (mode) return component("label", {
                    text = mode == "default" and "" or mode,
                    background = function(svg)
                        local color = mode == "default" and "transparent" or "red"

                        return ([[
                            <rect x="0" y="0" width="%d" height="%d" fill="%s" rx="2"/>
                        ]]):format(svg.width, svg.height, color)
                    end,
                }) end
            }),
            component("label", {
                justify = true,
                text = function() return "« " .. truncate(window_title(), 80) .. " »" end,
            }),
            component("container", {
                direction = "<",
                items = {
                    component("label", { -- clock
                        interval = 1000,
                        text = function() return os.date("%a %Y-%m-%d %X") end,
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
                    component("button", {
                        text = "shutdown",
                        justify = true,
                        crossJustify = true,
                        size = 100,
                        click = function() return spawn("~/.config/i3/scripts/powermenu") end
                    }),
                    component("button", {
                        text = "activate",
                        justify = true,
                        crossJustify = true,
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
