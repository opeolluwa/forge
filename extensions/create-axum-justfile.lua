local dir = (arg[0]:match("^(.*[/\\])") or "")
package.path = dir .. "?.lua;" .. package.path
require("help").check(arg)

local JUSTFILE_CONTENT = [=[# Alias
alias install := install-deps
alias config:= configure
alias d := dev
alias c := clean
alias rs := restart
alias rb := rebuild
alias lt := lint
alias lg := logs
alias s := stop

set dotenv-required := true
set dotenv-load := true
set dotenv-path := ".env"
set export := true

# constants
DOCKER_CMD := "docker compose -f docker-compose.yaml"


# Default Shows the default commands
@default:
    @just --list --list-heading $'Available commands\n'

# format the code
@lint:
    cargo fmt
    cargo group-imports --fix
    cargo sort -w

@dev:
    {{ DOCKER_CMD }} up -d
    @just logs

# see docker logs, this is called internally when you run just dev
@logs:
    {{ DOCKER_CMD }} logs -f --tail='30' app


# destroy the running docker instance and clean the cache
@kill:
    {{ DOCKER_CMD }} down -v

# stop the running docker instance without cleaning the cache, called internally when you restart the project
@stop:
    {{ DOCKER_CMD }} down

# stop and start the project without removing cache and local data
restart:
    @just stop
    @just dev

# delete the project, the cached data, target dir and restart
@rebuild:
    @just kill
    @just clean
    {{ DOCKER_CMD }} up --build  -d
    @just logs


#execute all initial setup after cloning the project
@configure:
    @just install-deps
    cp .env.example .env


#remove the target dir from local file system
@clean:
    cargo clean


#install the local dependencies
@install-deps:
    cargo install sea-orm-cli@^2.0.0-rc
    cargo install cargo-sort
    cargo install cargo-group-imports



[group('migration')]
@migrate-add target:
    @sea-orm-cli migrate generate "{{target}}"

@generate-entities:
    sea-orm-cli generate entity --database-url="$DATABASE_URL" --with-serde both -o src/entities
]=]

local function is_dir(path)
    -- os.rename on a path to itself succeeds for both files and dirs, but
    -- trying to open a directory with io.open returns nil, letting us distinguish.
    local ok, _, code = os.rename(path, path)
    if not ok then
        -- code 13 = EACCES (permission denied) means the path exists but we can't rename it;
        -- treat that as present (it exists, and on most systems that means it's a dir we can write into).
        return code == 13
    end
    -- Confirm it's a directory and not a regular file.
    local probe = io.open(path, "r")
    if probe then
        probe:close()
        return false -- opened as a file, so it's not a directory
    end
    return true
end

io.write("Enter directory to create Justfile in: ")
local target_dir = io.read():match("^%s*(.-)%s*$")

if target_dir == "" then
    print("Error: No directory provided")
    os.exit(1)
end

if not is_dir(target_dir) then
    print("Error: '" .. target_dir .. "' is not a valid directory")
    os.exit(1)
end

local justfile_path = target_dir .. "/Justfile"

local existing = io.open(justfile_path, "r")
if existing then
    existing:close()
    print("Error: Justfile already exists at " .. justfile_path)
    os.exit(1)
end

local f, err = io.open(justfile_path, "w")
if not f then
    print("Failed to write Justfile: " .. err)
    os.exit(1)
end

f:write(JUSTFILE_CONTENT)
f:close()

print("Justfile successfully created at " .. justfile_path)
