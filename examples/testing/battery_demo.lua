local batteries = xcake_battery_info()

print("Found " .. #batteries .. " battery(ies)")

for i, battery in ipairs(batteries) do
    print("Battery " .. i .. ":")
    print("  State of Charge: " .. string.format("%.1f%%", battery.charge))
    print("  State of Health: " .. string.format("%.1f%%", battery.health))
    print("  Energy: " .. string.format("%.2f Wh", battery.energy))
    print("  Energy Full: " .. string.format("%.2f Wh", battery.energy_full))
    print("  Voltage: " .. string.format("%.2f V", battery.voltage))

    if battery.temperature then
        print("  Temperature: " .. string.format("%.1f°C", battery.temperature))
    end

    if battery.cycle_count then
        print("  Cycle Count: " .. battery.cycle_count)
    end

    if battery.vendor then
        print("  Vendor: " .. battery.vendor)
    end

    if battery.model then
        print("  Model: " .. battery.model)
    end

    print("  Technology: " .. battery.technology)
    print("  State: " .. battery.state)
    print()
end
