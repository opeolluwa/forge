local dir = (arg[0]:match("^(.*[/\\])") or "")
package.path = dir .. "?.lua;" .. package.path
require("help").check(arg)

print("create a justfile in the present dir")
