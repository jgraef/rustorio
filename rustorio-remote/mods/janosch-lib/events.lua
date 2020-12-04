local log = require("__janosch-lib__.log")
local debug = require('__janosch-lib__.debug')


local events = {}

local handlers = {
    on_init = {},
    on_load = {},
    on_configuration_changed = {},
    on_event = {},
    on_nth_tick = {}
}

local next_event_id = 1


function insert_event(table, event, on_all_events_removed)
    local event_id = next_event_id

    table[event_id] = event

    local handle = {
        unregister = function()
            table[event_id] = nil

            if not next(table) and on_all_events_removed then
                -- Table is now empty
                on_all_events_removed()
            end
        end
    }

    next_event_id = next_event_id + 1

    return handle
end


function events.on_init(f, command)
    return insert_event(handlers.on_init, f)
end

script.on_init(function()
    for _, f in pairs(handlers.on_init) do
        f()
    end
end)


function events.on_load(f)
    return insert_event(handlers.on_load, f)
end

script.on_load(function()
    for _, f in pairs(handlers.on_load) do
        f()
    end
end)


function events.on_configuration_changed(f)
    return insert_event(handlers.on_configuration_changed, f)
end

script.on_configuration_changed(function(data)
    for _, f in pairs(handlers.on_configuration_changed) do
        f(data)
    end
end)


function events.on_event(event, f)
    if not handlers.on_event[event] then
        handlers.on_event[event] = {}

        script.on_event(event, function(triggered_event)
            for _, f in pairs(handlers.on_event[event]) do
                f(triggered_event)
            end
        end)
    end

    return insert_event(handlers.on_event[event], f, function()
        script.on_event(event, nil)
    end)
end


function events.on_nth_tick(tick, f, finally)
    log.debug("Initializing event on_nth_tick(tick=%d,f=%s,finally=%s)", tick, debug.format_info(f), debug.format_info(finally))

    if not handlers.on_nth_tick[tick] then
        handlers.on_nth_tick[tick] = {}

        script.on_nth_tick(tick, function (event)
            local handler_finally = {}

            for _, handler in pairs(handlers.on_nth_tick[tick]) do
                if handler.f then
                    --log.debug("on_nth_tick for", debug.format_info(debug.getinfo(f)))
                    handler.f(tick)
                end
                if handler.finally then
                    table.insert(handler_finally, handler.finally)
                end
            end

            for _, finally in pairs(handler_finally) do
                finally(tick)
            end

            if finally then
                finally(tick)
            end
        end)
    end

    return insert_event(
            handlers.on_nth_tick[tick], {
                f = f,
                finally = finally
            },
            function()
                script.on_nth_tick(tick, nil)
            end
    )
end


return events
