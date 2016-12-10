#[macro_use]
extern crate redismodule;
use redismodule::raw;
use redismodule::{RedisResult ,RedisValue};
use std::slice;

// fn echo(ctx: *mut raw::RedisModuleCtx,
//                    argv: *mut *mut raw::RedisModuleString,
//                    argc: ::std::os::raw::c_int)
//                    -> raw::Status {
//     unsafe {
//         if argc < 2 {
//             return raw::RedisModule_WrongArity(ctx);
//         }


//         return raw::RedisModule_ReplyWithString(ctx, slice::from_raw_parts(argv, argc as usize)[1]);
//     }
// }

fn echoa(ctx: *mut raw::RedisModuleCtx, args: &[&str]) -> RedisResult {
    return Ok(RedisValue::String(String::from("hehe")));
    // return Err("oops");
}


REDIS_MODULE!("example", 1, vec![("hehe", echoa)]);
