for _, monitor in monitors() do
    bar({
        monitor = monitor,
        position = "top",
        height = 50,
        flex = true,
        -- orientation = "v",

        -- wrap = true,
        -- direction = "top-down",
        -- justify = true,
        -- crossJustify = true,
        -- align = "end",
        -- crossAlign = "end",
        -- margin = 5,

--         background = function(svg) return string.format([[
--           <rect x="0" y="0" width="%d" height="%d" rx="5" fill="none" stroke="#996699" stroke-width="5"/>
--         ]], svg.width, svg.height) end,

        items = {
            {
                "workspaces",
                render = function (workspace) return {
                    "label",
                    text = tostring(workspace.number),
                    background = function(svg)
                        local color = 3

                        if workspace.urgent then
                            color = "red"
                        elseif workspace.focused then
                            color = "blue"
                        elseif workspace.visible then
                            color = "darkblue"
                        else
                            color = "black"
                        end

                        return string.format([[
                            <rect x="0" y="0" width="16" height="%d" fill="%s"/>
                        ]], svg.height, color)
                    end,
                } end
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
                "image",
                size = 50,
                path = "./demo.gif",
            },
            {
                "image",
                size = 50,
                markup = function(svg) return string.format([[
                    <rect x="0" y="0" width="%d" height="%d" fill="none" stroke="#FFAA00" stroke-width="5"/>
                ]], svg.width, svg.height) end,
            },
            {
                "input",
                size = 150,
                submit = print,
            },
            {
                "label",
                align = "end",
                justify = true,
                text = function() return "« "..windowTitle().." »" end,
            },
            {
                "container",
                direction = "<",
                items = {
                    {
                        "label",
                        interval = 1000,
                        text = function() return os.date("%Y-%m-%d %X") end,
                    },
                    {
                        "label",
                        text = "foo bar baz",
                    },
                },
            },
        },
    })
end

-- TODO
-- i3mode
-- window collapse
-- animated SVG for battery monitor, CPU graph
