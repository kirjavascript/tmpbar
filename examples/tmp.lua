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
                    "image",
                    size = 50,
                    -- justify = true,
                    -- crossJustify = true,
                    path = "./demo.gif",
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
                            text = function() return os.date("%Y-%m-%d %X") end
                        },
                        {
                            "label",
                            text = "foo bar baz",
                        },
                        {
                            "battery",
                            render = function(filled) return {
                                "svg",
                                markup = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"30\" height=\"30\"><path d=\"M4.2 10a11.1 11.1 0 0 0 0 10h20c.4-.6.6-1.3.8-2h1.6a12 12 0 0 0 0-6H25c-.2-.7-.4-1.4-.8-2h-20zM19 11h5v8h-5v-8z\" style=\"fill:#".. ({"red", "yellow", "green"})[(filled/44) + 1] ..";fill-opacity:1;stroke:none;stroke-width:.49999997;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1\"/></svg>"
                            } end
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
