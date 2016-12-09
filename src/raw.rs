
use libc::{c_int, c_uint, c_void, c_ulong, c_long, c_ulonglong, c_char, c_longlong};
use std::f32;
pub const REDISMODULE_APIVER_1: c_int = 1;
use std::mem;

#[repr(C)]
#[derive(Debug,PartialEq)]
pub enum Status {
    Ok = 0,
    Err = 1,
}

bitflags! {
    pub flags KeyMode: c_int {
        const KEYMODE_READ = (1 << 0),
        const KEYMODE_WRITE = (1 << 1),
    }
}

#[derive(Debug)]
pub enum ListPos {
    Head = 0,
    Tail = 1,
}
#[repr(C)]
#[derive(Debug)]
pub enum KeyType {
    Empty = 0,
    String = 1,
    List = 2,
    Hash = 3,
    Set = 4,
    ZSet = 5,
    Module = 6,
}

#[repr(C)]
#[derive(Debug)]
pub enum ReplyType {
    Unknown = -1,
    String = 0,
    Error = 1,
    Integer = 2,
    Array = 3,
    Null = 4,
}



pub const POSTPONED_ARRAY_LEN: c_int = -1;
pub const NO_EXPIRE: c_int = -1;

bitflags! {
    pub flags ZSetFlags: c_int {
        const ZADD_XX = (1<<0),
        const ZADD_NX = (1<<1),
        const ZADD_ADDED = (1<<2),
        const ZADD_UPDATED = (1<<3),
        const ZADD_NOP = (1<<4),
    }
}

bitflags! {
    pub flags HashFlags: c_uint {
        const HASH_NONE = 0,
        const HASH_NX = (1<<0),
        const HASH_XX = (1<<2),
        const HASH_CFIELDS = (1<<3),
        const HASH_EXISTS = (1<<4),
    }
}

pub const REDISMODULE_POSITIVE_INFINITE: f32 = f32::INFINITY;
pub const REDISMODULE_NEGATIVE_INFINITE: f32 = f32::NEG_INFINITY;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct RedisModuleCtx {
    pub _address: u8,
}
impl Clone for RedisModuleCtx {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct RedisModuleKey {
    pub _address: u8,
}
impl Clone for RedisModuleKey {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct RedisModuleString {
    pub _address: u8,
}
impl Clone for RedisModuleString {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct RedisModuleCallReply {
    pub _address: u8,
}
impl Clone for RedisModuleCallReply {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct RedisModuleIO {
    pub _address: u8,
}
impl Clone for RedisModuleIO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct RedisModuleType {
    pub _address: u8,
}
impl Clone for RedisModuleType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct RedisModuleDigest {
    pub _address: u8,
}
impl Clone for RedisModuleDigest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct RedisModuleBlockedClient {
    pub _address: u8,
}
impl Clone for RedisModuleBlockedClient {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RedisModuleCmdFunc = unsafe extern "C" fn(ctx: *mut RedisModuleCtx,
                                                   argv: *mut *mut RedisModuleString,
                                                   argc: c_int)
                                                   -> Status;

pub type RedisModuleTypeLoadFunc = unsafe extern "C" fn(rdb: *mut RedisModuleIO, encver: c_int)
                                                        -> *mut c_void;
pub type RedisModuleTypeSaveFunc = unsafe extern "C" fn(rdb: *mut RedisModuleIO,
                                                        value: *mut c_void);
pub type RedisModuleTypeRewriteFunc = unsafe extern "C" fn(aof: *mut RedisModuleIO,
                                                           key: *mut RedisModuleString,
                                                           value: *mut c_void);
pub type RedisModuleTypeMemUsageFunc = unsafe extern "C" fn(value: *mut c_void) -> c_ulong;
pub type RedisModuleTypeDigestFunc = unsafe extern "C" fn(digest: *mut RedisModuleDigest,
                                                          value: *mut c_void);
pub type RedisModuleTypeFreeFunc = unsafe extern "C" fn(value: *mut c_void);
#[repr(C)]
#[derive(Debug, Copy)]
pub struct RedisModuleTypeMethods {
    pub version: u64,
    pub rdb_load: RedisModuleTypeLoadFunc,
    pub rdb_save: RedisModuleTypeSaveFunc,
    pub aof_rewrite: RedisModuleTypeRewriteFunc,
    pub mem_usage: RedisModuleTypeMemUsageFunc,
    pub digest: RedisModuleTypeDigestFunc,
    pub free: RedisModuleTypeFreeFunc,
}
#[test]
fn bindgen_test_layout_RedisModuleTypeMethods() {
    assert_eq!(::std::mem::size_of::<RedisModuleTypeMethods>(), 56usize);
    assert_eq!(::std::mem::align_of::<RedisModuleTypeMethods>(), 8usize);
}
impl Clone for RedisModuleTypeMethods {
    fn clone(&self) -> Self {
        *self
    }
}
#[link(name="redismodule")] 
extern "C" {
     pub fn RedisModule_Alloc(bytes: usize) -> *mut c_void;

     pub fn RedisModule_Realloc(ptr: *mut c_void, bytes: usize) -> *mut c_void;

     pub fn RedisModule_Free(ptr: *mut c_void);

     pub fn RedisModule_Calloc(nmemb: usize, size: usize) -> *mut c_void;

     pub fn RedisModule_Strdup(str: *const c_char) -> *mut c_char;

    //  pub fn RedisModule_GetApi(arg1: *const c_char, arg2: *mut c_void) -> Status;

     pub fn RedisModule_CreateCommand(ctx: *mut RedisModuleCtx,
                                     name: *const c_char,
                                     cmdfunc: RedisModuleCmdFunc,
                                     strflags: *const c_char,
                                     firstkey: c_int,
                                     lastkey: c_int,
                                     keystep: c_int)
                                     -> Status;

     pub fn RedisModule_SetModuleAttribs(ctx: *mut RedisModuleCtx,
                                        name: *const c_char,
                                        ver: c_int,
                                        apiver: c_int)
                                        -> c_int;

     pub fn RedisModule_WrongArity(ctx: *mut RedisModuleCtx) -> Status;

     pub fn RedisModule_ReplyWithLongLong(ctx: *mut RedisModuleCtx, ll: c_longlong) -> Status;

     pub fn RedisModule_GetSelectedDb(ctx: *mut RedisModuleCtx) -> c_int;

     pub fn RedisModule_SelectDb(ctx: *mut RedisModuleCtx, newid: c_int) -> c_int;

     pub fn RedisModule_OpenKey(ctx: *mut RedisModuleCtx,
                               keyname: *mut RedisModuleString,
                               mode: c_int)
                               -> *mut c_void;

     pub fn RedisModule_CloseKey(kp: *mut RedisModuleKey);

     pub fn RedisModule_KeyType(kp: *mut RedisModuleKey) -> KeyType;

     pub fn RedisModule_ValueLength(kp: *mut RedisModuleKey) -> c_ulong;

     pub fn RedisModule_ListPush(kp: *mut RedisModuleKey,
                                where_: c_int,
                                ele: *mut RedisModuleString)
                                -> Status;

     pub fn RedisModule_ListPop(key: *mut RedisModuleKey, where_: c_int) -> *mut RedisModuleString;

     pub fn RedisModule_Call(ctx: *mut RedisModuleCtx,
                            cmdname: *const c_char,
                            fmt: *const c_char,
                            ...)
                            -> *mut RedisModuleCallReply;

     pub fn RedisModule_CallReplyProto(reply: *mut RedisModuleCallReply,
                                      len: *mut usize)
                                      -> *const c_char;

     pub fn RedisModule_FreeCallReply(reply: *mut RedisModuleCallReply);

     pub fn RedisModule_CallReplyType(reply: *mut RedisModuleCallReply) -> ReplyType;

     pub fn RedisModule_CallReplyInteger(reply: *mut RedisModuleCallReply) -> c_longlong;

     pub fn RedisModule_CallReplyLength(reply: *mut RedisModuleCallReply) -> c_ulong;

     pub fn RedisModule_CallReplyArrayElement(reply: *mut RedisModuleCallReply,
                                             idx: usize)
                                             -> *mut RedisModuleCallReply;

     pub fn RedisModule_CreateString(ctx: *mut RedisModuleCtx,
                                    ptr: *const c_char,
                                    len: usize)
                                    -> *mut RedisModuleString;

     pub fn RedisModule_CreateStringFromLongLong(ctx: *mut RedisModuleCtx,
                                                ll: c_longlong)
                                                -> *mut RedisModuleString;

     pub fn RedisModule_CreateStringFromString(ctx: *mut RedisModuleCtx,
                                              str: *const RedisModuleString)
                                              -> *mut RedisModuleString;

     pub fn RedisModule_CreateStringPrintf(ctx: *mut RedisModuleCtx,
                                          fmt: *const c_char,
                                          ...)
                                          -> *mut RedisModuleString;

     pub fn RedisModule_FreeString(ctx: *mut RedisModuleCtx, str: *mut RedisModuleString);

     pub fn RedisModule_StringPtrLen(str: *const RedisModuleString,
                                    len: *mut usize)
                                    -> *const c_char;

     pub fn RedisModule_ReplyWithError(ctx: *mut RedisModuleCtx, err: *const c_char) -> Status;

     pub fn RedisModule_ReplyWithSimpleString(ctx: *mut RedisModuleCtx,
                                             msg: *const c_char)
                                             -> Status;

     pub fn RedisModule_ReplyWithArray(ctx: *mut RedisModuleCtx, len: c_long) -> Status;


     pub fn RedisModule_ReplySetArrayLength(ctx: *mut RedisModuleCtx, len: c_long);
     pub fn RedisModule_ReplyWithStringBuffer(ctx: *mut RedisModuleCtx,
                                             buf: *const c_char,
                                             len: usize)
                                             -> Status;
     pub fn RedisModule_ReplyWithString(ctx: *mut RedisModuleCtx,
                                       str: *mut RedisModuleString)
                                       -> Status;
     pub fn RedisModule_ReplyWithNull(ctx: *mut RedisModuleCtx) -> Status;
     pub fn RedisModule_ReplyWithDouble(ctx: *mut RedisModuleCtx) -> Status;
     pub fn RedisModule_ReplyWithCallReply(ctx: *mut RedisModuleCtx,
                                          reply: *mut RedisModuleCallReply)
                                          -> c_int;
     pub fn RedisModule_StringToLongLong(str: *const RedisModuleString,
                                        ll: *mut c_longlong)
                                        -> Status;
     pub fn RedisModule_StringToDouble(str: *const RedisModuleString, d: *mut f64) -> Status;
     pub fn RedisModule_AutoMemory(ctx: *mut RedisModuleCtx);
     pub fn RedisModule_Replicate(ctx: *mut RedisModuleCtx,
                                 cmdname: *const c_char,
                                 fmt: *const c_char,
                                 ...)
                                 -> Status;
     pub fn RedisModule_ReplicateVerbatim(ctx: *mut RedisModuleCtx) -> Status;
     pub fn RedisModule_CallReplyStringPtr(reply: *mut RedisModuleCallReply,
                                          len: *mut usize)
                                          -> *const c_char;
     pub fn RedisModule_CreateStringFromCallReply(reply: *mut RedisModuleCallReply)
                                                 -> *mut RedisModuleString;
     pub fn RedisModule_DeleteKey(key: *mut RedisModuleKey) -> Status;
     pub fn RedisModule_StringSet(key: *mut RedisModuleKey, str: *mut RedisModuleString) -> Status;
     pub fn RedisModule_StringDMA(key: *mut RedisModuleKey,
                                 len: *mut usize,
                                 mode: c_int)
                                 -> *mut c_char;


     pub fn RedisModule_StringTruncate(key: *mut RedisModuleKey, newlen: usize) -> Status;


     pub fn RedisModule_GetExpire(key: *mut RedisModuleKey) -> c_longlong;

     pub fn RedisModule_SetExpire(key: *mut RedisModuleKey, expire: c_longlong) -> Status;


     pub fn RedisModule_ZsetAdd(key: *mut RedisModuleKey,
                               score: f64,
                               ele: *mut RedisModuleString,
                               flagsptr: *mut c_int)
                               -> Status;


     pub fn RedisModule_ZsetIncrby(key: *mut RedisModuleKey,
                                  score: f64,
                                  ele: *mut RedisModuleString,
                                  flagsptr: *mut c_int,
                                  newscore: *mut f64)
                                  -> Status;


     pub fn RedisModule_ZsetScore(key: *mut RedisModuleKey,
                                 ele: *mut RedisModuleString,
                                 score: *mut f64)
                                 -> Status;


     pub fn RedisModule_ZsetRem(key: *mut RedisModuleKey,
                               ele: *mut RedisModuleString,
                               deleted: *mut c_int)
                               -> Status;


     pub fn RedisModule_ZsetRangeStop(key: *mut RedisModuleKey);

     pub fn RedisModule_ZsetFirstInScoreRange(key: *mut RedisModuleKey,
                                             min: f64,
                                             max: f64,
                                             minex: c_int,
                                             maxex: c_int)
                                             -> Status;

     pub fn RedisModule_ZsetLastInScoreRange(key: *mut RedisModuleKey,
                                            min: f64,
                                            max: f64,
                                            minex: c_int,
                                            maxex: c_int)
                                            -> Status;


     pub fn RedisModule_ZsetFirstInLexRange(key: *mut RedisModuleKey,
                                           min: *mut RedisModuleString,
                                           max: *mut RedisModuleString)
                                           -> Status;

     pub fn RedisModule_ZsetLastInLexRange(key: *mut RedisModuleKey,
                                          min: *mut RedisModuleString,
                                          max: *mut RedisModuleString)
                                          -> Status;


     pub fn RedisModule_ZsetRangeCurrentElement(key: *mut RedisModuleKey,
                                               score: *mut f64)
                                               -> *mut RedisModuleString;


     pub fn RedisModule_ZsetRangeNext(key: *mut RedisModuleKey) -> c_int;


     pub fn RedisModule_ZsetRangePrev(key: *mut RedisModuleKey) -> c_int;


     pub fn RedisModule_ZsetRangeEndReached(key: *mut RedisModuleKey) -> c_int;


     pub fn RedisModule_HashSet(key: *mut RedisModuleKey, flags: c_int, ...) -> c_int;


     pub fn RedisModule_HashGet(key: *mut RedisModuleKey, flags: c_int, ...) -> Status;


     pub fn RedisModule_IsKeysPositionRequest(ctx: *mut RedisModuleCtx) -> c_int;


     pub fn RedisModule_KeyAtPos(ctx: *mut RedisModuleCtx, pos: c_int);


     pub fn RedisModule_GetClientId(ctx: *mut RedisModuleCtx) -> c_ulonglong;


     pub fn RedisModule_PoolAlloc(ctx: *mut RedisModuleCtx, bytes: usize) -> *mut c_void;


     pub fn RedisModule_CreateDataType(ctx: *mut RedisModuleCtx,
                                      name: *const c_char,
                                      encver: c_int,
                                      typemethods: *mut RedisModuleTypeMethods)
                                      -> *mut RedisModuleType;


     pub fn RedisModule_ModuleTypeSetValue(key: *mut RedisModuleKey,
                                          mt: *mut RedisModuleType,
                                          value: *mut c_void)
                                          -> Status;


     pub fn RedisModule_ModuleTypeGetType(key: *mut RedisModuleKey) -> *mut RedisModuleType;


     pub fn RedisModule_ModuleTypeGetValue(key: *mut RedisModuleKey) -> *mut c_void;


     pub fn RedisModule_SaveUnsigned(io: *mut RedisModuleIO, value: u64);


     pub fn RedisModule_LoadUnsigned(io: *mut RedisModuleIO) -> c_ulong;


     pub fn RedisModule_SaveSigned(io: *mut RedisModuleIO, value: i64);


     pub fn RedisModule_LoadSigned(io: *mut RedisModuleIO) -> c_long;


     pub fn RedisModule_EmitAOF(io: *mut RedisModuleIO,
                               cmdname: *const c_char,
                               fmt: *const c_char,
                               ...);


     pub fn RedisModule_SaveString(io: *mut RedisModuleIO, s: *mut RedisModuleString);


     pub fn RedisModule_SaveStringBuffer(io: *mut RedisModuleIO, str: *const c_char, len: usize);


     pub fn RedisModule_LoadString(io: *mut RedisModuleIO) -> *mut RedisModuleString;


     pub fn RedisModule_LoadStringBuffer(io: *mut RedisModuleIO, lenptr: *mut usize) -> *mut c_char;


     pub fn RedisModule_SaveDouble(io: *mut RedisModuleIO, value: f64);


     pub fn RedisModule_LoadDouble(io: *mut RedisModuleIO) -> f64;


     pub fn RedisModule_SaveFloat(io: *mut RedisModuleIO, value: f32);


     pub fn RedisModule_LoadFloat(io: *mut RedisModuleIO) -> f32;


     pub fn RedisModule_Log(ctx: *mut RedisModuleCtx,
                           level: *const c_char,
                           fmt: *const c_char,
                           ...);


     pub fn RedisModule_LogIOError(io: *mut RedisModuleIO,
                                  levelstr: *const c_char,
                                  fmt: *const c_char,
                                  ...);


     pub fn RedisModule_StringAppendBuffer(ctx: *mut RedisModuleCtx,
                                          str: *mut RedisModuleString,
                                          buf: *const c_char,
                                          len: usize)
                                          -> c_int;


     pub fn RedisModule_RetainString(ctx: *mut RedisModuleCtx, str: *mut RedisModuleString);


     pub fn RedisModule_StringCompare(a: *mut RedisModuleString,
                                     b: *mut RedisModuleString)
                                     -> c_int;


     pub fn RedisModule_GetContextFromIO(io: *mut RedisModuleIO) -> *mut RedisModuleCtx;
     pub fn RedisModule_BlockClient(ctx: *mut RedisModuleCtx,
                                   reply_callback: RedisModuleCmdFunc,
                                   timeout_callback: RedisModuleCmdFunc,
                                   free_privdata: unsafe extern "C" fn(arg1: *mut c_void),
                                   timeout_ms: c_longlong)
                                   -> *mut RedisModuleBlockedClient;


     pub fn RedisModule_UnblockClient(bc: *mut RedisModuleBlockedClient,
                                     privdata: *mut c_void)
                                     -> c_int;
     pub fn RedisModule_IsBlockedReplyRequest(ctx: *mut RedisModuleCtx) -> c_int;
     pub fn RedisModule_IsBlockedTimeoutRequest(ctx: *mut RedisModuleCtx) -> c_int;
     pub fn RedisModule_GetBlockedClientPrivateData(ctx: *mut RedisModuleCtx) -> *mut c_void;
     pub fn RedisModule_AbortBlock(bc: *mut RedisModuleBlockedClient) -> c_int;
     pub fn RedisModule_Milliseconds() -> c_longlong;
}



pub extern "C" fn RedisModule_Init(ctx: *mut RedisModuleCtx,
                                   name: *const c_char,
                                   ver: c_int,
                                   apiver: c_int)
                                   -> Status {


    unsafe {
        let RedisModule_GetApi: fn(arg1: *const c_char, arg2: *mut c_void) -> Status =
            mem::transmute(*(ctx as *mut *mut c_void));
        macro_rules! getapi {
            ($name:ident) => ( RedisModule_GetApi(format!("{}\0",stringify!($name)).as_ptr() as *const i8, mem::transmute(&$name)););
        }
        getapi!(RedisModule_Alloc);
        getapi!(RedisModule_Alloc);
        getapi!(RedisModule_Calloc);
        getapi!(RedisModule_Free);
        getapi!(RedisModule_Realloc);
        getapi!(RedisModule_Strdup);
        getapi!(RedisModule_CreateCommand);
        getapi!(RedisModule_SetModuleAttribs);
        getapi!(RedisModule_WrongArity);
        getapi!(RedisModule_ReplyWithLongLong);
        getapi!(RedisModule_ReplyWithError);
        getapi!(RedisModule_ReplyWithSimpleString);
        getapi!(RedisModule_ReplyWithArray);
        getapi!(RedisModule_ReplySetArrayLength);
        getapi!(RedisModule_ReplyWithStringBuffer);
        getapi!(RedisModule_ReplyWithString);
        getapi!(RedisModule_ReplyWithNull);
        getapi!(RedisModule_ReplyWithCallReply);
        getapi!(RedisModule_ReplyWithDouble);
        getapi!(RedisModule_ReplySetArrayLength);
        getapi!(RedisModule_GetSelectedDb);
        getapi!(RedisModule_SelectDb);
        getapi!(RedisModule_OpenKey);
        getapi!(RedisModule_CloseKey);
        getapi!(RedisModule_KeyType);
        getapi!(RedisModule_ValueLength);
        getapi!(RedisModule_ListPush);
        getapi!(RedisModule_ListPop);
        getapi!(RedisModule_StringToLongLong);
        getapi!(RedisModule_StringToDouble);
        getapi!(RedisModule_Call);
        getapi!(RedisModule_CallReplyProto);
        getapi!(RedisModule_FreeCallReply);
        getapi!(RedisModule_CallReplyInteger);
        getapi!(RedisModule_CallReplyType);
        getapi!(RedisModule_CallReplyLength);
        getapi!(RedisModule_CallReplyArrayElement);
        getapi!(RedisModule_CallReplyStringPtr);
        getapi!(RedisModule_CreateStringFromCallReply);
        getapi!(RedisModule_CreateString);
        getapi!(RedisModule_CreateStringFromLongLong);
        getapi!(RedisModule_CreateStringFromString);
        getapi!(RedisModule_CreateStringPrintf);
        getapi!(RedisModule_FreeString);
        getapi!(RedisModule_StringPtrLen);
        getapi!(RedisModule_AutoMemory);
        getapi!(RedisModule_Replicate);
        getapi!(RedisModule_ReplicateVerbatim);
        getapi!(RedisModule_DeleteKey);
        getapi!(RedisModule_StringSet);
        getapi!(RedisModule_StringDMA);
        getapi!(RedisModule_StringTruncate);
        getapi!(RedisModule_GetExpire);
        getapi!(RedisModule_SetExpire);
        getapi!(RedisModule_ZsetAdd);
        getapi!(RedisModule_ZsetIncrby);
        getapi!(RedisModule_ZsetScore);
        getapi!(RedisModule_ZsetRem);
        getapi!(RedisModule_ZsetRangeStop);
        getapi!(RedisModule_ZsetFirstInScoreRange);
        getapi!(RedisModule_ZsetLastInScoreRange);
        getapi!(RedisModule_ZsetFirstInLexRange);
        getapi!(RedisModule_ZsetLastInLexRange);
        getapi!(RedisModule_ZsetRangeCurrentElement);
        getapi!(RedisModule_ZsetRangeNext);
        getapi!(RedisModule_ZsetRangePrev);
        getapi!(RedisModule_ZsetRangeEndReached);
        getapi!(RedisModule_HashSet);
        getapi!(RedisModule_HashGet);
        getapi!(RedisModule_IsKeysPositionRequest);
        getapi!(RedisModule_KeyAtPos);
        getapi!(RedisModule_GetClientId);
        getapi!(RedisModule_PoolAlloc);
        getapi!(RedisModule_CreateDataType);
        getapi!(RedisModule_ModuleTypeSetValue);
        getapi!(RedisModule_ModuleTypeGetType);
        getapi!(RedisModule_ModuleTypeGetValue);
        getapi!(RedisModule_SaveUnsigned);
        getapi!(RedisModule_LoadUnsigned);
        getapi!(RedisModule_SaveSigned);
        getapi!(RedisModule_LoadSigned);
        getapi!(RedisModule_SaveString);
        getapi!(RedisModule_SaveStringBuffer);
        getapi!(RedisModule_LoadString);
        getapi!(RedisModule_LoadStringBuffer);
        getapi!(RedisModule_SaveDouble);
        getapi!(RedisModule_LoadDouble);
        getapi!(RedisModule_SaveFloat);
        getapi!(RedisModule_LoadFloat);
        getapi!(RedisModule_EmitAOF);
        getapi!(RedisModule_Log);
        getapi!(RedisModule_LogIOError);
        getapi!(RedisModule_StringAppendBuffer);
        getapi!(RedisModule_RetainString);
        getapi!(RedisModule_StringCompare);
        getapi!(RedisModule_GetContextFromIO);
        RedisModule_SetModuleAttribs(ctx, name, ver, apiver);
        return Status::Ok;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
