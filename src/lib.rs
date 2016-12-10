#![feature(untagged_unions)]
#![feature(conservative_impl_trait)]
#[macro_use]
extern crate bitflags;
extern crate libc;
use std::slice;
pub mod raw;
pub mod command;




#[macro_export]
macro_rules! REDIS_MODULE (
    ($name:expr,$module_version:expr, $commands:expr) => (

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn RedisModule_OnLoad(ctx: *mut redismodule::raw::RedisModuleCtx) -> redismodule::raw::Status {
            unsafe {
              if redismodule::raw::RedisModule_Init(ctx,format!("{}\0",$name).as_ptr() as *const i8, $module_version, redismodule::raw::REDISMODULE_APIVER_1) == redismodule::raw::Status::Err {
                  return redismodule::raw::Status::Err;
              }
              for (name,function) in $commands {


                if redismodule::raw::RedisModule_CreateCommand(ctx,
                                                            format!("{}\0", name).as_ptr() as *const i8,
                                                            redismodule::wrap_command(function),
                                                            format!("write\0").as_ptr() as *const i8,
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



pub fn wrap_command(command: fn(ctx: *mut raw::RedisModuleCtx, args: &[&str]) -> RedisResult)
                -> impl Fn(*mut raw::RedisModuleCtx, *mut *mut raw::RedisModuleString, libc::c_int) -> raw::Status{

    move |ctx: *mut raw::RedisModuleCtx,
          argv: *mut *mut raw::RedisModuleString,
          argc: libc::c_int|
          -> raw::Status {
        match command(ctx, &["a"]) {
            Ok(redisvalue) => raw::Status::Ok,
            Err(e) => raw::Status::Err,
        }
    }
}

pub type RedisError = &'static str;
pub type RedisResult = Result<RedisValue, RedisError>;


#[derive(Debug)]
pub enum RedisValue {
    String(String),
    Integer(isize),
    Array(Vec<RedisValue>),
}



// pub struct Context {
//     ctx: *mut raw::RedisModuleCtx,
// }

// impl Context {
//      pub fn name(arg: Type) -> RetType {
//          unimplemented!();
//      }
// }



//  #[derive(Debug)]

// pub enum RedisError {
//     Error(String),
//     WrongArity,
// }