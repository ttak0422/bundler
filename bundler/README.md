```
packages/<id>                 → `return {"foo-pname","bar-pname"}`
startup_configs/<id>          → `vim.cmd([[set cmdheight=1]])`
pre_configs/<id>              → `vim.g.bar = 1`
post_configs/<id>             → `require("foo").setup()`
modules/<module_name>         → `return {1,2,3}`
events/<event_name>           → `return {1,2,3}`
user_events/<user_event_name> → `return {1,2,3}`
commands/<command_name>       → `return {1,2,3}`
module_keys                   → `return {"foo-module","bar-module"}`
event_keys                    → `return {"foo-event","bar-event"}`
user_event_keys               → `return {"foo-user-event","bar-user-event"}`
startup_config_keys           → `return {1,2,3}`
denops_keys                   → `return {1,2,3}`
after/<XXXX (e.g. ftplugin)>/<YYYY (e.g. rust.lua)> → `-- some config`
```
