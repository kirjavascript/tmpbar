dump = function(o)
   if type(o) == 'table' then
      local s = '{ '
      for k,v in pairs(o) do
         if type(k) ~= 'number' then k = '"'..k..'"' end
         s = s .. '['..k..'] = ' .. dump(v) .. ','
      end
      return s .. '} '
   else
      return tostring(o)
   end
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

monitors = function()
    return ipairs(xcake_monitors)
end

bar = function(config)
    table.insert(xcake_bars, config)
end
