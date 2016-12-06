#![feature(untagged_unions)]

#[macro_use]
extern crate bitflags;
extern crate libc;
pub mod raw;

#[macro_export]
macro_rules! REDIS_MODULE (
    ($name:expr, $module_version:expr) => (
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn RedisModule_OnLoad(ctx: *mut raw::RedisModuleCtx) -> raw::Status {
            unsafe {
              if raw::Export_RedisModule_Init(ctx,CString::new($name).unwrap().as_ptr(), $module_version, raw::REDISMODULE_APIVER_1) ==  raw::Status::Err {
                  return raw::Status::Err;
              }
              return raw::Status::Ok;
            }

        }
    )
);
