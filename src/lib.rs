#![feature(untagged_unions)]

#[macro_use]
extern crate bitflags;
extern crate libc;

pub mod raw;
pub mod command;


#[macro_export]
macro_rules! REDIS_MODULE (
    ($name:expr,$module_version:expr, $commands:expr) => (

        use redismodule::raw;
        use std::ptr;
        extern fn test(ctx: *mut raw::RedisModuleCtx,argv: *mut *mut raw::RedisModuleString,argc: ::std::os::raw::c_int) -> raw::Status {
            println!("Nailed it");
            raw::Status::Ok
        }
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn RedisModule_OnLoad(ctx: *mut raw::RedisModuleCtx) -> raw::Status {
            unsafe {
              if raw::Export_RedisModule_Init(ctx,format!("{}\0",$name).as_ptr() as *const i8, $module_version, raw::REDISMODULE_APIVER_1) == raw::Status::Err {
                  return raw::Status::Err;
              }
              for command in $commands {
                    if raw::RedisModule_CreateCommand(ctx,format!("{}\0",command).as_ptr() as *const i8, test,format!("write\0").as_ptr() as *const i8,1,1,1) == raw::Status::Err {
                        return raw::Status::Err;
                    }
              }
              return raw::Status::Ok;
            }
        }
    )
);
