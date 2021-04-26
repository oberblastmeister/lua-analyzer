local function person(...)
  print(...)
  local hello = ... + 1
  print(..., "another", ...)
end
