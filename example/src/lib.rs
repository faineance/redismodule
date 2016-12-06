extern crate redismodule;
extern crate libc;
use std::ffi::CString;
use redismodule::raw;
#[no_mangle]
pub extern "C" fn RedisModule_OnLoad(ctx: *mut raw::RedisModuleCtx) -> raw::Status {
    unsafe {
    raw::Export_RedisModule_Init(ctx, CString::new("Hello, world!").unwrap().as_ptr(), 1, raw::REDISMODULE_APIVER_1);
    }
    return raw::Status::Ok;
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
