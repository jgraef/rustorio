local config = require("config")


data:extend{
    -- Technology
    {
        type = "technology",
        name = config.TECHNOLOGY_NAME,
        effects = {
            { type = "unlock-recipe", recipe = config.STATUS_COMBINATOR_NAME },
            { type = "unlock-recipe", recipe = config.SIGNAL_SENDER_NAME },
            { type = "unlock-recipe", recipe = config.SIGNAL_RECEIVER_NAME },
            { type = "unlock-recipe", recipe = config.BLUEPRINT_IMPORTER_NAME },
            { type = "unlock-recipe", recipe = config.TRAIN_STOP_NAME },
            { type = "unlock-recipe", recipe = config.RADAR_NAME },
        },
        icon = "__base__/graphics/technology/circuit-network.png",
        icon_size = 128,
        order = "e-g",
        prerequisites = {
            "advanced-electronics-2",
            "circuit-network"
        },
        unit = {
            count = 500,
            time = 30,
            ingredients = {
                { "automation-science-pack", 1 },
                { "logistic-science-pack", 1 },
                { "chemical-science-pack", 1 },
                { "utility-science-pack", 1 },
            }
        },
    },

    -- Recipes
    {
        type = "recipe",
        name = config.STATUS_COMBINATOR_NAME,
        ingredients = {
            {"constant-combinator", 1},
            {"processing-unit", 200},
        },
        result = config.STATUS_COMBINATOR_NAME,
        result_count = 1,
        energy_required = 1,
        icon = "__base__/graphics/icons/constant-combinator.png",
        icon_size = 64,
        enabled = config.DEBUG_RECIPE_ENABLED,
    },
    {
        type = "recipe",
        name = config.SIGNAL_SENDER_NAME,
        ingredients = {
            {"constant-combinator", 1},
            {"processing-unit", 200},
        },
        result = config.SIGNAL_SENDER_NAME,
        result_count = 1,
        energy_required = 1,
        icon = "__base__/graphics/icons/constant-combinator.png",
        icon_size = 64,
        enabled = config.DEBUG_RECIPE_ENABLED,
    },
    {
        type = "recipe",
        name = config.SIGNAL_RECEIVER_NAME,
        ingredients = {
            {"constant-combinator", 1},
            {"processing-unit", 200},
        },
        result = config.SIGNAL_RECEIVER_NAME,
        result_count = 1,
        energy_required = 1,
        icon = "__base__/graphics/icons/constant-combinator.png",
        icon_size = 64,
        enabled = config.DEBUG_RECIPE_ENABLED,
    },
    {
        type = "recipe",
        name = config.BLUEPRINT_IMPORTER_NAME,
        ingredients = {
            {"steel-chest", 1},
            {"processing-unit", 200},
        },
        result = config.BLUEPRINT_IMPORTER_NAME,
        result_count = 1,
        energy_required = 1,
        icon = "__base__/graphics/icons/steel-chest.png",
        icon_size = 64,
        enabled = config.DEBUG_RECIPE_ENABLED,
    },
    {
        type = "recipe",
        name = config.TRAIN_STOP_NAME,
        ingredients = {
            {"train-stop", 1},
            {"processing-unit", 200},
        },
        result = config.TRAIN_STOP_NAME,
        result_count = 1,
        energy_required = 1,
        icon = "__base__/graphics/icons/train-stop.png",
        icon_size = 64,
        enabled = config.DEBUG_RECIPE_ENABLED,
    },
    {
        type = "recipe",
        name = config.RADAR_NAME,
        ingredients = {
            {"radar", 1},
            {"processing-unit", 200},
        },
        result = config.RADAR_NAME,
        result_count = 1,
        energy_required = 1,
        icon = "__base__/graphics/icons/radar.png",
        icon_size = 64,
        enabled = config.DEBUG_RECIPE_ENABLED,
    },

    -- Items
    {
        type = "item",
        name = config.STATUS_COMBINATOR_NAME,
        icon = "__base__/graphics/icons/constant-combinator.png",
        icon_size = 64,
        subgroup = "logistic-network",
        order = "c[signal]-b[" .. config.STATUS_COMBINATOR_NAME .. "]",
        place_result = config.STATUS_COMBINATOR_NAME,
        stack_size = 10,
    },
    {
        type = "item",
        name = config.SIGNAL_SENDER_NAME,
        icon = "__base__/graphics/icons/constant-combinator.png",
        icon_size = 64,
        subgroup = "logistic-network",
        order = "c[signal]-b[" .. config.SIGNAL_SENDER_NAME .. "]",
        place_result = config.SIGNAL_SENDER_NAME,
        stack_size = 10,
    },
    {
        type = "item",
        name = config.SIGNAL_RECEIVER_NAME,
        icon = "__base__/graphics/icons/constant-combinator.png",
        icon_size = 64,
        subgroup = "logistic-network",
        order = "c[signal]-b[" .. config.SIGNAL_RECEIVER_NAME .. "]",
        place_result = config.SIGNAL_RECEIVER_NAME,
        stack_size = 10,
    },
    {
        type = "item",
        name = config.BLUEPRINT_IMPORTER_NAME,
        icon = "__base__/graphics/icons/steel-chest.png",
        icon_size = 64,
        subgroup = "logistic-network",
        order = "c[signal]-b[" .. config.BLUEPRINT_IMPORTER_NAME .. "]",
        place_result = config.BLUEPRINT_IMPORTER_NAME,
        stack_size = 10,
    },
    {
        type = "item",
        name = config.TRAIN_STOP_NAME,
        icon = "__base__/graphics/icons/train-stop.png",
        icon_size = 64,
        subgroup = "logistic-network",
        order = "c[signal]-b[" .. config.TRAIN_STOP_NAME .. "]",
        place_result = config.TRAIN_STOP_NAME,
        stack_size = 10,
    },
    {
        type = "item",
        name = config.RADAR_NAME,
        icon = "__base__/graphics/icons/radar.png",
        icon_size = 64,
        subgroup = "logistic-network",
        order = "c[signal]-b[" .. config.RADAR_NAME .. "]",
        place_result = config.RADAR_NAME,
        stack_size = 10,
    },

    -- Virtual signals
    {
        type = "virtual-signal",
        name = config.SIGNAL_CHANNEL_NAME,
        icon = config.MOD_PATH .. "graphics/signal_channel.png",
        icon_size = 64,
        subgroup = "virtual-signal",
        order = "z[" .. config.MOD_NAME .. "]-[3S]"
    },
    {
        type = "virtual-signal",
        name = config.SIGNAL_STATUS_NAME,
        icon = config.MOD_PATH .. "graphics/signal_status.png",
        icon_size = 64,
        subgroup = "virtual-signal",
        order = "z[" .. config.MOD_NAME .. "]-[3S]"
    },
}


function add_signal_transceiver(name)
    local prototype = table.deepcopy(data.raw["constant-combinator"]["constant-combinator"])
    prototype.name = name
    prototype.minable.result = name
    prototype.item_slot_count = config.MAX_SIGNALS
    data:extend{prototype}
end

add_signal_transceiver(config.STATUS_COMBINATOR_NAME)
-- TODO: Change sender to use lamp
add_signal_transceiver(config.SIGNAL_SENDER_NAME)
add_signal_transceiver(config.SIGNAL_RECEIVER_NAME)


local blueprint_importer = table.deepcopy(data.raw["container"]["steel-chest"])
blueprint_importer.name = config.BLUEPRINT_IMPORTER_NAME
blueprint_importer.minable.result = config.BLUEPRINT_IMPORTER_NAME
blueprint_importer.inventory_size = 1

local train_stop = table.deepcopy(data.raw["train-stop"]["train-stop"])
train_stop.name = config.TRAIN_STOP_NAME
train_stop.minable.result = config.TRAIN_STOP_NAME

local radar = table.deepcopy(data.raw["radar"]["radar"])
radar.name = config.RADAR_NAME
radar.minable.result = config.RADAR_NAME

local radar_combinator = table.deepcopy(data.raw["constant-combinator"]["constant-combinator"])
radar_combinator.name = config.RADAR_COMBINATOR_NAME
radar_combinator.result = config.RADAR_COMBINATOR_NAME

data:extend{blueprint_importer, train_stop, radar, radar_combinator}
