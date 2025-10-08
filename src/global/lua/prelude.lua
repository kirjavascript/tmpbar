-- "private" bindings

-- track built in packages
local builtin_modules = {}
for name in pairs(package.preload) do
    builtin_modules[name] = true
end

builtin_modules["_G"] = true
builtin_modules["package"] = true

---@diagnostic disable: lowercase-global
function xcake_reset_state()
    -- reset vars
    xcake_bars = {}
    -- clear require() cache
    for name in pairs(package.loaded) do
        if not builtin_modules[name] then
            package.loaded[name] = nil
        end
    end
    -- ???
    collectgarbage()
end

-- public modules

local ui = {}

function ui.monitors()
    ---@diagnostic disable: undefined-global
    return ipairs(xcake_monitors)
end

function ui.bar(config)
    table.insert(xcake_bars, config)
end

function ui.component(name, config)
    config = config or {}
    config['xcake_component'] = name
    return config
end

-- wildcard access for ui.label etc
local mt = getmetatable(ui) or {}
mt.__index = function(_, key)
    return function(...)
        return ui.component(key, ...)
    end
end
setmetatable(ui, mt)

package.loaded["ui"] = ui
builtin_modules["ui"] = true

local wm = {}

xcake_window_title = ""
xcake_i3_mode = "default"

function wm.window_title()
    return xcake_window_title
end

function wm.i3_mode()
    return xcake_i3_mode
end

function wm.set_workspace(value)
    if type(value) == "string" then
        xcake_cycle_workspace(value)
    elseif type(value) == "number" then
        xcake_focus_workspace(value)
    end
end

package.loaded["wm"] = wm
builtin_modules["wm"] = true

local sys = {}

function sys.exec(cmd)
    local handle = io.popen(cmd)
    local result = handle:read("*a")
    handle:close()
    return result
end

function sys.spawn(...)
    return xcake_spawn(...)
end

function sys.memory(...)
    return xcake_memory(...)
end

function sys.disk(...)
    return xcake_disk(...)
end

function sys.cpu_temp(...)
    return xcake_cpu_temp(...)
end

function sys.bandwidth(...)
    return xcake_bandwidth(...)
end

package.loaded["sys"] = sys
builtin_modules["sys"] = true

-- "stdlib"

function read_file(filePath)
    local file, err = io.open(filePath, "r")
    if not file then
        return nil, err
    end

    local content = {}
    local byte = file:read(1)
    while byte do
        table.insert(content, string.byte(byte))
        byte = file:read(1)
    end

    file:close()
    return content
end

function trim(s)
    return (s:gsub("^%s*(.-)%s*$", "%1"))
end

function truncate(s, length, ellipse)
    ellipse = ellipse or "..."
    if #s > length then
        return string.sub(s, 1, length) .. ellipse
    else
        return s
    end
end

function log(...)
    local args = {...}
    local result = {}

    local function serialize(val, indent, visited)
        indent = indent or ""
        visited = visited or {}

        if visited[val] then
            return "[Circular Reference]"
        end

        local valType = type(val)

        if valType == "nil" then
            return "nil"
        elseif valType == "number" or valType == "boolean" then
            return tostring(val)
        elseif valType == "string" then
            return string.format("%q", val)
        elseif valType == "table" then
            if visited[val] then
                return "[Circular Reference]"
            end
            visited[val] = true

            local str = "{\n"
            local nextIndent = indent .. "  "

            -- First serialize numeric indices (array part)
            local arrayPart = {}
            for i, v in ipairs(val) do
                table.insert(arrayPart, nextIndent .. serialize(v, nextIndent, visited))
            end

            -- Then serialize hash part (non-numeric keys)
            local hashPart = {}
            for k, v in pairs(val) do
                if type(k) ~= "number" or k < 1 or k > #val or math.floor(k) ~= k then
                    local key = type(k) == "string" and k or "[" .. serialize(k, nextIndent, visited) .. "]"
                    table.insert(hashPart, nextIndent .. key .. " = " .. serialize(v, nextIndent, visited))
                end
            end

            -- Combine array and hash parts
            local parts = {}
            if #arrayPart > 0 then
                table.insert(parts, table.concat(arrayPart, ",\n"))
            end
            if #hashPart > 0 then
                table.insert(parts, table.concat(hashPart, ",\n"))
            end

            str = str .. table.concat(parts, ",\n") .. "\n" .. indent .. "}"
            return str
        elseif valType == "function" then
            return "[Function]"
        elseif valType == "userdata" then
            return "[Userdata]"
        elseif valType == "thread" then
            return "[Thread]"
        else
            return "[" .. valType .. "]"
        end
    end

    for _, v in ipairs(args) do
        table.insert(result, serialize(v))
    end

    print(table.concat(result, "  "))
end
