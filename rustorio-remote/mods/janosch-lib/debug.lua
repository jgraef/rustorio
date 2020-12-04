

local debug = {
    -- rename Lua's debug module
    lua_debug = debug,
}

debug.get_info = debug.lua_debug.getinfo
debug.traceback = debug.lua_debug.traceback


function debug.format_info(f)
    if not f then
        return "nil"
    end

    local info = debug.get_info(f, "Snl")
    if not info then
        return "nil"
    end

    local line = info.currentline
    if line == -1 then
        line = info.linedefined
    end

    return string.format("%s:%d", info.short_src, line)
end


function debug.on_off(x)
    if x then
        return "on"
    else
        return "off"
    end
end



function debug.format(format, ...)
    -- TODO: Use regex?

    local varargs = table.pack(...)

    local output = {}
    local pos = 1

    local function next_char()
        local c = format:sub(pos, pos)
        pos = pos + 1
        return c
    end

    while true do
        local c = next_char()
        if not c then
            break
        end

        if c == "{" then
            local c = next_char()
            if c == ":" then
                -- TODO: Debug print
            end
        else
            table.insert(output, c)
        end
    end

    table.concat(output)
end


return debug
