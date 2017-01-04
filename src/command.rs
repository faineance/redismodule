use libc;
use RedisResult;
use redis::Context;
use raw;
use std::slice;
use std::mem;
use redis::RedisString;
pub struct Command<F: Fn(&Context, Vec<String>) -> RedisResult> {
    pub name: &'static str,
    pub handler: Box<F>,
    pub flags: &'static str,
}

impl<F: Fn(&Context, Vec<String>) -> RedisResult> Command<F> {
    pub fn new(name: &'static str, handler: F, flags: &'static str) -> Command<F> {
        Command {
            name: name,
            handler: Box::new(handler),
            flags: flags,
        }
    }
    pub fn wrap_handler(&self) -> raw::RedisModuleCmdFunc {
        // super funky
        extern "C" fn do_command<F: Fn(&Context, Vec<String>) -> RedisResult>
            (ctx: *mut raw::RedisModuleCtx,
            argv: *mut *mut raw::RedisModuleString,
            argc: libc::c_int)
            -> raw::Status {
            unsafe {
                // let cmd: *const F = mem::transmute(&());
                let cmd: *const F = &() as *const () as *const F;
                let context = Context::new(ctx);

                let args: Vec<String> = slice::from_raw_parts(argv, argc as usize)
                    .into_iter()
                    .map(|a| RedisString::from_ptr(*a).unwrap().to_string())
                    .collect();

                context.reply((*cmd)(&context, args))
            }
        }
        assert!(mem::size_of::<F>() == 0);
        do_command::<F> as _
    }
}
