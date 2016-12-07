#![feature(untagged_unions)]

#[macro_use]
extern crate bitflags;
extern crate libc;

pub mod raw;
pub mod command;


#[macro_export]
macro_rules! REDIS_MODULE (
    ($name:expr,$module_version:expr, $commands:expr) => (

        use std::ptr;
        
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn RedisModule_OnLoad(ctx: *mut redismodule::raw::RedisModuleCtx) -> redismodule::raw::Status {
            unsafe {
              if redismodule::raw::Export_RedisModule_Init(ctx,format!("{}\0",$name).as_ptr() as *const i8, $module_version, redismodule::raw::REDISMODULE_APIVER_1) == redismodule::raw::Status::Err {
                  return redismodule::raw::Status::Err;
              }
              for (name,function) in $commands {

                    if redismodule::raw::RedisModule_CreateCommand(ctx,format!("{}\0",name).as_ptr() as *const i8, function,format!("write\0").as_ptr() as *const i8,1,1,1) == redismodule::raw::Status::Err {
                        return redismodule::raw::Status::Err;
                    }
              }
              return redismodule::raw::Status::Ok;
            }
        }
    )
);
