---@class Options
---@field root string
---@field lazy_time number
---@field log_level '"debug"' | '"info"'| '"warn"' | '"error"'

---@class Bundler
---@field root string
---@field lazy_time number
---@field new fun(opts: Options): Bundler
---@field setup_loader fun(self: Bundler)
---@field configure fun(self: Bundler, id: string, is_pre: boolean)
---@field loaded_plugins { [string]: boolean }
---@field loaded_modules { [string]: boolean }
---@field load_plugin fun(self: Bundler, id: string)
---@field load_plugins fun(self: Bundler, path: string)

---@class LoggerLevelConfig
---@field name string
---@field hl string

---@class LoggerConfig
---@field plugin? string
---@field use_console? boolean
---@field highlights? boolean
---@field use_file? boolean
---@field level? string
---@field modes? LoggerLevelConfig[]
---@field float_percision? number

---@class Logger
---@field new fun(config: LoggerConfig, standalone: boolean)
---@field trace fun(...)
---@field debug fun(...)
---@field info fun(...)
---@field warn fun(...)
---@field error fun(...)
---@field fatal fun(...)
