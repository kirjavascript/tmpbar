function xcake_reset_state()
    xcake_bars = {}
end

xcake_window_title = ""

function windowTitle()
    return xcake_window_title
end

function readFile(filePath)
    if rawget(_G, "xcake_parent_path") ~= nil then
        if filePath:sub(1, 1) ~= '/' then
            filePath = xcake_parent_path .. filePath
        end
    end

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

function monitors()
    return ipairs(xcake_monitors)
end

function bar(config)
    table.insert(xcake_bars, config)
end
