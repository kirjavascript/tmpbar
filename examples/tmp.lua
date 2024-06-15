for _, monitor in monitors() do
    bar({
        monitor = monitor,
        position = "top",
        height = 28,
        layout = {
            {
                "label",
                text = "foo",
                text2 = function() return os.date("%Y-%m-%d %X") end
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
-- system() function for menus / buttons
