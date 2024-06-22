for _, monitor in monitors() do
    bar({
            monitor = monitor,
            position = "top",
            height = 50,
            flex = true,
            items = {
                {
                    "button",
                    text = "shutdown",
                    size = 100,
                    click = function() return exec("~/.config/i3/scripts/powermenu") end
                },
                {
                    "label", -- active-window
                    align = "end",
                    size = 1/1.9,
                    text = function() return trim(exec("xdotool getactivewindow getwindowname")) end
                },
                {
                    "container",
                    flex = true,
                    -- wrap = true,
                    -- justify = false,
                    -- dir = "right-left",
                    -- align = "end",
                    -- crossDir = "right-left",
                    -- crossAlign = "end",
                    -- crossJustify = true,
                    -- direction = "h",
                    -- debugLayout = true,
                    items = {
                        {
                            "label",
                            text = "foo bar baz",
                        },
                        -- { "label", text = function() return "disk " ..trim(exec("~/.config/i3/scripts/disk")) end },
                        -- { "label", text = function() return "temp" ..trim(exec("~/.config/i3/scripts/temperature")) end },
                        {
                            "label",
                            align = "end",
                            -- size = 100,
                            text = function() return os.date("%Y-%m-%d %X") end
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
