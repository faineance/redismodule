#[macro_use]
extern crate redismodule;
use redismodule::raw;
use redismodule::{RedisResult, RedisValue, RedisError, Context};
use std::slice;
use redismodule::command::Command;


fn array(ctx: &Context, args: &[&str]) -> RedisResult {
    return Ok(RedisValue::Array(vec![RedisValue::Integer(1),
                                     RedisValue::String("asd"),
                                     RedisValue::String("asd"),
                                     RedisValue::String("asd"),
                                     RedisValue::String("asd")]));

}

fn wrong_arity(ctx: &Context, args: &[&str]) -> RedisResult {
    Err(RedisError::WrongArity)
}
fn err_msg(ctx: &Context, args: &[&str]) -> RedisResult {
    Err(RedisError::String("sample text"))
}


redis_module!("example", 1, [Command::new("array", array, "")]);
