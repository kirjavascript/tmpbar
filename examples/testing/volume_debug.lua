local sys = require('sys')

local volume = sys.volume.info()

if volume.error then
    print('Error: ' .. volume.error)
    return
end

print('Current Volume Info:')
print('  Volume: ' .. volume.volume .. ' (raw)')
print('  Volume Percent: ' .. string.format('%.1f%%', volume.percent))
print('  Is Muted: ' .. tostring(volume.is_muted))
print('  Range: ' .. volume.min .. ' - ' .. volume.max)
print()

os.exit()
