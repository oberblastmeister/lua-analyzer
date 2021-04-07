local one_index = tbl[34]
local two_index = tbl[23][10]
local index_expr = tbl[23^23 / 12]["hello person"][function(hello, this) print('hello') end]
local index_call = fn("hello")['index']()
