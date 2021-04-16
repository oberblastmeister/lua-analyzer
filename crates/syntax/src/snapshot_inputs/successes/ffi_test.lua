ffi.cdef[[
typedef enum enum_i { FOO_I = -1, II = 10 } enum_i;
typedef enum enum_u { FOO_U = 1, UU = 10 } enum_u;

enum_i call_ei_i(int a) asm("call_i");
enum_u call_eu_i(int a) asm("call_i");
int call_i_ei(enum_i a) asm("call_i");
int call_i_eu(enum_u a) asm("call_i");
]]
