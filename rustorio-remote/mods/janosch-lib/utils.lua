
local utils = {}

function utils.sub_global(name, constructor)
    if not global[name] then
        if constructor then
            global[name] = constructor()
        else
            global[name] = {}
        end
    end

    return global[name]
end


return utils
