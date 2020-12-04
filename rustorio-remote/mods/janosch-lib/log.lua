local debug = require("__janosch-lib__.debug")


local log = {
    ERROR = 1,
    WARNING = 2,
    INFO = 3,
    DEBUG = 4,
    TRACE = 5,

    -- rename factorio's log function
    factorio_log = log,
    player_print = function(message)
        if game then
            game.print(message)
        end
    end
}

level_names = {
    [log.ERROR] = "ERROR",
    [log.WARNING] = "WARN",
    [log.INFO] = "INFO",
    [log.DEBUG] = "DEBUG",
    [log.TRACE] = "TRACE",
}

level_colors = {
    [log.ERROR] = { 1.0, 0.0, 0.0 },
    [log.WARNING] = {},
    [log.INFO] = {},
    [log.DEBUG] = {},
    [log.TRACE] = { 1.0, 1.0, 1.0 },
}

function log.default_formatter()
    return function(level, n, message)
        local tick
        if game then
            tick = game.tick
        else
            tick = 0
        end

        return string.format("[%s] %s %010d %s - %s", script.mod_name, debug.format_info(n), tick, level_names[level], message)
    end
end


local log_loggers = {}
local log_level = log.WARNING
local min_log_level = log_level
local any_non_default_formatter = false
local log_formatter = log.default_formatter()



function log.set_level(level)
    log_level = level
end

function log.register_logger(logger, level, formatter)
    if level and level < log_level then
        min_log_level = level
    end

    if not formatter then
        any_non_default_formatter = true
    end

    table.insert(log_loggers, {
        logger = logger,
        level = level,
        formatter = formatter,
    })
end

function format_message(format, ...)
    --local log_message = table.concat(table.pack(...), " ")
    return string.format(format, ...)
end

function log.log(level, n, ...)
    n = (n or 1) + 4

    local log_message = format_message(...)

    local full_message
    if any_non_default_formatter then
        full_message = log_formatter(level, n, log_message)
    end

    for _, logger in pairs(log_loggers) do
        --[[if game and level == log.DEBUG then
            log.factorio_log("log: " .. serpent.line(logger.level) .. ", " .. serpent.line(log_level) .. ", " .. level)
            log.factorio_log(log_message)
        end]]

        -- logger.log_level=nil,
        -- log_level=4(debug)
        -- level=5(trace)
        --

        --[[local a = not logger.log_level -- true
        local b = level <= (logger.level or log_level) -- false
        if game then
            game.print(string.format("a=%s, b=%s", serpent.line(a), serpent.line(b)))
        end]]

        if not logger.log_level and level <= (logger.level or log_level) then
            local message
            if logger.formatter then
                message = logger.formatter(level, log_message)
            else
                message = full_message
            end

            logger.logger(message)
        end
    end
end

function log.fail(...)
    log.log(log.ERROR, 1, ...)
    local message = format_message(...)
    error(message)
end
function log.error(...) log.log(log.ERROR, 1, ...) end
function log.warn(...) log.log(log.WARNING, 1, ...) end
log.warning = log.warn
function log.info(...) log.log(log.INFO, 1, ...) end
function log.debug(...) log.log(log.DEBUG, 1, ...) end
function log.trace(...) log.log(log.TRACE, 1, ...) end

function log.global(level)
    log.log(level or log.DEBUG, 1, "global =", serpent.block(global))
end

function log.traceback(level)
    log.log(level or log.DEBUG, 1, debug.traceback())
end


function log.game_logger()
    return function(message)
        log.factorio_log(message)
    end
end

function log.player_logger(player)
    if player then
        return function(message)
            player.print(message)
        end
    else
        return function(message)
            if game then
                game.print(message)
            end
        end
    end
end

function log.rcon_logger()
    return function(message)
        rcon.print(message)
    end
end

function log.file_logger(path, player_index)
    path = path or "logs/" .. script.mod_name .. "/current.log"
    player_index = player_index or 0
    return function(message)
        if game then
            game.write_file(path, message .. "\n", true, player_index)
        end
    end
end

function log.plain_formatter(message)
    return message
end


return log
