local log = require("__janosch-lib__.log")
local events = require("__janosch-lib__.events")
local entities = require("__janosch-lib__.entities")
local remote = require("__janosch-remote__.remote")
local debug = require("__janosch-lib__.debug")

local config = require("config")


-- Register logging to file, rcon and player console
log.set_level(log.DEBUG)
log.register_logger(log.game_logger(), log.WARNING)
log.register_logger(log.file_logger(), log.DEBUG)
log.register_logger(log.rcon_logger(), log.INFO)
log.register_logger(log.player_logger(), log.INFO)


CHANNEL_SIGNAL = { type = "virtual", name = config.SIGNAL_CHANNEL_NAME }


-- Initialize remote communication
remote.init(config.MOD_NAME)

remote.register_request_handler("get_available_signals", function (request)
    local signals = {}

    for _, virtual in pairs(game.virtual_signal_prototypes) do
        if (not virtual.special) then
            table.insert(signals, { name = virtual.name, type = "virtual" })
        end
    end
    for _, fluid in pairs(game.fluid_prototypes) do
        if not fluid.hidden then
            table.insert(signals, { name = fluid.name, type = "fluid" })
        end
    end
    for _, item in pairs(game.item_prototypes) do
        if not item.has_flag("hidden") then
            table.insert(signals, { name = item.name, type = "item" })
        end
    end

    return signals
end)

remote.register_request_handler("send_signals", function (request)
    set_receiver_frame(request.channel, request.signals)
end)

remote.register_request_handler("receive_signals", function (request)
    local response = get_sender_frame(request.channel)
    log.debug("receive_signals(%s) = %s", serpent.line(request), serpent.line(response))
    return response
end)

remote.register_request_handler("import_blueprint", function (request)
    return import_blueprint(request.channel, request.blueprint_data)
end)


remote.track_connection(start_receiver_refresh, stop_receiver_refresh)


function get_and_update_channel(entity_data)
    if not entity_data then
        log.fail("entity_data is nil")
    end

    log.debug("get_and_update_channel: entity_name=%s", entity_data.entity_name)
    local channel = entity_data.entity.get_merged_signal({ name = config.SIGNAL_CHANNEL_NAME, type = "virtual" })
    entity_data.channel = channel
    log.debug("channel = %s", channel)
    return channel
end

function set_receiver_frame(channel, frame)
    local parameters = {}

    for i, signal in ipairs(frame) do
        parameters[i] = { index = i, count = signal.count, signal = {name = signal.name, type = signal.type } }
    end

    global.received_signals[channel] = parameters

    entities.for_each(config.SIGNAL_RECEIVER_NAME,
            function(signal_receiver)
                refresh_signal_receiver(signal_receiver, channel)
            end,
            entities.for_each_filters.channel(channel)
    )
end

function refresh_signal_receiver(signal_receiver, channel)
    log.debug("Refresh signal receiver: " .. signal_receiver.unit_number .. ", channel=" .. serpent.line(channel))

    channel = channel or get_and_update_channel(signal_receiver)
    signal_receiver.entity.get_or_create_control_behavior().parameters = { parameters = global.received_signals[channel] }
end

function refresh_all_signal_receivers()
    --??
    entities.for_each(config.SIGNAL_RECEIVER_NAME, function(signal_receiver)
        local current_channel = signal_receiver.channel

        if get_and_update_channel(signal_receiver) ~= current_channel then
            refresh_signal_receiver(signal_receiver)
        end
    end)
end

function reset_receivers()
    global.received_signals = {}

    entities.for_each(config.SIGNAL_RECEIVER_NAME, function(signal_receiver)
        signal_receiver.entity.get_or_create_control_behavior().parameters = { parameters = {} }
    end)
end

function get_sender_frame(channel)
    local added_signals = {}
    local is_channel = false

    entities.for_each(
            config.SIGNAL_SENDER_NAME,
            function(signal_sender)
                local sender_signals = signal_sender.entity.get_merged_signals()
                log.trace("get_sender_frame: sender_signal=%s", serpent.block(sender_signals))

                if not sender_signals then
                    log.warn("%s is not connected to a circuit network: position=%s", signal_sender.entity_name, serpent.line(signal_sender.entity.position))
                    return
                end

                for _, signal in pairs(sender_signals) do
                    if signal.signal.type == "virtual" and signal.signal.name == config.SIGNAL_CHANNEL_NAME then
                        log.trace("Sender unit_number=%d has channel=%d, channel_filter=%d", signal_sender.unit_number, signal.count, channel)
                        if signal.count == channel then
                            log.trace("is_channel=true")
                            is_channel = true
                        else
                            break
                        end
                    else
                        added_signals[signal.signal.type] = added_signals[signal.signal.type] or {}
                        added_signals[signal.signal.type][signal.signal.name] = (added_signals[signal.signal.type][signal.name] or 0) + signal.count
                    end
                end
            end
    )

    log.trace("Added signals: is_channel=%s, %s", debug.on_off(is_channel), serpent.block(added_signals))

    local frame = {}
    if is_channel then
        for signal_type, signals in pairs(added_signals) do
            log.debug("signal_type=%s", signal_type)
            for signal_name, signal_count in pairs(signals) do
                log.debug("signal_name=%s, signal_count=%s", signal_name, signal_count)
                table.insert(frame, { type = signal_type, name = signal_name, count = signal_count })
            end
        end

        log.debug("Sender frame: %s", serpent.block(frame))
    end

    return frame
end

function import_blueprint(channel, blueprint_data)
    local final_result = 0

    entities.for_each(
            config.BLUEPRINT_IMPORTER_NAME,
            function(blueprint_importer)
                log.debug("Importing blueprint to " .. blueprint_importer.unit_number .. ", " ..blueprint_importer.entity.name)
                local inventory = blueprint_importer.entity.get_inventory(defines.inventory.chest)
                --local stack = inventory.find_empty_stack()
                local stack = inventory[1]

                log.debug("Stack: %s", serpent.block(stack))

                if stack then
                    log.debug("Importing blueprint: " .. serpent.line(blueprint_data))
                    local result = stack.import_stack(blueprint_data)

                    if result == 1 then
                        final_result = 1
                        return true
                    elseif result == -1 then
                        final_result = -1
                    end
                end
            end,
            entities.for_each_filters.channel(channel)
    )

    return final_result
end

function refresh_status_combinator(status_combinator)
    log.debug("Refresh status combinator: " .. status_combinator.unit_number)

    status_combinator.entity.get_or_create_control_behavior().parameters = { parameters = { {
        index = 1,
        signal = { type = "virtual", name = config.SIGNAL_STATUS_NAME },
        count = global.connection_status_code ,
    } } }
end

function set_connection_status(status)
    if status ~= global.is_connected then
        local status_code = 1

        if not status then
            log.info("Remote disconnected")
            status_code = -1

            -- better call something like on_remote_disconnected
            reset_receivers()
        else
            log.info("Remote connected")
        end

        global.connection_status_code = status_code

        for _, status_combinator in pairs(global.status_combinators) do
            refresh_status_combinator(status_combinator)
        end

        global.is_connected = status
    end
end

--[[
    # Trains
]]


function apply_train_schedule(train, train_schedule)
    log.debug("apply_train_schedule: train.id=%d, train_schedule=%s", train.id, serpent.block(train_schedule))

    for _, record in pairs(train_schedule.records) do
        log.debug("record.rail =", serpent.line(record.rail))
        if record.rail then
            local entity = entities.find_entity({ position = record.rail, --[[name = "rail"]] })

            if not entity then
                log.fail("Could not find entity at", serpent.line(record.rail))
            end

            record.rail = entity
        end
    end

    log.debug("Setting train's (train_id=%s) schedule to %s", train.id, serpent.block(train_schedule))
    train.schedule = train_schedule
end

remote.register_request_handler("set_train_stop_color", function(request)
    entities.for_each(
            config.TRAIN_STOP_NAME,
            function(train_stop)
                train_stop.color = request.color
            end,
            entities.for_each_filters.channel(request.channel)
    )
end)

remote.register_request_handler("set_train_schedule", function(request)
    log.debug("set_train_schedule: channel=%s: %s", request.channel, serpent.block(request.train_schedule))
    entities.for_each(
        config.TRAIN_STOP_NAME,
        function(train_stop)
            local train = train_stop.entity.get_stopped_train()
            log.debug("Checking train_stop unit_number=%d", train_stop.unit_number)

            if not train then
                log.warn("No train parked at train stop (unit_number=%d)", train_stop.unit_number)
                return
            end

            log.debug("Checking train:", serpent.block(train))

            if train_stop.channel == request.channel then
                apply_train_schedule(train, request.train_schedule)
            end
        end,
        entities.for_each_filters.channel(request.channel)
    )
end)

remote.register_request_handler("get_train_schedule", function(request)
    log.debug("get_train_schedule: channel=%d", request.channel)

    local train_stops = {}

    entities.for_each(
            config.TRAIN_STOP_NAME,
            function(train_stop)
                local train = train_stop.entity.get_stopped_train()
                if train then
                    train_stops[train_stop.unit_number] = train.schedule
                end
            end,
            entities.for_each_filters.channel(request.channel)
    )

    return train_stops
end)


remote.register_request_handler("get_radar", function(request)
    log.debug("get_radar: channel=%d", request.channel)

    local radars = {}

    entities.for_each(
            config.RADAR_NAME,
            function(radar)
                log.debug("Radar: " .. radar.entity_name)
                local distance = radar.entity.prototype.max_distance_of_nearby_sector_revealed
                log.debug("get_radar: unit_number=%d, distance=%d", radar.unit_number, distance)

                local radar_data = {}
                for _, entity in pairs(radar.entity.surface.find_entities_filtered{ position = radar.entity.position, radius = distance }) do
                    table.insert(radar_data, {
                        unit_number = entity.unit_number,
                        name = entity.entity_name,
                        position = entity.position
                    })
                end

                table.insert(radars, {
                    entities = radar_data,
                    radar = radar.unit_number,
                    origin = radar.entity.position,
                })
            end,
            function (radar)
                return get_and_update_channel(radar.combinator) == request.channel
            end
    )

    return radars
end)

remote.register_request_handler("get_radar_screenshot", function(request)
    local screenshots = {}

    entities.for_each(
            config.RADAR_NAME,
            function(radar)
                log.debug("Radar: " .. radar.entity_name)
                local distance = radar.entity.prototype.max_distance_of_nearby_sector_revealed
                log.debug("get_radar: unit_number=%d, distance=%d", radar.unit_number, distance)

                local tick = game.tick;
                local position = radar.entity.position

                local file_name = string.format(request.file_name, tick, position.x, position.y)
                local path = "screenshots/janosch-sh/" .. file_name

                game.take_screenshot{
                    surface = request.surface,
                    position = { position.x + request.offset.x, position.y + request.offset.y },
                    resolution = request.resolution,
                    zoom = request.zoom,
                    path = path,
                    show_gui = false,
                }

                table.insert(screenshots, {
                    position = position,
                    file_name = file_name,
                    tick = tick,
                })
            end,
            function (radar)
                log.debug("request filter: channel=%d", request.channel)
                return get_and_update_channel(radar.combinator) == request.channel
            end
    )

    return screenshots
end)


events.on_init(function()
    log.debug("Initializing " .. config.MOD_NAME .. " global state.")

    reset_mod()
end)


function reset_mod()
    log.info("Resetting mod " .. config.MOD_NAME)

    global.connection_status_code = -1

    entities.init({ "on_channel_change" })

    entities.register(config.STATUS_COMBINATOR_NAME, function(entity_data)
        entity_data.entity.operable = false;
        refresh_status_combinator(entity_data)
    end)

    entities.register(config.SIGNAL_SENDER_NAME, function(entity_data)
        entity_data.entity.operable = false;
    end)

    entities.register(config.SIGNAL_RECEIVER_NAME, function(entity_data)
        entity_data.entity.operable = false;
        refresh_signal_receiver(entity_data)
    end)

    entities.register(config.BLUEPRINT_IMPORTER_NAME, function(blueprint_importer)

    end)

    entities.register(config.TRAIN_STOP_NAME, function(train_stop)

    end)

    entities.register(
            config.RADAR_NAME,
            function(radar)
                local entity = radar.entity.surface.create_entity {
                    name = config.RADAR_COMBINATOR_NAME,
                    position = { radar.entity.position.x, radar.entity.position.y + 1 },
                    force = radar.entity.force,
                }

                radar.combinator = entities.add_entity(entity, {
                    --parent = radar
                })
            end,
            function(radar)
                radar.combinator.entity.destroy()
            end
    )

    entities.register(config.RADAR_COMBINATOR_NAME, function(radar_combinator)
        radar_combinator.entity.operable = false
    end)

    entities.update(
            config.CHANNEL_SIGNAL_UPDATE_INTERVAL,
            "on_channel_change",
            config.SIGNAL_RECEIVER_NAME,
            entities.update_filters.on_signal_change(CHANNEL_SIGNAL),
            function (signal_receiver)
                if not signal_receiver then
                    log.fail("signal_receiver is nil")
                end
                log.debug("Updating signal receiver: unit_number=%s, channel=%s", serpent.line(signal_receiver.unit_number), serpent.line(signal_receiver.channel))
                signal_receiver.channel = signal_receiver.signals[CHANNEL_SIGNAL]
                signal_receiver.entity.get_or_create_control_behavior().parameters = { parameters = global.received_signals[signal_receiver.channel] }
            end
    )

    entities.enable_update_group("on_channel_change", true)

    reset_receivers()
end


commands.add_command("janosch-sh-rescan", "Rescan all entities for Simulation Hacking (debug!)", function()
    log.debug("Rescanning entities for " .. config.MOD_NAME)

    entities.rescan()
end)

commands.add_command("janosch-sh-reset", "Reset state of mod (connection status, received signals).", function (request)
    if not config.DEV_MODE then
        log.error("Trying to reset while not in dev mode.")
        return
    end

    reset_mod()
end)
