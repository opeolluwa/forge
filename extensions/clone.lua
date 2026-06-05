local dir = (arg[0]:match("^(.*[/\\])") or "")
package.path = dir .. "?.lua;" .. package.path
require("help").check(arg)

io.write("Repository URL? ")
local url = io.read():match("^%s*(.-)%s*$")

if url == "" then
    print("Error: URL is required.")
    os.exit(1)
end

io.write("Folder name? (leave blank to use default) ")
local folder = io.read():match("^%s*(.-)%s*$")

local clone_cmd
local target_dir

if folder ~= "" then
    clone_cmd = string.format("git clone %s %s", string.format("%q", url), string.format("%q", folder))
    target_dir = folder
else
    -- derive folder name from URL (strip .git suffix and take last path segment)
    target_dir = url:match("([^/]+)$"):gsub("%.git$", "")
    clone_cmd = string.format("git clone %s", string.format("%q", url))
end

print("Cloning into '" .. target_dir .. "'...")
local ok = os.execute(clone_cmd)

if not ok then
    print("Error: git clone failed.")
    os.exit(1)
end

-- write a shell snippet that the toolbox runner can source to cd
local cd_file = os.getenv("DEV_TOOLBOX_CD_FILE")
if cd_file and cd_file ~= "" then
    local f = io.open(cd_file, "w")
    if f then
        f:write(target_dir)
        f:close()
    end
else
    -- fallback: print so the user can cd manually
    print("cd " .. target_dir)
end
