#[macro_use]
extern crate redismodule;

use redismodule::{RedisResult, RedisValue, Context};
use redismodule::command::Command;


fn array(ctx: &Context, args: Vec<String>) -> RedisResult {

    let mut resp = vec![RedisValue::Integer(args.len() as i64)];
    let resp_args: Vec<RedisValue> = args.into_iter()
        .map(RedisValue::String)
        .collect();

    resp.extend(resp_args);
    
    Ok(RedisValue::Array(resp))
}

// fn wrong_arity(ctx: &Context, args: &[&str]) -> RedisResult {
//     Err(RedisError::WrongArity)
// }
// fn err_msg(ctx: &Context, args: &[&str]) -> RedisResult {
//     Err(RedisError::String("sample text"))
// }

// redis_module!("example", 1, [Command::new("array", array, ""), Command::new("box", wrong_arity, "")]);

redis_module!("example", 1, [Command::new("array", array, "")]);
