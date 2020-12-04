local log = require("__janosch-lib__.log")
local events = require("__janosch-lib__.events")
local debug = require("__janosch-lib__.debug")


local config = require("__janosch-remote__.config")
local error = require("__janosch-remote__.error")


local remote = {}

local next_request_id = 1
local is_connected = false


-- Handlers for requests sent from client
local request_handlers = {
    -- Built-in request types

    list_request_types = { on_request = function (request)
        request_types = {}

        for request_type, _ in pairs(request_handlers) do
            table.insert(request_types, request_type)
        end

        return request_types
    end },

    ping = { on_request = function (request)
        return request.cookie
    end },

    print = { on_request = function (request)
        if request.color then
            game.print(request.message, request.color)
        else
            game.print(request.message)
        end
    end },

    run_code = { on_request = function (request) -- TODO: Test/Debug
        if not config.DEBUG then
            error("Only in DEBUG modus")
        end

        return loadstring(request.code)()
    end },

    global = { on_request = function (request)
        if not config.DEBUG then
            error("Only in DEBUG modus")
        end

        return global
    end}
}

-- Current requests sent to the client, i.e. the function to call upon completion
local open_requests = {}

-- Event handlers registered by other mods
local event_handlers = {}


-- Packet encoder
function packet_encode(packet)
    return game.table_to_json(packet)
end

-- Packet decoder
function packet_decode(table)
    return game.json_to_table(table)
end


-- When a packet was received
function on_packet_received()
    last_heartbeat = {
        tick = game.tick,
    }
end


-- Dispatches the request handler and returns the response
function dispatch_request(request, request_handler)
    if not request_handler then
        return {
            status = "error",
            error = error.no_handler_for_request_type(request.request_type),
        }
    end

    log.debug("dispatch_request: Calling request handler for %s: %s", request.request_type, debug.format_info(request_handler.on_request))

    local status, value = pcall(request_handler.on_request, request)

    log.trace("dispatch_request: pcall: status=%s, value=%s", serpent.line(status), serpent.block(value))

    if status then
        return {
            status = "success",
            value = value,
        }
    else
        log.warn("Request handler error: " .. value)

        return {
            status = "error",
            error = error.request_handler_failed(request.request_type, value, request_handler.on_request),
        }
    end
end


function register_command(type, name, f)
    local full_name = string.format("%s-%s-%s", config.MOD_NAME, name, type)

    log.debug("Registering command:", full_name)

    commands.add_command(full_name, "Internal command for " .. config.MOD_NAME, f)
end

-- Initializes the remote interfaces
function remote.init(name)
    log.info("Initializing " .. config.MOD_NAME)

    next_request_id = 1
    last_heartbeat = nil

    -- When the client sends a heartbeat
    register_command("hb", name, function (event)
        if game.player then
            log.warn("Player", game.player.name, "tried to execute internal command.")
        else
            on_packet_received()
        end
    end)

    -- When a request is received via RCON
    local response_prefix = string.format("%s-%s-resp:", config.MOD_NAME, name)
    register_command("req", name, function(event)
        if game.player then
            log.warn("Player", game.player.name, "tried to execute internal command.")
        else
            local request = packet_decode(event.parameter)

            log.debug("Received request: %s", serpent.block(request))

            local request_handler = request_handlers[request.request_type]

            if not request.request_type then
                log.error("Type missing in request")
                return
            end

            on_packet_received()

            response = dispatch_request(request, request_handler)

            log.trace("Responding: %s", serpent.block(response))

            rcon.print(response_prefix .. packet_encode(response))
        end
    end)


    -- When a response is received via RCON
    register_command("resp", name, function (event)
        if game.player then
            log.warn("Player", game.player.name, "tried to execute internal command.")
        else
            local response = packet_decode(event.parameter)

            if not response.request_id then
                log.error("ID missing in response")
                return
            end

            on_packet_received()

            local request = open_requests[response.request_id]

            if not request then
                log.error("Unknown request ID: " .. response.request_id)
                return
            end

            -- Remove request
            current_requests[response.request_id] = nil

            request.on_response(response)
        end
    end)
end


-- Checks for timeouts
events.on_nth_tick(
        config.TIMEOUT_CHECK_INTERVAL,
        function()
            -- Check heartbeat timeout
            if is_connected and last_heartbeat and game.tick - last_heartbeat.tick >= config.CONNECTION_TIMEOUT then
                log.debug("Connection lost")

                is_connected = false

                event_handlers.on_connection_lost()
            end

            -- Check request timeout
            local drop_requests = {}
            for request_id, request in pairs(open_requests) do
                if game.tick >= request.timeout then
                    log.warn("Request #" .. request_id .. " timeouted")

                    -- Make sure this request is removed from the open requests
                    table.insert(drop_requests, equest_id)

                    -- Call timeout handler
                    request.on_timeout()
                end
            end

            -- Remove all requests that timeouted
            for _, request_id in pairs(drop_requests) do
                open_requests[request_id] = nil
            end
        end
)


function remote.register_request_handler(request_type, handler)
    if not handler and request_handlers[request_type] then
        log.fail("Request type '" .. request_type .. "' is already registered")
    else
        log.debug("Registering handler for request: " , request_type)
        request_handlers[request_type] = { on_request = handler }
    end
end

function remote.send_request(request, on_response, timeout, on_timeout)
    timeout = timeout or config.DEFAULT_REQUEST_TIMEOUT

    log.debug("Sending request:", serpent.line(request))

    local request_data = packet_encode(request)

    request.request_id = next_request_id
    next_request_id = next_request_id + 1

    open_requests[request.request_id] = {
        timeout = game.tick + timeout,
        on_response = on_response,
        on_timeout = on_timeout,
    }

    game.write_file(config.MOD_TO_CLIENT_REQUEST_FILE)
end

function remote.is_connected()
    return is_connected
end

function remote.on_connection_established(handler)
    log.debug("Registering handler for on_connection_established")
    table.insert(event_handlers.on_connection_established, handler)
end

function remote.on_connection_lost(handler)
    log.debug("Registering handler for on_connection_lost")
    table.insert(event_handlers.on_connection_lost, handler)
end

function remote.track_connection(on_connection_established, on_connection_lost)
    function established_handler()
        remote.on_connection_established(nil)
        remote.on_connection_lost(lost_handler)

        on_connection_established()
    end

    function end_connection_tracking()
        remote.on_connection_established(established_handler)
        remote.on_connection_lost(nil)

        on_connection_lost()
    end

    if is_connected then
        remote.on_connection_lost(function ()
            remote.on_connection_established(established_handler)
        end)

        remote.on_connection_lost(function ()
            remote.on_connection_lost(lost_handler)
        end)
    else

    end
end


return remote
