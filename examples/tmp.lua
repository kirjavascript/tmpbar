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
        margin = 5,

        background = function(width, height) return string.format([[
          <rect x="0" y="0" width="%d" height="%d" rx="5" fill="none" stroke="#996699" stroke-width="5"/>
        ]], width, height) end,

        items = {
            {
                "workspaces",
                showAll = false,
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
                markup = function(width, height) return string.format([[
                    <rect x="0" y="0" width="%d" height="%d" fill="none" stroke="#FFAA00" stroke-width="5"/>
                ]], width, height) end,
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
-- get i3mode from i3blocks code
-- WebView ?! File Menu (everything from cakey)
-- window collapse
-- animated SVG for battery monitor, CPU graph
