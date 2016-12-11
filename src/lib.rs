#![feature(untagged_unions)]
#![feature(conservative_impl_trait)]
#[macro_use]
extern crate bitflags;
extern crate libc;
use std::slice;
pub mod raw;
pub mod command;

#[macro_export]
macro_rules! redis_module (
    ($name:expr,$module_version:expr, $commands:expr) => (

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



pub fn wrap_command<F: Fn(*mut raw::RedisModuleCtx, &[&str]) -> RedisResult>
    (command: F)
     -> raw::RedisModuleCmdFunc {
    extern "C" fn do_command<F: Fn(*mut raw::RedisModuleCtx, &[&str]) -> RedisResult>
        (ctx: *mut raw::RedisModuleCtx,
         argv: *mut *mut raw::RedisModuleString,
         argc: libc::c_int)
         -> raw::Status {
        unsafe {
            let cmd: *const F = std::mem::transmute(&());
            let args = slice::from_raw_parts(argv, argc as usize);
            match (*cmd)(ctx, &[]) {
                Ok(RedisValue::Integer(v)) => raw::RedisModule_ReplyWithLongLong(ctx, v),
                Ok(RedisValue::String(s)) => unimplemented!(),
                Ok(RedisValue::Array(v)) => unimplemented!(),
                Err(RedisError::WrongArity) => raw::RedisModule_WrongArity(ctx),
                Err(RedisError::String(s)) => {
                    raw::RedisModule_ReplyWithError(ctx, s.as_ptr() as *const i8)
                }
            }
        }
    }
    assert!(std::mem::size_of::<F>() == 0);
    do_command::<F> as _
}


pub type RedisResult = Result<RedisValue, RedisError>;

#[derive(Debug)]
pub enum RedisError {
    WrongArity,
    String(&'static str),
}

#[derive(Debug)]
pub enum RedisValue {
    String(String),
    Integer(i64),
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
