local commit_kinds = { feat = true, chore = true, refactor = true, update = true }

io.write("Commit kind? ")
local kind = io.read():match("^%s*(.-)%s*$")

if not commit_kinds[kind] then
    local sorted = {}
    for k in pairs(commit_kinds) do sorted[#sorted + 1] = k end
    table.sort(sorted)
    print("Invalid commit kind. Use one of: " .. table.concat(sorted, ", "))
    os.exit(1)
end

io.write("Commit title? ")
local commit_title = io.read():match("^%s*(.-)%s*$")

io.write("Commit message? ")
local commit_message = io.read():match("^%s*(.-)%s*$")

local message = string.format("%s(%s):\n\n%s\n", kind, commit_title, commit_message)

print(message)

local pathspec = arg and arg[1]

if pathspec then
    os.execute("git add " .. pathspec)
else
    io.write("No pathspec given — stage all changes? [y/N] ")
    local confirm = io.read():match("^%s*(.-)%s*$"):lower()
    if confirm ~= "y" and confirm ~= "yes" then
        print("Aborted. Pass a pathspec pattern (e.g. '*.rs' or 'src/') to stage specific files.")
        os.exit(1)
    end
    os.execute("git add .")
end

os.execute("git commit -m " .. string.format("%q", message))
