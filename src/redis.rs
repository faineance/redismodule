use raw;
pub type RedisResult = Result<RedisValue, RedisError>;

#[derive(Debug)]
pub enum RedisError {
    WrongArity,
    String(&'static str),
}

#[derive(Debug)]
pub enum RedisValue {
    String(&'static str),
    Integer(i64),
    Array(Vec<RedisValue>),

}

pub struct Context {
    ctx: *mut raw::RedisModuleCtx,
}

impl Context {
    pub fn new(ctx: *mut raw::RedisModuleCtx) -> Context {
        Context { ctx: ctx }
    }
}

#[derive(Debug)]
pub struct RedisString {
    ctx: *mut raw::RedisModuleCtx,
    pub inner: *mut raw::RedisModuleString,
}

impl RedisString {
    pub fn new(ctx: *mut raw::RedisModuleCtx, s: &str) -> RedisString {
        let inner =
            raw::RedisModule_CreateString(ctx, format!("{}\0", s).as_ptr() as *const i8, s.len());
        RedisString {
            ctx: ctx,
            inner: inner,
        }
    }
}

impl Drop for RedisString {
    fn drop(&mut self) {
        raw::RedisModule_FreeString(self.ctx, self.inner);
    }
}
