local ffi = require("ffi")
ffi.cdef[[
    char *grapheme();
]]

local grapheme = ffi.load("grapheme")
