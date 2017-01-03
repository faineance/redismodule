//! # redismodule
//!
//! Write Redis modules in Rust.
//!
//! #![deny(missing_docs)]
#![feature(untagged_unions)]
#![feature(conservative_impl_trait)]
#[macro_use]
extern crate bitflags;
extern crate libc;
pub mod command;
pub mod redis;
pub use redis::{RedisResult, RedisValue, RedisError, Context};
pub mod raw;

#[macro_export]
macro_rules! redis_module (
    ($name:expr, $module_version:expr, $commands:expr) => (

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn RedisModule_OnLoad(ctx: *mut redismodule::raw::RedisModuleCtx) -> redismodule::raw::Status {
            unsafe {
              if redismodule::raw::RedisModule_Init(ctx,format!("{}\0",$name).as_ptr() as *const i8, $module_version, redismodule::raw::REDISMODULE_APIVER_1) == redismodule::raw::Status::Err {
                  return redismodule::raw::Status::Err;
              }
              for command in $commands.into_iter() {
                if redismodule::raw::RedisModule_CreateCommand(ctx,
                                                            format!("{}\0", command.name).as_ptr() as *const i8,
                                                            command.wrap_handler(),
                                                            format!("{}\0", command.flags).as_ptr() as *const i8,
                                                            1,
                                                            1,
                                                            1) == redismodule::raw::Status::Err {
                    return redismodule::raw::Status::Err;
                }
              }
              return redismodule::raw::Status::Ok;
            }
        }
    )
);
