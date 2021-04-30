
return {
  formatters = {
    ["lua-format"] = {
      command = "lua-format",
      args = {"%filepath", "--config", vim.fn.expand("~/.config/luaformatter/config.yml")},
    },
  },
}
