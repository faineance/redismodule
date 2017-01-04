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
        use redismodule::raw;
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn RedisModule_OnLoad(ctx: *mut raw::RedisModuleCtx) -> raw::Status {
            unsafe {
              if raw::RedisModule_Init(ctx,format!("{}\0",$name).as_ptr() as *const i8, $module_version, raw::REDISMODULE_APIVER_1) == raw::Status::Err {
                  return raw::Status::Err;
              }
              for command in &$commands {
                if raw::RedisModule_CreateCommand(ctx,
                                                            format!("{}\0", command.name).as_ptr() as *const i8,
                                                            command.wrap_handler(),
                                                            format!("{}\0", command.flags).as_ptr() as *const i8,
                                                            1,
                                                            1,
                                                            1) == raw::Status::Err {
                    return raw::Status::Err;
                }
              }
             raw::Status::Ok
            }
        }
    )
);
