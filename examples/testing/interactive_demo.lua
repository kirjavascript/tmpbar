local ui = require('ui')

local counter = 0
local input_text = ''
local status_message = 'Ready'

for _, monitor in ui.monitors() do
    ui.bar({
        monitor = monitor,
        position = 'top',

        style = {
            height = 240,
            background_color = '#2d3748',
            padding = 20,
            gap = 20,
            flex_direction = 'column',
        },

        items = {
            ui.container({
                style = {
                    gap = 15,
                    align_items = 'center',
                    background_color = '#4a5568',
                    padding = 15,
                    border_radius = 8,
                },
                items = {
                    ui.label({
                        text = function() return 'Counter: ' .. counter end,
                        style = {
                            color = '#e2e8f0',
                            font_size = 18,
                            font_weight = 'bold',
                        }
                    }),
                    ui.button({
                        text = 'Increment',
                        style = {
                            background_color = '#48bb78',
                            color = 'white',
                            padding = '8px 16px',
                            border_radius = 4,
                        },
                        click = function()
                            counter = counter + 1
                            status_message = 'Counter incremented to ' .. counter
                        end
                    }),

                    ui.button({
                        text = 'Decrement',
                        style = {
                            background_color = '#f56565',
                            color = 'white',
                            padding = '8px 16px',
                            border_radius = 4,
                        },
                        click = function()
                            counter = counter - 1
                            status_message = 'Counter decremented to ' .. counter
                        end
                    }),

                    ui.button({
                        text = 'Reset',
                        style = {
                            background_color = '#ed8936',
                            color = 'white',
                            padding = '8px 16px',
                            border_radius = 4,
                        },
                        click = function()
                            counter = 0
                            status_message = 'Counter reset'
                        end
                    }),
                }
            }),

            ui.container({
                style = {
                    gap = 15,
                    align_items = 'center',
                    background_color = '#4a5568',
                    padding = 15,
                    border_radius = 8,
                },
                items = {
                    ui.label({
                        text = 'Text Input:',
                        style = {
                            color = '#e2e8f0',
                            font_size = 16,
                        }
                    }),

                    ui.input({
                        text = input_text,
                        style = {
                            background_color = '#2d3748',
                            color = '#e2e8f0',
                            border = '1px solid #718096',
                            padding = '6px 12px',
                            border_radius = 4,
                            width = 200,
                        },
                        change = function(text)
                            input_text = text
                            status_message = "Input changed: '" .. text .. "'"
                        end,
                        submit = function(text)
                            status_message = "Submitted: '" .. text .. "'"
                        end
                    }),

                    ui.button({
                        text = 'Clear',
                        style = {
                            background_color = '#9f7aea',
                            color = 'white',
                            padding = '6px 12px',
                            border_radius = 4,
                        },
                        click = function()
                            input_text = ''
                            status_message = 'Input cleared'
                        end
                    }),

                    ui.label({
                        text = function() return 'Length: ' .. string.len(input_text) end,
                        style = {
                            color = '#a0aec0',
                            font_size = 14,
                        }
                    }),
                }
            }),

            ui.container({
                style = {
                    background_color = '#1a202c',
                    padding = 10,
                    border_radius = 4,
                    border_left = '4px solid #4299e1',
                },
                items = {
                    ui.label({
                        text = function() return 'Status: ' .. status_message end,
                        style = {
                            color = '#4299e1',
                            font_size = 14,
                            font_style = 'italic',
                        }
                    }),
                }
            }),
        },
    })
end
