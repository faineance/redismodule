use raw;
use RedisResult;

pub struct Command<'a, F: Fn(*mut raw::RedisModuleCtx, &[&str]) -> RedisResult> {
    pub name: &'a str,
    pub handler: F,
    pub flags: &'a str,
}
