
local config = {}


config.DEBUG = true


-- Name of the mod. Used as prefix for other names
config.MOD_NAME = "janosch-remote"
config.MOD_PATH = "__" .. config.MOD_NAME .."__/"

config.INTERFACE_NAME = "janosch-remote"

config.REQUEST_PREFIX = "j-req"
config.RESPONSE_PREFIX = "j-resp"
config.HEARTBEAT_COMMAND = "j-hb"

config.REQUEST_PIPE = config.MOD_NAME .. "/request_pipe:"

config.CONNECTION_TIMEOUT = 60
config.DEFAULT_REQUEST_TIMEOUT = 60
config.TIMEOUT_CHECK_INTERVAL = 20

config.ALLOW_CODE_EXECUTION = false

return config
