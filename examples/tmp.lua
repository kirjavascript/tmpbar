for _, monitor in monitors() do
    bar({
            monitor = monitor,
            position = "top",
            height = 50,

            flex = true,
            -- orientation = "h",

            -- wrap = true,
            -- direction = "top-down",
            -- justify = true,
            -- crossJustify = true,
            -- align = "end",
            -- crossAlign = "end",
            -- debugLayout

            items = {
                {
                    "button",
                    text = "shutdown",
                    justify = true,
                    crossJustify = true,
                    size = 100,
                    click = function() return exec("~/.config/i3/scripts/powermenu") end
                },
                {
                    "label", -- active-window
                    align = "end",
                    justify = true,
                    size = 1/1.9,
                    text = function() return trim(exec("xdotool getactivewindow getwindowname")) end
                },
                {
                    "container",
                    direction = "<",
                    items = {
                        {
                            "label",
                            align = "end",
                            text = function() return os.date("%Y-%m-%d %X") end
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
-- title, get i3mode from i3blocks code
-- WebView ?! File Menu (everything from cakey)
-- window collapse
-- animated SVG for battery monitor, CPU graph
