use raw;
use std::slice;
use std::str;
use libc;

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
    pub fn reply(&self, r: RedisResult) -> raw::Status {
        match r {
            Ok(RedisValue::Integer(v)) => unsafe {
                raw::RedisModule_ReplyWithLongLong(self.ctx, v)
            },
            Ok(RedisValue::String(s)) => {
                unsafe {
                    raw::RedisModule_ReplyWithString(self.ctx, RedisString::new(self.ctx, s).inner)
                }
            }
            Ok(RedisValue::Array(array)) => {
                unsafe { raw::RedisModule_ReplyWithArray(self.ctx, array.len() as libc::c_long) };

                for elem in array {
                    self.reply(Ok(elem));
                }

                raw::Status::Ok
            }
            Err(RedisError::WrongArity) => unsafe { raw::RedisModule_WrongArity(self.ctx) },
            Err(RedisError::String(s)) => unsafe {
                raw::RedisModule_ReplyWithError(self.ctx, s.as_ptr() as *const i8)
            },
        }
    }
}

#[derive(Debug)]
pub struct RedisString {
    ctx: *mut raw::RedisModuleCtx,
    pub inner: *mut raw::RedisModuleString,
}

impl RedisString {
    pub fn new(ctx: *mut raw::RedisModuleCtx, s: &str) -> RedisString {
        let inner: *mut raw::RedisModuleString;
        unsafe {
            inner = raw::RedisModule_CreateString(ctx,
                                                  format!("{}\0", s).as_ptr() as *const i8,
                                                  s.len());
        }

        RedisString {
            ctx: ctx,
            inner: inner,
        }
    }
}

impl Drop for RedisString {
    fn drop(&mut self) {
        unsafe {
            raw::RedisModule_FreeString(self.ctx, self.inner);
        }

    }
}
