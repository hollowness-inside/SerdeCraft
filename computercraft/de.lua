local utils = require("utils")

local start = {-1, 56 + 1, 0}
local previous = start

local direction = "north"
local radius = 3
local currentFold = 1
local current = 2

function proceed()
    start = utils.forward(start, direction)
    current = current + 1
    if current >= radius then
        if currentFold == 3 then
            radius = radius + 1
            currentFold = 1
        end
        current = 1
        direction = utils.NEXT_DIRECTION[direction]
        currentFold = currentFold + 1
    end
end

function rewind()
    current = current - 1
    if current <= 0 then
        currentFold = currentFold - 1
        if currentFold <= 0 then
            radius = radius - 1
            currentFold = 4
        end
        current = radius - 1
        direction = utils.PREV_DIRECTION[direction]
    end
    start = utils.backward(start, direction)
end

local address = "ws://localhost:8765"
local ws, connected = http.websocket(address)
if ws then
    ws.send("de")
else
    print("Failed to connect to websocket")
    return
end

while true do
    local event, p1, cmd, p3 = os.pullEvent()
    if event == "websocket_success" then
        print("Websocket connected!")
        utils.place_block(previous, "minecraft:glass")
        ws.send("de")

    elseif event == "websocket_failure" then
        printError("Websocket failed: " .. cmd)
        break

    elseif event == "websocket_message" then
        utils.place_block(previous, "minecraft:air")
        utils.place_block(start, "minecraft:glass")
        print(cmd)

        if cmd == "peek" then
            local block = commands.getBlockInfo(start[1], start[2] - 1, start[3])
            ws.send(block.name)
        elseif cmd == "consume" then
            local block = commands.getBlockInfo(start[1], start[2] - 1, start[3])
            ws.send(block.name)
            previous = start
            proceed()
        elseif cmd == "rewind" then
            rewind()
            ws.send("done")
        end

    elseif event == "websocket_closed" then
        print("Websocket closed.")
        break
    end
end

ws.close()
