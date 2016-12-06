#[macro_use]
extern crate redismodule;
extern crate libc;
use std::ffi::CString;
use redismodule::raw;


REDIS_MODULE!("example", 1);
