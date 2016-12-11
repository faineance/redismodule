#[macro_use]
extern crate redismodule;
use redismodule::raw;
use redismodule::{RedisResult, RedisValue};
use std::slice;
use redismodule::command::Command;


fn echo(ctx: *mut raw::RedisModuleCtx, args: &[&str]) -> RedisResult {
    println!("asd");
    return Ok(RedisValue::String(String::from("hehe")));
    // return Err("oops");
}


redis_module!("example",
              1,
              vec![Command {
                       name: "hehe",
                       handler: echo,
                       flags: "",
                   }]);
