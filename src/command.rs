use raw;
use RedisResult;
use redis::Context;
pub struct Command<'a, F: Fn(&Context, &[&str]) -> RedisResult> {
    pub name: &'a str,
    pub handler: F,
    pub flags: &'a str,
}
