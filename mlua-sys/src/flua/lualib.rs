//! Contains definitions from `lualib.h`.

use std::os::raw::{c_char, c_int};

use super::lua::lua_State;

pub const LUA_TABLIBNAME: *const c_char = cstr!("table");
pub const LUA_STRLIBNAME: *const c_char = cstr!("string");
pub const LUA_BITLIBNAME: *const c_char = cstr!("bit32");
pub const LUA_MATHLIBNAME: *const c_char = cstr!("math");
pub const LUA_DBLIBNAME: *const c_char = cstr!("debug");
pub const LUA_LOADLIBNAME: *const c_char = cstr!("package");

#[cfg_attr(all(windows, raw_dylib), link(name = "flua", kind = "raw-dylib"))]
unsafe extern "C-unwind" {
    pub fn luaopen_base(L: *mut lua_State) -> c_int;
    pub fn luaopen_table(L: *mut lua_State) -> c_int;
    pub fn luaopen_string(L: *mut lua_State) -> c_int;
    pub fn luaopen_bit32(L: *mut lua_State) -> c_int;
    pub fn luaopen_math(L: *mut lua_State) -> c_int;
    pub fn luaopen_fulldebug(L: *mut lua_State) -> c_int;
    pub fn luaopen_partialdebug(L: *mut lua_State) -> c_int;

    // open all builtin libraries
    pub fn luaL_openlibs(L: *mut lua_State, full_debug: c_int);
}
