local debug = require("__janosch-lib__.debug")


local error = {}

-- error types

error.no_handler_for_request_type = function (request_type)
    return {
        code = 1,
        message = "No handler found for request_type: " .. request_type,
    }
end

error.request_handler_failed = function (request_type, lua_error, handler)
    local origin
    if handler then
        origin = debug.format_info(handler)
    else
        origin = "unknown"
    end

    return {
        code = 2,
        lua_error = lua_error,
        message = "Handler failed with unexpectedly: " .. origin,
    }
end


return error
