function workspaces()
    return {
        "workspaces",
        render = function (workspace) return {
                "label",
                text = tostring(workspace.number),
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
        } end
    }
end

for _, monitor in monitors() do
    bar({
        monitor = monitor,
        position = "top",
        height = 40,
        flex = true,
        -- orientation = "v",

        -- wrap = true,
        -- direction = "top-down",
        -- justify = true,
        -- crossJustify = true,
        -- align = "end",
        -- crossAlign = "end",
        -- margin = { bottom = 5 },

        background = function(svg) return string.format([[
            <rect x="0" y="0" width="%d" height="%d" rx="5" fill="none" stroke="black" stroke-width="5"/>
        ]], svg.width, svg.height) end,

        items = {
            {
                "image",
                size = 40,
                path = "./archlinux.svg",
            },
            workspaces(),
            -- {
            --     "image",
            --     size = 50,
            --     markup = function(svg) return ([[
            --         <rect x="0" y="0" width="%d" height="%d" fill="none" stroke="#FFAA00" stroke-width="5"/>
            --     ]]):format(svg.width, svg.height) end,
            -- },
            -- {
            --     "input",
            --     size = 150,
            --     submit = print,
            -- },
            {
                "label",
                justify = true,
                text = function() return "« "..window_title().." »" end,
            },
            {
                "container",
                direction = "<",
                items = {
                    {
                        "label", -- clock
                        interval = 1000,
                        text = function() return os.date("%Y-%m-%d %X") end,
                    },
                    {
                        "button",
                        text = "shutdown",
                        justify = true,
                        crossJustify = true,
                        size = 100,
                        click = function() return exec("~/.config/i3/scripts/powermenu") end
                    },
                    {
                        "button",
                        text = "activate",
                        justify = true,
                        crossJustify = true,
                        size = 100,
                        click = function() return exec("activate-linux") end
                    },
                    {
                        "tray",
                        foo = "bar",
                    },
                },
            },
        },
    })
end


-- TODO
-- fix activate linux being sync
-- i3mode
-- window collapse
-- animated SVG for battery monitor, CPU graph
