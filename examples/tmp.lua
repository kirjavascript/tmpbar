-- TODO
-- click on everything
-- animated SVG for battery monitor, CPU graph
-- change inverval to max_interval
-- window()

for monitor_index, monitor in monitors() do
    bar({
        monitor = monitor,
        position = "top",
        height = 100,

        style = {
            -- display = "flex",
            -- justify_items = "flex_end",
            -- justify_content = "flex_end",
            -- align_items = "flex_end",
            -- flex_direction = "column",
            -- postion = "absolute",
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
            component("image", {
                path = "./archlinux.svg",
            }),
            component("workspaces", {
                style = {
                    display = "flex",
                    flex_direction = "column",
                    padding = 20,
                    gap = 10,
                },
                render = function (workspace) return component("label", {
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
            component("label", { -- i3 mode
                text = function() return i3_mode() == "default" and "" or i3_mode() end,
                background = function(svg)
                    local color = i3_mode() == "default" and "transparent" or "red"

                    return ([[
                        <rect x="0" y="0" width="%d" height="%d" fill="%s" rx="2"/>
                    ]]):format(svg.width, svg.height, color)
                end,
            }),
            component("container", {
                style = {
                    position = "absolute",
                    display = "flex",
                    align_items = "center",
                    justify_content = "center",
                    justify_items = "center",
                },
                items = {
                    component("label", {
                        debug = true,
                        style = {
                            -- display = "block",
                            -- align_self = "center",
                        },
                        text = function() return "« " .. truncate(window_title(), 80) .. " »" end,
                    }),
                },
            }),
            component("label", {
                text = function() return "« " .. truncate(window_title(), 80) .. " »" end,
            }),
            component("container", {
                items = {
                    component("label", { -- clock
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
