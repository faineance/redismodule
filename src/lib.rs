//! # redismodule
//!
//! Write Redis modules in Rust.
//!
#![feature(untagged_unions)]
#![feature(conservative_impl_trait)]
#[macro_use]
extern crate bitflags;
extern crate libc;
use std::slice;

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
              for command in $commands {
                if redismodule::raw::RedisModule_CreateCommand(ctx,
                                                            format!("{}\0", command.name).as_ptr() as *const i8,
                                                            redismodule::wrap_command(command.handler),
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



pub fn wrap_command<F: Fn(&Context, &[&str]) -> RedisResult>(command: F)
                                                             -> raw::RedisModuleCmdFunc {
    // super funky
    extern "C" fn do_command<F: Fn(&Context, &[&str]) -> RedisResult>
        (ctx: *mut raw::RedisModuleCtx,
         argv: *mut *mut raw::RedisModuleString,
         argc: libc::c_int)
         -> raw::Status {
        unsafe {
            let cmd: *const F = std::mem::transmute(&());
            let args = slice::from_raw_parts(argv, argc as usize);

            reply(ctx, (*cmd)(&Context::new(ctx), &[]))
        }
    }
    assert!(std::mem::size_of::<F>() == 0);
    do_command::<F> as _
}



fn reply(ctx: *mut raw::RedisModuleCtx, r: RedisResult) -> raw::Status {
    match r {
        Ok(RedisValue::Integer(v)) => raw::RedisModule_ReplyWithLongLong(ctx, v),
        Ok(RedisValue::String(s)) => {
            raw::RedisModule_ReplyWithString(ctx, redis::RedisString::new(ctx, s).inner)
        }
        Ok(RedisValue::Array(array)) => {
            raw::RedisModule_ReplyWithArray(ctx, array.len() as libc::c_long);

            for elem in array {
                reply(ctx, Ok(elem));
            }

            return raw::Status::Ok;
        }
        Err(RedisError::WrongArity) => raw::RedisModule_WrongArity(ctx),
        Err(RedisError::String(s)) => raw::RedisModule_ReplyWithError(ctx, s.as_ptr() as *const i8),
    }
}
