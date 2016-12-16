
use RedisResult;
use redis::Context;


pub struct Command< F: Fn(&Context, &[&str]) -> RedisResult> {
    pub name: &'static str,
    pub handler: F,
    pub flags: &'static str,
}

