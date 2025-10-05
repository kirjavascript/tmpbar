for monitor_index, monitor in monitors() do
    bar({
        monitor = monitor,
        position = "top",
        height = 100,

        style = {
            display = "flex",
            justify_content = "space_between",
            align_items = "space_between",
        },

        items = {
            component("workspaces", {
                style = {
                    flex_direction = "row",
                    align_items = "stretch",
                    justify_content = "stretch",
                    justify_items = "stretch",
                    align_content = "stretch",
                    justify_self = "stretch",
                },
                render = function (workspace) return component("label", {
                    style = {align_self = "stretch" },
                    text = tostring(workspace.name):sub(1, 1),
                    background = function(svg)
                        local color = workspace.urgent and "red"
                            or workspace.focused and "#0A83FD"
                            or workspace.visible and "#0022CC"
                            or "black"

                        return ([[
                            <rect x="0" y="0" width="%d" height="%d" fill="%s" rx="2"/>
                        ]]):format(svg.width, svg.height, color)
                    end,
                    click = function()
                        set_workspace(workspace.number)
                    end,
                }) end
            }),
            component("label", {
                    debug = true,
                    style={
                        text_align = "center",
                    },
                    text = "?????",
            }),
            component("container", {
                style = {
                    gap = 10,
                    -- flex_direction = "column"
                    justify_content = "space_between",
                    align_items = "baseline",
                },
                items = {
                    component("label", {
                        text = "1",
                    }),
                    component("label", {
                        text = "2",
                    }),
                    component("label", {
                        text = "3",
                    }),
                    component("label", {
                        text = "4",
                    }),
                    component("label", {
                        text = "5",
                    }),
                },
            }),
        },
    })
end
