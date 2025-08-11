local utils = require("utils")

local start = {-1, 56, 0}
local direction = "north"
local radius = 3
local currentFold = 1
local current = 2
local p = start

function proceed()
    p = utils.forward(p, direction)
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

local address = "ws://localhost:8765"
local ws, connected = http.websocket(address)
if ws then
    ws.send("ser")
else
    print("Failed to connect to websocket")
    return
end

while true do
    local event, p1, cmd, p3 = os.pullEvent()

    if event == "websocket_failure" then
        printError("Websocket failed: " .. cmd)
        break

    elseif event == "websocket_message" then
        print(cmd)
        utils.place_block(p, cmd)
        if ws then
            ws.send("1")
        end
        proceed()

    elseif event == "websocket_closed" then
        print("Websocket closed.")
        break
    end
end

ws.close()
