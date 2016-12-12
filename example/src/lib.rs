#[macro_use]
extern crate redismodule;
use redismodule::raw;
use redismodule::{RedisResult, RedisValue, Context};
use std::slice;
use redismodule::command::Command;


fn echo(ctx: &Context, args: &[&str]) -> RedisResult {
    return Ok(RedisValue::Array(vec![RedisValue::Integer(1),
                                     RedisValue::String("asd"),
                                     RedisValue::String("asd"),
                                     RedisValue::String("asd"),
                                     RedisValue::String("asd")]));

}


redis_module!("example",
              1,
              vec![Command {
                       name: "hehe",
                       handler: echo,
                       flags: "",
                   }]);
