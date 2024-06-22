for _, monitor in monitors() do
    bar({
            monitor = monitor,
            position = "top",
            height = 50,
            items = {
                {
                    "container",
                    -- flex = true,
                    -- wrap = true,
                    -- justify = true,
                    -- dir = "left-right",
                    align = "end",
                    -- crossDir = "left-right",
                    crossAlign = "end",
                    -- crossJustify = true,
                    direction = "h",
                    items = {
                        {
                            "label",
                            align = "start",
                            text = function() return os.date("%Y-%m-%d %X") end
                        },
                        {
                            "label",
                            text = "bar",
                        },
                        {
                            "label",
                            text = "[12345]",
                            align = "end"
                        },
                    },
                },
                -- {
                --         "label",
                --         text = "foo",
                -- },
                -- {
                --         "label",
                --         text = "bar",
                -- },
                -- {
                    --     "button",
                    --     text = "shutdown",
                    --     click = function() return exec("~/.config/i3/scripts/powermenu") end
                -- },
                -- {
                    --     "label",
                    --     text = function() return os.date("%Y-%m-%d %X") end
                -- },
                -- -- {
                    -- --     "text-input",
                    -- --     submit = function(text) return exec(text) end
                -- -- },
                -- {
                    --     "label", -- active-window
                    --     text = function() return exec("xdotool getactivewindow getwindowname") end
                -- },
                -- { "label", text = function() return "disk " ..exec("~/.config/i3/scripts/disk") end },
                -- { "label", text = function() return "temp" ..exec("~/.config/i3/scripts/temperature") end },
            },
        })
end

-- TODO
-- title, get i3mode from i3blocks code
-- WebView ?! File Menu (everything from cakey)
-- window collapse
