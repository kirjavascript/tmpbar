for _, monitor in monitors() do
    bar({
        monitor,
        position = "top",
        height = 25,
        layout = {
            {
                "label",
                text = "foo",
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
