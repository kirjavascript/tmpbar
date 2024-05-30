function dump(o)
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

function bar()
end

print("foo")

print(dump(monitors))

for _, monitor in ipairs(monitors) do

    bar({
        monitor = monitor.name,
        position = "top",
        minHeight = 20,
        -- children
    })

end
