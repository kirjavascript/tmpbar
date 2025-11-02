local ui = require('ui')
local wm = require('wm')
local sys = require('sys')

for _, monitor in ui.monitors() do
    ui.bar({
        monitor = monitor,
        position = 'top',

        style = {
            height = 25,
            color = '#FFFFFF',
            font_family = 'monospace',
            font_size = 12,
            justify_content = 'space_between',
            background_color = '#2E3440',
            padding_left = 10,
            padding_right = 10,
        },

        items = {
            ui.container({
                items = {
                    ui.workspaces({
                        style = {
                            gap = 5,
                            height = 25,
                        },
                        render = function (workspace) return ui.label({
                            style = {
                                width = 30,
                                text_align = 'center',
                                text_valign = 'center',
                                background_color = function()
                                    return workspace.urgent and '#BF616A'
                                        or workspace.focused and '#5E81AC'
                                        or workspace.visible and '#4C566A'
                                        or 'transparent'
                                end,
                                border_radius = 3,
                            },
                            text = tostring(workspace.name),
                            click = function()
                                wm.set_workspace(workspace.number)
                            end,
                        }) end
                    }),
                },
            }),

            ui.label({
                style = {
                    position = 'absolute',
                    margin = 'auto',
                },
                text = function()
                    local title = wm.window_title()
                    return title and title ~= '' and title or 'Desktop'
                end,
            }),

            ui.container({
                items = {
                    ui.label({
                        style = {
                            margin_right = 15,
                            padding = 5,
                            background_color = '#3B4252',
                            border_radius = 3,
                            cursor = 'pointer',
                        },
                        text = function()
                            local volume = sys.volume.info()
                            if volume.error then
                                return '🔇 Error'
                            end

                            print(volume.percent)

                            local icon = volume.is_muted and '🔇'
                                or volume.percent > 66 and '🔊'
                                or volume.percent > 33 and '🔉'
                                or volume.percent > 0 and '🔈'
                                or '🔇'

                            return string.format('%s %d%%', icon, math.floor(volume.percent))
                        end,
                        click = function()
                            local volume = sys.volume.info()
                            if not volume.error then
                                sys.volume.mute(not volume.is_muted)
                            end
                        end,
                        scroll = function(delta)
                            local volume = sys.volume.info()
                            if not volume.error then
                                local new_percent = volume.percent + (delta > 0 and 5 or -5)
                                new_percent = math.max(0, math.min(100, new_percent))
                                sys.volume.set(new_percent)
                            end
                        end,
                    }),

                    ui.label({
                        style = {
                            padding = 5,
                        },
                        text = function()
                            return os.date('%H:%M:%S')
                        end,
                    }),
                },
            }),
        },
    })
end
