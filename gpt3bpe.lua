local ffi = require("ffi")

-- Determine library suffix based on OS
local function suffix()
    if jit.os == "OSX" then
        return "dylib"
    elseif jit.os == "Windows" then
        return "dll"
    else -- Linux, BSD, POSIX
        return "so"
    end
end

-- Determine the Rust target triple based on the current OS and architecture.
local function get_arch_triple()
    local arch_map = { x64 = "x86_64", arm64 = "aarch64" }
    local os_map = {
        OSX = "apple-darwin",
        Linux = "unknown-linux-gnu",
        -- Add other OS mappings here if needed
    }

    local arch = arch_map[jit.arch]
    local os = os_map[jit.os]

    if not arch or not os then
        error(string.format("Unsupported platform: OS=%s, Arch=%s. Please update get_arch_triple().", jit.os, jit.arch))
    end

    return string.format("%s-%s", arch, os)
end

-- Construct the path to the dynamic library.
local arch = get_arch_triple()
local lib_path = string.format("./target/%s/release/libgpt3bpe.%s", arch, suffix())

-- Define C types and function signatures for the FFI
ffi.cdef[[
    // Callback function types that Rust will call into
    typedef void (*rust_callback_u8)(size_t idx, uint8_t value);
    typedef void (*rust_callback_u32)(size_t idx, uint32_t value);

    // Grapheme function
    void grapheme(const uint8_t* buffer, size_t len, rust_callback_u8 cb);

    // Encoding functions (string -> tokens)
    void encode_p50k(const uint8_t* buffer, size_t len, rust_callback_u32 cb);
    void encode_r50k(const uint8_t* buffer, size_t len, rust_callback_u32 cb);
    void encode_cl100k(const uint8_t* buffer, size_t len, rust_callback_u32 cb);
    void encode_o200k(const uint8_t* buffer, size_t len, rust_callback_u32 cb);

    // Decoding functions (tokens -> string)
    void decode_p50k(const uint16_t* buffer, size_t len, rust_callback_u8 cb);
    void decode_r50k(const uint16_t* buffer, size_t len, rust_callback_u8 cb);
    void decode_cl100k(const uint32_t* buffer, size_t len, rust_callback_u8 cb);
    void decode_o200k(const uint32_t* buffer, size_t len, rust_callback_u8 cb);
]]

-- Load the shared library
local lib = ffi.load(lib_path)

local M = {}

--- Splits a string into its grapheme clusters (bytes).
-- @param input_str The string to process.
-- @return A table of bytes representing grapheme clusters.
function M.grapheme(input_str)
    local pointer = {}

    local callback = ffi.cast("rust_callback_u8", function(idx, value)
        table.insert(pointer, { idx = tonumber(idx), value = tonumber(value) })
    end)
    
    local buffer = ffi.cast("const uint8_t*", input_str)
    local len = #input_str
    
    lib.grapheme(buffer, len, callback)

    -- Sort the results by index
    table.sort(pointer, function(a, b) return a.idx < b.idx end)

    -- Extract values
    local result = {}
    for i, v in ipairs(pointer) do
        result[i] = v.value
    end

    return result
end

-- Vocabularies: 'r50k', 'p50k', 'cl100k', 'o200k'

--- Encodes a string into a sequence of tokens.
-- @param input_str The string to encode.
-- @param vocabulary The vocabulary to use ('r50k', 'p50k', 'cl100k', 'o200k'). Defaults to 'p50k'.
-- @return A table of token numbers.
function M.encode(input_str, vocabulary)
    local pointer = {}
    
    vocabulary = vocabulary or 'p50k'

    local callback = ffi.cast("rust_callback_u32", function(idx, value)
        table.insert(pointer, { idx = tonumber(idx), value = tonumber(value) })
    end)
    
    local buffer = ffi.cast("const uint8_t*", input_str)
    local len = #input_str
    
    local encode_func = lib["encode_" .. vocabulary] or lib.encode_p50k
    encode_func(buffer, len, callback)

    -- Sort the results by index
    table.sort(pointer, function(a, b) return a.idx < b.idx end)

    -- Extract values
    local result = {}
    for i, v in ipairs(pointer) do
        result[i] = v.value
    end

    return result
end

--- Decodes a sequence of tokens into a string.
-- @param tokens A table of token numbers.
-- @param vocabulary The vocabulary to use ('r50k', 'p50k', 'cl100k', 'o200k'). Defaults to 'p50k'.
-- @return The decoded string.
function M.decode(tokens, vocabulary)
    local pointer = {}

    vocabulary = vocabulary or 'p50k'

    local callback = ffi.cast("rust_callback_u8", function(idx, value)
        table.insert(pointer, { idx = tonumber(idx), value = tonumber(value) })
    end)

    -- Create a C array from the Lua table of tokens
    local len = #tokens
    local buffer
    if vocabulary == 'p50k' or vocabulary == 'r50k' then
        buffer = ffi.new("uint16_t[?]", len)
    else
        buffer = ffi.new("uint32_t[?]", len)
    end

    for i = 1, len do
        buffer[i-1] = tokens[i]
    end
    
    local decode_func = lib["decode_" .. vocabulary] or lib.decode_p50k
    decode_func(buffer, len, callback)

    -- Sort the results by index
    table.sort(pointer, function(a, b) return a.idx < b.idx end)

    -- Build the result string from byte values
    local result_len = #pointer
    if result_len == 0 then return "" end
    
    local result_buf = ffi.new("uint8_t[?]", result_len)
    for i, v in ipairs(pointer) do
        result_buf[i-1] = v.value
    end
    
    return ffi.string(result_buf, result_len)
end

--[[ Test Runner ]]

local function run_tests()
    -- NOTE: This test runner requires a JSON library. It will try to load 'cjson'
    -- (from LuaJIT) or 'dkjson' (a pure Lua implementation).
    -- Please ensure one is available in your Lua path.
    -- e.g., `luarocks install dkjson`
    local json_decode
    local ok, cjson = pcall(require, "cjson")
    if ok then
        json_decode = cjson.decode
    else
        local ok_dk, dkjson = pcall(require, "dkjson")
        if ok_dk then
            json_decode = dkjson.decode
        else
            print("[WARNING]: No JSON library found (tried 'cjson', 'dkjson'). Skipping tests.")
            return
        end
    end

    local function array_equal(t1, t2)
        if #t1 ~= #t2 then return false end
        for i = 1, #t1 do
            if t1[i] ~= t2[i] then return false end
        end
        return true
    end

    print("[INFO]: Running tests...")
    local test_file_path = "./src/bpe/vocabulary/tests.jsonl"
    local file = io.open(test_file_path, "r")
    if not file then
        print("[WARNING]: Could not open test file: " .. test_file_path)
        return
    end

    local test_count = 0
    for line in file:lines() do
        if line == "" then
            goto continue
        end
        local status, data = pcall(json_decode, line)
        -- Silently ignore parse errors or empty encodings, like in the Deno script.
        if not status or not data.encoded or #data.encoded == 0 then
            goto continue
        end

        local encoded_tokens = M.encode(data.text, data.model)
        assert(array_equal(encoded_tokens, data.encoded), "[ERROR]: Encode mismatch for: " .. data.text)

        local decoded_text = M.decode(encoded_tokens, data.model)
        assert(decoded_text == data.text, "[ERROR]: Decode mismatch for: " .. data.text)
        
        test_count = test_count + 1
        ::continue::
    end
    file:close()
    print("[INFO]: All " .. test_count .. " tests passed!")
end

run_tests()

return M
