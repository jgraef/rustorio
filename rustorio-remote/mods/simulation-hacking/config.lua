

local config = {}


config.DEV_MODE = true
config.DEBUG_RECIPE_ENABLED = true


-- Name of the mod. Used as prefix for other names
config.MOD_NAME = "janosch-sh"

config.MOD_PATH = "__" .. config.MOD_NAME .."__/"

-- Technology name
config.TECHNOLOGY_NAME = config.MOD_NAME .. "-tech"

-- Names of recipes, items and entities
config.SIGNAL_SENDER_NAME = config.MOD_NAME .. "-signal-sender"
config.SIGNAL_RECEIVER_NAME = config.MOD_NAME .. "-signal-receiver"
config.BLUEPRINT_IMPORTER_NAME = config.MOD_NAME .. "-blueprint-importer"
config.STATUS_COMBINATOR_NAME = config.MOD_NAME .. "-status-combinator"
config.TRAIN_STOP_NAME = config.MOD_NAME .. "-train-stop"
config.RADAR_NAME = config.MOD_NAME .. "-radar"
config.RADAR_COMBINATOR_NAME = config.MOD_NAME .. "-radar-combinator"

-- Names of virtual signals
config.SIGNAL_CHANNEL_NAME = config.MOD_NAME .. "-channel"
config.SIGNAL_STATUS_NAME = config.MOD_NAME .. "-status"

-- TODO: Instead of a fixed value, initialize as something and set it to the number of signals in data-final-fixes.lua
config.MAX_SIGNALS = 512

config.CHANNEL_SIGNAL_UPDATE_INTERVAL = 10

return config
