use libc;
use RedisResult;
use redis::Context;
use raw;
use std::slice;
use std::mem;
pub struct Command<F: Fn(&Context, &[&str]) -> RedisResult> {
    pub name: &'static str,
    pub handler: F,
    pub flags: &'static str,
}

impl<F: Fn(&Context, &[&str]) -> RedisResult> Command<F> {
    pub fn new(name: &'static str, handler: F, flags: &'static str) -> Command<F> {
        Command {
            name: name,
            handler: handler,
            flags: flags,
        }
    }


    pub fn wrap_handler(&self) -> raw::RedisModuleCmdFunc {
        // super funky
        extern "C" fn do_command<F: Fn(&Context, &[&str]) -> RedisResult>
            (ctx: *mut raw::RedisModuleCtx,
            argv: *mut *mut raw::RedisModuleString,
            argc: libc::c_int)
            -> raw::Status {
            unsafe {
                let cmd: *const F = mem::transmute(&());
                let args = slice::from_raw_parts(argv, argc as usize);
                Context::new(ctx).reply((*cmd)(&Context::new(ctx), &[]))

            }
        }
        assert!(mem::size_of::<F>() == 0);
        do_command::<F> as _
    }
}
