local log = require("__janosch-lib__.log")
local events = require("__janosch-lib__.events")
local utils = require("__janosch-lib__.utils")
local debug = require("__janosch-lib__.debug")


local entities = {}

local entity_handlers = {}


function entities.init(enabled_update_groups)
    log.info("Initializing entity management")
    global.entities = {
        known_entities = {},
        by_name = {},
        update_filters = {},
        enabled_update_groups = enabled_update_groups,
        children = {}
    }
end


function entities.register(name, on_created, on_destroyed)
    --[[local children = {}

    for name, constructor in pairs(children_constructors) do
        children[name] = constructor()
        if children[name].name
    end]]

    --global.entities.children_constructors[name] = children_constructors

    entity_handlers[name] = {
        on_created = on_created,
        on_destroyed = on_destroyed,
    }
end


function by_name(name)
    if not global.entities.by_name[name] then
        global.entities.by_name[name] = {}
    end
    return global.entities.by_name[name]
end


function entities.for_each(name, f, filter)
    local by_name = by_name(name)

    if not by_name then
        log.fail("by_name empty")
        return
    end

    for unit_number, _ in pairs(by_name) do
        local entity_data = global.entities.known_entities[unit_number]
        if not entity_data then
            log.fail("Entity " .. unit_number .. " is known by name '" .. name .."', but not could not be found.")
        end

        --log.trace("for_each: name=%s, filter=%s", name, debug.format_info(filter))
        if filter == nil or filter(entity_data) then
            if f(entity_data) then
                break
            end
        end
    end
end

function entities.update(ticks, group, entity_name, filter, handler)
    log.debug(
            "Registering entity update: ticks=%d, group=%s, entity=%s, filter.filter=%s, filter.cleanup=%s, handler=%s",
            ticks,
            group,
            entity_name,
            debug.format_info(filter.filter),
            debug.format_info(filter.cleanup),
            debug.format_info(handler)
    )

    events.on_nth_tick(
            ticks,
            function (tick)
                if global.entities.enabled_update_groups[group] then
                    log.trace("Checking update group %s", group)

                    --log.trace(serpent.block(global.entities.known_entities))
                    log.trace("entities.update: entity_name=%s", entity_name)

                    entities.for_each(entity_name, function(entity_data)
                        --log.trace("Checking entity for update group: group=%s, unit_number=%d, filter=%s", group, unit_number, debug.format_info(filter.filter))

                        if not filter.filter or filter.filter(entity_data) then
                            log.trace("Update group %s: unit_number=%d", group, entity_data.unit_number)
                            handler(entity_data)
                        end
                    end)
                end
            end,
            function()
                if filter.cleanup then
                    filter.cleanup()
                end
            end
    )
end

function entities.enable_update_group(group, on_off)
    global.entities.enabled_update_groups[group] = on_off
end

function get_and_update_signals(entity_data, always_update)
    log.debug("get_and_update_signals: unit_number=%d, always_update=%s", entity_data.unit_number, serpent.line(always_update))

    if not entity_data.signals or always_update then
        entity_data.signals = entity_data.entity.get_merged_signals()
        log.debug("updated:", serpent.block(entity_data.signals))
    end


    return entity_data.signals
end

entities.update_filters = {}

function entities.update_filters.on_signal_change(signal)
    local on_signal_change = {}

    function on_signal_change.filter(entity_data)
        local current_signal = entity_data.entity.get_merged_signal(signal)
        local previous_signal = entity_data.signals[signal]

        log.trace("Checking signal %s/%s: unit_number=%d: previous=%s current=%s", signal.type, signal.name, entity_data.unit_number, serpent.line(previous_signal), serpent.line(current_signal))

        if current_signal ~= previous_signal then
            log.debug("Signal %s/%s changed: unit_number=%d: %s -> %d", signal.type, signal.name, entity_data.unit_number, serpent.line(previous_signal), serpent.line(current_signal))
            entity_data.signals[signal] = current_signal
            return true
        end
    end

    return on_signal_change
end


entities.for_each_filters = {}
function entities.for_each_filters.channel(channel)
    return function(entity_data)
        log.debug("for_each_filters.channel(%d): entity_data=%s", channel, serpent.block(entity_data))
        local entity_channel =  get_and_update_channel(entity_data)
        log.debug("for_each_filters.channel(%d): entity_channel=%d", channel, entity_channel)
        return entity_channel == channel
    end
end


function entities.rescan()
    log.info("Rescanning entities")

    local filters = { name = {} }

    for name, _ in pairs(entity_handlers) do
        global.entities.by_name[name] = {}
        table.insert(filters.name, name)
    end

    log.debug("Searching entities: filters = " .. serpent.block(filters))
    if not next(filters.name) then
        log.warn("No entities registered. Aborting.")
        return
    end

    for _, surface in pairs(game.surfaces) do
        for _, entity in pairs(surface.find_entities_filtered(filters)) do
            log.debug("found entity: " .. entity.name)
            entities.add_entity(entity)
        end
    end
end

function entities.find_entity(filter)
    log.debug("Finding entities:", serpent.block(position))
    for _, surface in pairs(game.surfaces) do
        for _, entity in pairs(surface.find_entities_filtered(filter)) do
            log.debug("Found entity:", serpent.block(entity))
            return entity
        end
    end
end

function entities.add_entity(entity, data)
    data = data or {}

    if not entity or not entity.valid then
        log.fail("Entity is invalid")
        return
    end

    if not entity.unit_number then
        log.error("Entity doesn't have a unit_number: " .. entity.name)
        return
    end

    local handlers = entity_handlers[entity.name]
    if not handlers then
        return
    end

    script.register_on_entity_destroyed(entity)

    log.debug("Entity added: " .. entity.name .. ", unit_number=" .. entity.unit_number)

    data.entity = entity
    data.entity_name = entity.name
    data.unit_number = entity.unit_number

    data.signals = {}
    if handlers.on_created then
        handlers.on_created(data)
    end

    global.entities.known_entities[entity.unit_number] = data

    if not global.entities.by_name[entity.name] then
        global.entities.by_name[entity.name] = {}
    end
    global.entities.by_name[entity.name][entity.unit_number] = true

    return data
end

function on_entity_created(event)
    log.trace("on_entity_created: event=" .. serpent.block(entity))
    entities.add_entity(event.created_entity)
end

function on_entity_destroyed(event)
    local entity_data = global.entities.known_entities[event.unit_number]
    if not entity_data then
        log.warn("Unknown entity destroyed: ", event.unit_number)
        return
    end

    local by_name = global.entities.by_name[entity_data.entity_name]
    if by_name then
        by_name[event.unit_number] = nil
    end

    local handlers = entity_handlers[entity_data.entity_name]
    if not handlers then
        log.warn("Handlers for entity '" .. entity_data.entity_name .. "', unit_number=" .. event.unit_number .. " missing")
        return
    end

    if handlers.on_destroyed then
        handlers.on_destroyed(entity_data)
    end

    global.entities.known_entities[event.unit_number] = nil

    log.debug("Entity removed: name=%s, unit_number=%s", entity_data.entity_name, event.unit_numbere)
end


events.on_event(defines.events.on_built_entity, on_entity_created)
events.on_event(defines.events.on_robot_built_entity, on_entity_created)
events.on_event(defines.events.on_entity_destroyed, on_entity_destroyed)


return entities
