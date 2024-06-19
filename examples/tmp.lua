for _, monitor in monitors() do
    bar({
        monitor = monitor,
        position = "top",
        height = 18,
        layout = {
            {
                "button",
                text = "shutdown",
                click = function() return exec("~/.config/i3/scripts/powermenu") end
            },
            {
                "label",
                text = function() return os.date("%Y-%m-%d %X") end
            },
            -- {
            --     "text-input",
            --     submit = function(text) return exec(text) end
            -- },
            {
                "label", -- active-window
                text = function() return exec("xdotool getactivewindow getwindowname") end
            },
            { "label", text = function() return "disk " ..exec("~/.config/i3/scripts/disk") end },
            { "label", text = function() return "temp" ..exec("~/.config/i3/scripts/temperature") end },
            -- {
            --     "container",
            --     layout = {
            --         "label",
            --         text = "foo",
            --     },
            --     foo = {
            --         bar = "bar"
            --     },
            -- },
        }
    })
end

-- TODO
-- title, get i3mode from i3blocks code
-- WebView ?! File Menu (everything from cakey)
-- window collapse
