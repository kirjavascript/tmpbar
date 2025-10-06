for monitor_index, monitor in monitors() do
    bar({
        monitor = monitor,
        position = "top",
        height = 200,

        style = {
            display = "flex",
            flex_direction = "column",
            justify_content = "center",
            align_items = "center",
            padding = "20px",
        },

        items = {
            -- First row with 6 columns
            component("container", {
                style = {
                    display = "flex",
                    flex_direction = "row",
                    gap = "15px",
                    margin_bottom = "10px",
                    justify_content = "space_between",
                    align_items = "center",
                },
                items = {
                    component("label", {
                        text = "Col 1",
                        style = {
                            width = "80px",
                            height = "40px",
                            padding = "5px",
                        },
                        background = function(svg)
                            return string.format([[
                                <rect x="0" y="0" width="%d" height="%d" fill="#FF6B6B" rx="5"/>
                            ]], svg.width, svg.height)
                        end,
                    }),
                    component("label", {
                        text = "Col 2",
                        style = {
                            width = "80px",
                            height = "40px",
                            padding = "5px",
                        },
                        background = function(svg)
                            return string.format([[
                                <rect x="0" y="0" width="%d" height="%d" fill="#4ECDC4" rx="5"/>
                            ]], svg.width, svg.height)
                        end,
                    }),
                    component("label", {
                        text = "Col 3",
                        style = {
                            width = "80px",
                            height = "40px",
                            padding = "5px",
                        },
                        background = function(svg)
                            return string.format([[
                                <rect x="0" y="0" width="%d" height="%d" fill="#45B7D1" rx="5"/>
                            ]], svg.width, svg.height)
                        end,
                    }),
                    component("label", {
                        text = "Col 4",
                        style = {
                            width = "80px",
                            height = "40px",
                            padding = "5px",
                        },
                        background = function(svg)
                            return string.format([[
                                <rect x="0" y="0" width="%d" height="%d" fill="#96CEB4" rx="5"/>
                            ]], svg.width, svg.height)
                        end,
                    }),
                    component("label", {
                        text = "Col 5",
                        style = {
                            width = "80px",
                            height = "40px",
                            padding = "5px",
                        },
                        background = function(svg)
                            return string.format([[
                                <rect x="0" y="0" width="%d" height="%d" fill="#FFEAA7" rx="5"/>
                            ]], svg.width, svg.height)
                        end,
                    }),
                    component("label", {
                        text = "Col 6",
                        style = {
                            width = "80px",
                            height = "40px",
                            padding = "5px",
                        },
                        background = function(svg)
                            return string.format([[
                                <rect x="0" y="0" width="%d" height="%d" fill="#DDA0DD" rx="5"/>
                            ]], svg.width, svg.height)
                        end,
                    }),
                },
            }),

            -- Second row with 6 columns
            component("container", {
                style = {
                    display = "flex",
                    flex_direction = "row",
                    gap = "15px",
                    justify_content = "space_evenly",
                    align_items = "center",
                },
                items = {
                    component("button", {
                        text = "Btn 1",
                        style = {
                            width = "80px",
                            height = "40px",
                        },
                        click = function() print("Button 1 clicked") end,
                    }),
                    component("button", {
                        text = "Btn 2",
                        style = {
                            width = "80px",
                            height = "40px",
                        },
                        click = function() print("Button 2 clicked") end,
                    }),
                    component("button", {
                        text = "Btn 3",
                        style = {
                            width = "80px",
                            height = "40px",
                        },
                        click = function() print("Button 3 clicked") end,
                    }),
                    component("button", {
                        text = "Btn 4",
                        style = {
                            width = "80px",
                            height = "40px",
                        },
                        click = function() print("Button 4 clicked") end,
                    }),
                    component("button", {
                        text = "Btn 5",
                        style = {
                            width = "80px",
                            height = "40px",
                        },
                        click = function() print("Button 5 clicked") end,
                    }),
                    component("button", {
                        text = "Btn 6",
                        style = {
                            width = "80px",
                            height = "40px",
                        },
                        click = function() print("Button 6 clicked") end,
                    }),
                },
            }),
        },
    })
end
