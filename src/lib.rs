#![feature(untagged_unions)]

#[macro_use]
extern crate bitflags;
extern crate libc;
pub mod raw;
pub mod command;
#[macro_export]
macro_rules! REDIS_MODULE (
    ($name:expr,$module_version:expr) => (
        use redismodule::raw;
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn RedisModule_OnLoad(ctx: *mut raw::RedisModuleCtx) -> raw::Status {
            unsafe {
              if raw::Export_RedisModule_Init(ctx,format!("{}\0",$name).as_ptr() as *const i8, $module_version, raw::REDISMODULE_APIVER_1) == raw::Status::Err {
                  return raw::Status::Err;
              }

              
              return raw::Status::Ok;
            }
        }
    )
);
