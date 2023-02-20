## lua2json

`lua2json` is a simple tool to convert Lua tables to JSON.

It reads *lines* from `stdin` and writes to `stdout`.

It consumes tables in `serpent` format, which is also the input
format for literal tables in the Lua language.

e.g., given a line containing:

```lua
{foo = "bar", baz = { qux = "quux" }}
```

..it will print:
```json
{"foo":"bar","baz":{"qux":"quux"}}
```

This is quite similar to:

```lua
json=require "json"
for s in io.open("/dev/stdin", "r"):read("a"):gmatch("[^\n]+") do
    load("g="..s)()
    print(json.encode(g))
end
```

...except it doesn't execute arbitrary code (and it happens to be 6x faster
and order-preserving, but if you care about that, you probably need help.)

The behaviour on mixed dict/arrays is MADE UP. If you have a usecase, please raise an issue.


### License

MIT / Apache-2.0
