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
    String(String),
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


    pub fn call(&self, command: &str, args: &[&str]) -> RedisResult {
        let terminated_args: Vec<RedisString> = args.iter()
            .map(|s| RedisString::new(self.ctx, s))
            .collect();

        return Err(RedisError::String("not implemented"));
    }
    pub fn reply(&self, r: RedisResult) -> raw::Status {
        match r {
            Ok(RedisValue::Integer(v)) => unsafe {
                raw::RedisModule_ReplyWithLongLong(self.ctx, v)
            },
            Ok(RedisValue::String(s)) => {
                unsafe {
                    raw::RedisModule_ReplyWithString(self.ctx,
                                                     RedisString::new(self.ctx, s.as_ref()).inner)
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
        RedisString {
            ctx: ctx,
            inner: unsafe {
                raw::RedisModule_CreateString(ctx,
                                              format!("{}\0", s).as_ptr() as *const i8,
                                              s.len())
            },
        }
    }

    // probably not the best naming, or interface
    pub fn as_str<'a>(&self) -> Result<&'a str, str::Utf8Error> {
        RedisString::from_ptr(self.inner)
    }

    pub fn from_ptr<'a>(ptr: *mut raw::RedisModuleString) -> Result<&'a str, str::Utf8Error> {
        let mut len: libc::size_t = 0;
        let bytes = unsafe { raw::RedisModule_StringPtrLen(ptr, &mut len) };

        str::from_utf8(unsafe { slice::from_raw_parts(bytes as *const u8, len) })
    }
}

impl Drop for RedisString {
    fn drop(&mut self) {
        unsafe {
            raw::RedisModule_FreeString(self.ctx, self.inner);
        }

    }
}
