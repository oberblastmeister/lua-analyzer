# lua-analyzer

lua-analyzer is a lsp server for lua. This is mostly for me to learn the lsp protocol and language analysis so suggestions are helpful. This server is highly inspired by rust-analyzer and has a lot of code that is similar or copied from it. Currently only the parser is (partially) implemented. Please don't use this yet.

## Installation

```sh
git clone https://github.com/oberblastmeister/lua-analyzer
cd lua-analyzer
cargo install --path ./crates/lua-analyzer
```

## neovim lspconfig

First add it to your configurations

```lua
local configs = require("lspconfig/configs")
configs.lua_analyzer = {
  default_config = {
    cmd = { "lua-analyzer" },
      filetypes = {"lua"},
      root_dir = util.root_pattern("*.lua", vim.fn.getcwd()),
      docs = {
        description = [[]],
        default_config = {root_dir = [[root_pattern(".git", vim.fn.getcwd())]]},
    },
  }
}
```

Then you can just set it up as usual

```lua
lspconfig.lua_analyzer.setup { ... }
```

## Lsp features

- [x] simple syntax diagnostics
- [ ] completion
- [ ] symbols
- [ ] workspace symbols
- [ ] renaming
- [ ] goto definition
- [ ] hover
