//! Contains definitions from `lualib.h`.

use std::os::raw::c_int;

use super::lua::lua_State;

pub const LUA_TABLIBNAME: &str = "table";
pub const LUA_STRLIBNAME: &str = "string";
pub const LUA_BITLIBNAME: &str = "bit32";
pub const LUA_MATHLIBNAME: &str = "math";
pub const LUA_DBLIBNAME: &str = "debug";
pub const LUA_LOADLIBNAME: &str = "package";

#[cfg_attr(all(windows, raw_dylib), link(name = "flua", kind = "raw-dylib"))]
extern "C-unwind" {
    pub fn luaopen_base(L: *mut lua_State) -> c_int;
    pub fn luaopen_table(L: *mut lua_State) -> c_int;
    pub fn luaopen_string(L: *mut lua_State) -> c_int;
    pub fn luaopen_bit32(L: *mut lua_State) -> c_int;
    pub fn luaopen_math(L: *mut lua_State) -> c_int;
    pub fn luaopen_fulldebug(L: *mut lua_State) -> c_int;
    pub fn luaopen_partialdebug(L: *mut lua_State) -> c_int;
    pub fn luaopen_package(L: *mut lua_State) -> c_int;

    // open all builtin libraries
    pub fn luaL_openlibs(L: *mut lua_State, full_debug: c_int);
}
