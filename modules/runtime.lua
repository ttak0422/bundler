---@class Options
---@field root string
---@field log_level 0 | 1 | 2 | 3
---@type fun(opt: Options)
(function(opt)
	-- logger. inspired by ttak0422/LL.
	local log
	do
		local path = (vim.fn.stdpath("data") .. "/" .. "bundler.log")
		local function to_str(...)
			local acc = {}
			for i = 1, select("#", ...) do
				local v = select(i, ...)
				acc[#acc + 1] = type(v) == "table" and vim.inspect(v) or tostring(v)
			end
			return table.concat(acc, " ")
		end
		local M = {}
		for lv, name in ipairs({ "debug", "info", "warn", "error" }) do
			M[name] = function(...)
				if lv >= opt.log_level then
					local i = debug.getinfo(2, "Sl")
					local msg = string.format(
						"[%-6s%s] %s: %s",
						name:upper(),
						os.date("%Y-%m-%d %H:%M:%S"),
						(i.short_src .. ":" .. i.currentline),
						to_str(...)
					)
					local f = io.open(path, "a")
					if f ~= nil then
						f:write(msg)
						f:close()
					end
					vim.notify(msg, lv)
				end
			end
		end
		log = M
	end

	log.debug("STARTED")

	-- execute startup
	for _, id in ipairs(dofile(opt.root .. "/startup_config_keys")) do
		log.debug("[startup] start:", id)
		local ok, err = pcall(dofile, opt.root .. "/startup_configs/" .. id)
		if not ok then
			log.error(id, "startup error:", err)
		end
		log.debug("[startup] end:", id)
	end

	-- setup lazy loaders
	local au = vim.api.nvim_create_autocmd
	local denops_ids = dofile(opt.root .. "/denops_keys")
	local function load_denops(id)
		for _, rtp in ipairs(dofile(opt.root .. "/rtp/" .. id)) do
			local candidates = vim.fn.globpath(rtp, "denops/*/main.ts", true, true)
			for _, c in ipairs(candidates) do
				local name = vim.fn.fnamemodify(c, ":h:t")
				local ok, status = pcall(vim.fn["denops#server#status"])
				if not ok then
					log.error(id, "load error: `denops.vim` has not been loaded yet.")
					return
				end
				if status == "running" then
					ok = pcall(vim.fn["denops#plugin#load"], name, rtp .. "/denops/" .. name .. "/main.ts")
					if not ok then
						log.error(name, "failed to denops plugin load.")
					end
				end
				vim.fn["denops#plugin#wait"](name)
			end
		end
	end

	local function config(id, is_pre)
		log.debug(is_pre and "[pre_config]" or "[post_config]", "start", id)
		local dir = is_pre and "/pre_configs/" or "/post_configs/"
		local ok, err = pcall(dofile, opt.root .. dir .. id)
		if not ok then
			log.error(id, "configure error:", err)
		end
		log.debug(is_pre and "[pre_config]" or "[post_config]", "end", id)
	end

	local M = {}
	M.loaded_plugins = {}
	M.loaded_modules = {}
	M.load_plugins = function(self, ids)
		for _, id in ipairs(ids) do
			self:load_plugin(id)
		end
	end
	M.load_plugin = function(self, id)
		if not self.loaded_plugins[id] then
			log.debug("[load] start", id)
			self.loaded_plugins[id] = true
			config(id, true)
			self:load_plugins(dofile(opt.root .. "/depends/" .. id))
			for _, p in ipairs(dofile(opt.root .. "/packages/" .. id)) do
				vim.cmd("packadd " .. p)
			end
			if denops_ids[id] then
				load_denops(id)
			end
			config(id, false)
			log.debug("[load] end", id)
		end
	end

	package.preload["bundler"] = M

	-- register autocmds
	for _, ev in ipairs(dofile(opt.root .. "/event_keys")) do
		log.debug("[event]", ev)
		au(ev, {
			pattern = "*",
			once = true,
			callback = function()
				M:load_plugins(dofile(opt.root .. "/events/" .. ev))
			end,
		})
	end
	for _, ev in ipairs(dofile(opt.root .. "/user_event_keys")) do
		log.debug("[user_event]", ev)
		au("User", {
			pattern = ev,
			once = true,
			callback = function()
				M:load_plugins(dofile(opt.root .. "/user_events/" .. ev))
			end,
		})
	end
	au("CmdUndefined", {
		pattern = "*",
		callback = function(ev)
			local ok, ids = pcall(dofile, opt.root .. "/commands/" .. ev.match)
			if ok then
				M:load_plugins(ids)
			end
		end,
	})
	-- module loader interception
	table.insert(package.loaders, 1, function(mod_name)
		if not M.loaded_modules[mod_name] then
			M.loaded_modules[mod_name] = true
			local ok, ids = pcall(dofile, opt.root .. "/modules/" .. mod_name)
			if ok then
				for _, id in ipairs(ids) do
					M:load_plugin(id)
				end
			end
		end
	end)

	log.debug("COMPLETED")
end)(REPLACED_BY_NIX)
