use raw;
use RedisResult;
pub struct Command<'a> {
    pub name: &'a str,
    pub handler: fn(ctx: *mut raw::RedisModuleCtx,
        argv: *mut *mut raw::RedisModuleString,
        argc: ::std::os::raw::c_int)
        -> raw::Status,
    pub flags: &'a str,
}
impl<'a> Command<'a> {
    pub fn run(ctx: *mut raw::RedisModuleCtx,
               argv: *mut *mut raw::RedisModuleString,
               argc: ::std::os::raw::c_int)
               -> raw::Status {


        return raw::Status::Ok;
    }
}