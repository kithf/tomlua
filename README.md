# tomlua
Lua TOML library written in Rust

## Usage
```lua
local toml = require "tomlua"

local doc, err = toml.decode[=[
title = "Example"
working = true
]=]

if not doc then
  print(err)
end

local src = toml.encode(doc)
print(src) 
--[[
title = "Example"
working = true
--]]
```

## Docs
Note: Keys may not preserve order when encoded. This is a limitation of the TOML format.

### decode(val: `string`): `table`
Parses a TOML string into a table. On error returns nil and an error message.

```lua
local doc, err = toml.decode "a = 1"
```

### encode(val: `table`): `string`
Encodes a table into a TOML string.

```lua
local src = toml.encode {a = 1}
```
