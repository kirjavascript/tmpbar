-- "private" bindings

-- track built in packages
local keep_set = {}
for name in pairs(package.preload) do
    keep_set[name] = true
end

keep_set["_G"] = true
keep_set["package"] = true

function xcake_reset_state()
    -- reset vars
    xcake_bars = {}
    -- clear require() cache
    for name in pairs(package.loaded) do
        if not keep_set[name] then
            package.loaded[name] = nil
        end
    end
    -- ???
    collectgarbage()
end

-- core API

function monitors()
    return ipairs(xcake_monitors)
end

function bar(config)
    table.insert(xcake_bars, config)
end

function component(name, config)
    config = config or {}
    config['xcake_component'] = name
    return config
end

-- public bindings

xcake_window_title = ""
xcake_i3_mode = "default"

function window_title()
    return xcake_window_title
end

function i3_mode()
    return xcake_i3_mode
end

function set_workspace(value)
    if type(value) == "string" then
        xcake_cycle_workspace(value)
    elseif type(value) == "number" then
        xcake_focus_workspace(value)
    end
end

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

function exec(cmd)
    local handle = io.popen(cmd)
    local result = handle:read("*a")
    handle:close()
    return result
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

    for i, v in ipairs(args) do
        table.insert(result, serialize(v))
    end

    print(table.concat(result, "  "))
end
