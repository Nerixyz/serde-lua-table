# serde-lua-table

Serialize values in lua representation.

It's basically `serde_json` but emitting lua tables.

## Usage

```toml
serde-lua-table = { git = "https://github.com/Nerixyz/serde-lua-table.git", tag = "v0.1.1" }
```

```rust
serde_lua_table::to_string(&value);
serde_lua_table::to_string_pretty(&value);
```

## Example

Using mlua's `Value` and [`test_example.lua`](test_example.lua) we get:

```lua
{
  [4] = "a",
  ["xd"] = {
    ["array"] = {
      1,
      2,
      3,
      4
    },
    ["forsen"] = 5,
    ["combined"] = {
      0,
      1
    }
  }
}
```

_Note that in `combined` we also added `["a"]` which wasn't serialized,
this is because of the `Serialize` implementation on `mlua::Value` which didn't include it._