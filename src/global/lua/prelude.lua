-- "stdlib"

local function read_file(filePath)
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

local function trim(s)
    return (s:gsub("^%s*(.-)%s*$", "%1"))
end

---@diagnostic disable: lowercase-global
function truncate(s, length, ellipse)
    ellipse = ellipse or "..."
    if #s > length then
        return string.sub(s, 1, length) .. ellipse
    else
        return s
    end
end

local function throttle(fn, delay)
    local last_call = 0
    local value
    return function(...)
        local now = os.clock()
        if now - last_call >= delay or value == nil then
            last_call = now
            value = fn(...)
        end

        return value
    end
end

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

function ui.load_font(name, path)
    xcake_load_font(name, path)
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
    local handle, err = io.popen(cmd)
    if not handle then
        return nil, err
    end

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

local get_ip = throttle(function()
    return trim(sys.exec([[
        local_ip=$(
          if command -v ip >/dev/null 2>&1; then
            ip route get 8.8.8.8 2>/dev/null | awk '{for(i=1;i<=NF;i++) if ($i=="src") print $(i+1)}'
          elif command -v ifconfig >/dev/null 2>&1; then
            ifconfig | grep -Eo 'inet (addr:)?([0-9]+\.){3}[0-9]+' | grep -vE '127\.0\.0\.1|255\.255\.255\.255' | awk '{print $2; exit}'
          fi
        )

        echo "$local_ip"
    ]]))
end, 120.0)

function sys.ip()
    return get_ip()
end

package.loaded["sys"] = sys
builtin_modules["sys"] = true

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
