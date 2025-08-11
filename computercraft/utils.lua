local lib = {}

lib.NEXT_DIRECTION = {
    north = "east",
    east = "south",
    south = "west",
    west = "north"
}

lib.PREV_DIRECTION = {
    north = "west",
    east = "north",
    south = "east",
    west = "south"
}

function lib.forward(pos, direction)
    if direction == "north" then
        return {pos[1], pos[2], pos[3] - 1}
    elseif direction == "east" then
        return {pos[1] + 1, pos[2], pos[3]}
    elseif direction == "south" then
        return {pos[1], pos[2], pos[3] + 1}
    elseif direction == "west" then
        return {pos[1] - 1, pos[2], pos[3]}
    end
end

function lib.backward(pos, direction)
    if direction == "north" then
        return {pos[1], pos[2], pos[3] + 1}
    elseif direction == "east" then
        return {pos[1] - 1, pos[2], pos[3]}
    elseif direction == "south" then
        return {pos[1], pos[2], pos[3] - 1}
    elseif direction == "west" then
        return {pos[1] + 1, pos[2], pos[3]}
    end
end

function lib.place_block(pos, block)
    commands.execAsync("setblock " .. pos[1] .. " " .. pos[2] .. " " .. pos[3] .. " " .. block)
end

return lib
