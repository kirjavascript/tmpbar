for _, monitor in monitors() do
    bar({
        monitor = monitor,
        position = "top",
        height = 28,
        layout = {
            {
                "label",
                text = function() return os.date("%Y-%m-%d %X") end
            },
            {
                "label",
                text = function() return exec("xdotool getactivewindow getwindowname") end
            },
            {
                "container",
                layout = {
                    "label",
                    text = "foo",
                },
                foo = {
                    bar = "bar"
                },
            },
        }
    })
end

-- TODO
-- title, get i3mode from i3blocks code
-- WebView ?! File Menu (everything from cakey)
-- os.execute() function for menus / buttons
