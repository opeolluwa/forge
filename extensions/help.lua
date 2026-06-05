local M = {}

function M.check(args)
    if not (args and args[1] == "--help") then return end

    local script = args[0]:gsub("%.lua$", "")
    local f = io.open(script .. ".yaml", "r")
    if not f then
        print("No docs found: " .. script .. ".yaml")
        os.exit(0)
    end

    local lines = {}
    for line in f:lines() do lines[#lines + 1] = line end
    f:close()

    local name, desc, args_list, cmds = "", "", {}, {}
    local section, cur = nil, nil

    for _, line in ipairs(lines) do
        if line:match("^name:") then
            name = line:match("^name:%s*(.*)")
        elseif line:match("^description:") then
            desc = line:match("^description:%s*(.*)")
        elseif line:match("^args:") then
            section = "args"
        elseif line:match("^commands:") then
            section = "commands"
            cur = nil
        elseif section == "args" then
            local n = line:match("^  %- name:%s*(.*)")
            if n then
                cur = { name = n }
                args_list[#args_list + 1] = cur
            elseif cur then
                local k, v = line:match("^    (%a+):%s*(.*)")
                if k then cur[k] = v end
            end
        elseif section == "commands" then
            local c = line:match("^  %- (.*)")
            if c then cmds[#cmds + 1] = c end
        end
    end

    print(name .. " -- " .. desc)

    if #args_list > 0 then
        print("\nARGUMENTS:")
        for _, a in ipairs(args_list) do
            local req = a.required == "true" and "[required]" or "[optional]"
            print(string.format("  %-16s %s  %s", a.name, a.description or "", req))
        end
    end

    if #cmds > 0 then
        print("\nCOMMAND:")
        for _, c in ipairs(cmds) do print("  " .. c) end
    end

    os.exit(0)
end

return M
