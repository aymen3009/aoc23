-- local file_handle = io.open( "./day2_lua/demo" )
local file_handle = io.open( "./day2_lua/input" )
local file_content = file_handle:read( "*line" )
local array = {}
local colors = {
    ['red'] = 12,
    ['green'] = 13,
    ['blue'] = 14
}

while file_content ~= nil do
    table.insert( array, file_content )
    file_content = file_handle:read( "*line" )
end


file_handle:close()

function split(s, delimiter)
    result = {};
    for match in (s..delimiter):gmatch("(.-)"..delimiter) do
        table.insert(result, match);
    end
    return result;
end

function all_trim(s)
    return s:match( "^%s*(.-)%s*$" )
 end


local function part_one ( array )
    local res = 0
    for i = 1, #array do
        local game_index = tonumber(split(split( array[i], ":" )[1], " ")[2])
        local game_records = split( split( array[i], ":" )[2], ";" )
        local is_valid = true
        for j in ipairs( game_records ) do
            local game_record = split( game_records[j], "," )
            for k in ipairs( game_record ) do
                local element = split(all_trim(game_record[k])," ")
                local number = tonumber(element[1])
                local color = element[2]
                if number > colors[color] then
                    is_valid = false
                    break
                end
            end
        end
        if is_valid then
            res = res + tonumber(game_index)
        end
    end
    return res
end

local function part_two ( array )
    local res = 0
    for i = 1, #array do
        local game_index = tonumber(split(split( array[i], ":" )[1], " ")[2])
        local game_records = split( split( array[i], ":" )[2], ";" )
        local total_colors_count = {
            ['red'] = 0,
            ['green'] = 0,
            ['blue'] = 0
        }
        for j in ipairs( game_records ) do
            local game_record = split( game_records[j], "," )
            for k in ipairs( game_record ) do
                local element = split(all_trim(game_record[k])," ")
                local number = tonumber(element[1])
                local color = element[2]
                total_colors_count[color] = math.max( total_colors_count[color], number )
            end
        end
        res = res + (total_colors_count['red'] * total_colors_count['green'] * total_colors_count['blue'])
    end
    return res
end
    
print( "Part one: " .. part_one( array ) )
print( "Part two: " .. part_two( array ) )

