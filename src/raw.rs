#![allow(dead_code, non_camel_case_types, non_snake_case)]

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


// Opaque types
pub enum RedisModuleCtx {}
pub enum RedisModuleKey {}
pub enum RedisModuleString {}
pub enum RedisModuleCallReply {}
pub enum RedisModuleIO {}
pub enum RedisModuleType {}
pub enum RedisModuleDigest {}
pub enum RedisModuleBlockedClient {}



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
#[derive(Debug, Copy, Clone)]
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
fn test_layout_RedisModuleTypeMethods() {
    assert_eq!(::std::mem::size_of::<RedisModuleTypeMethods>(), 56usize);
    assert_eq!(::std::mem::align_of::<RedisModuleTypeMethods>(), 8usize);
}

#[link(name = "redismodule", kind = "dylib")]
extern "C" {
    static RedisModule_Alloc: extern "C" fn(bytes: usize) -> *mut c_void;
    static RedisModule_Realloc: extern "C" fn(ptr: *mut c_void, bytes: usize) -> *mut c_void;
    static RedisModule_Free: extern "C" fn(ptr: *mut c_void);
    static RedisModule_Calloc: extern "C" fn(nmemb: usize, size: usize) -> *mut c_void;
    static RedisModule_Strdup: extern "C" fn(str: *const c_char) -> *mut c_char;
    pub static RedisModule_CreateCommand: extern "C" fn(ctx: *mut RedisModuleCtx,
                                                        name: *const c_char,
                                                        cmdfunc: RedisModuleCmdFunc,
                                                        strflags: *const c_char,
                                                        firstkey: c_int,
                                                        lastkey: c_int,
                                                        keystep: c_int)
                                                        -> Status;
    static RedisModule_SetModuleAttribs: extern "C" fn(ctx: *mut RedisModuleCtx,
                                                       name: *const c_char,
                                                       ver: c_int,
                                                       apiver: c_int)
                                                       -> c_int;
    pub static RedisModule_WrongArity: extern "C" fn(ctx: *mut RedisModuleCtx) -> Status;
    pub static RedisModule_ReplyWithLongLong: extern "C" fn(ctx: *mut RedisModuleCtx, ll: c_longlong)
                                                        -> Status;
    static RedisModule_GetSelectedDb: extern "C" fn(ctx: *mut RedisModuleCtx) -> c_int;
    static RedisModule_SelectDb: extern "C" fn(ctx: *mut RedisModuleCtx, newid: c_int) -> c_int;
    static RedisModule_OpenKey: extern "C" fn(ctx: *mut RedisModuleCtx,
                                              keyname: *mut RedisModuleString,
                                              mode: c_int)
                                              -> *mut c_void;
    static RedisModule_CloseKey: extern "C" fn(kp: *mut RedisModuleKey);
    static RedisModule_KeyType: extern "C" fn(kp: *mut RedisModuleKey) -> KeyType;

    static RedisModule_ValueLength: extern "C" fn(kp: *mut RedisModuleKey) -> c_ulong;

    static RedisModule_ListPush: extern "C" fn(kp: *mut RedisModuleKey,
                                               where_: c_int,
                                               ele: *mut RedisModuleString)
                                               -> Status;
    static RedisModule_ListPop: extern "C" fn(key: *mut RedisModuleKey, where_: c_int)
                                              -> *mut RedisModuleString;
    static RedisModule_Call: extern "C" fn(ctx: *mut RedisModuleCtx,
                                           cmdname: *const c_char,
                                           fmt: *const c_char,
                                           ...)
                                           -> *mut RedisModuleCallReply;
    static RedisModule_CallReplyProto: extern "C" fn(reply: *mut RedisModuleCallReply,
                                                     len: *mut usize)
                                                     -> *const c_char;
    static RedisModule_FreeCallReply: extern "C" fn(reply: *mut RedisModuleCallReply);
    static RedisModule_CallReplyType: extern "C" fn(reply: *mut RedisModuleCallReply) -> ReplyType;

    static RedisModule_CallReplyInteger: extern "C" fn(reply: *mut RedisModuleCallReply)
                                                       -> c_longlong;
    static RedisModule_CallReplyLength: extern "C" fn(reply: *mut RedisModuleCallReply) -> c_ulong;
    static RedisModule_CallReplyArrayElement: extern "C" fn(reply: *mut RedisModuleCallReply,
                                                            idx: usize)
                                                            -> *mut RedisModuleCallReply;
    pub static RedisModule_CreateString: extern "C" fn(ctx: *mut RedisModuleCtx,
                                                   ptr: *const c_char,
                                                   len: usize)
                                                   -> *mut RedisModuleString;
    static RedisModule_CreateStringFromLongLong: extern "C" fn(ctx: *mut RedisModuleCtx,
                                                               ll: c_longlong)
                                                               -> *mut RedisModuleString;
    static RedisModule_CreateStringFromString: extern "C" fn(ctx: *mut RedisModuleCtx,
                                                             str: *const RedisModuleString)
                                                             -> *mut RedisModuleString;
    static RedisModule_CreateStringPrintf: extern "C" fn(ctx: *mut RedisModuleCtx,
                                                         fmt: *const c_char,
                                                         ...)
                                                         -> *mut RedisModuleString;
    pub static RedisModule_FreeString: extern "C" fn(ctx: *mut RedisModuleCtx,
                                                 str: *mut RedisModuleString);
    static RedisModule_StringPtrLen: extern "C" fn(str: *const RedisModuleString, len: *mut usize)
                                                   -> *const c_char;
    pub static RedisModule_ReplyWithError: extern "C" fn(ctx: *mut RedisModuleCtx, err: *const c_char)
                                                     -> Status;

    static RedisModule_ReplyWithSimpleString: extern "C" fn(ctx: *mut RedisModuleCtx,
                                                            msg: *const c_char)
                                                            -> Status;

    pub static RedisModule_ReplyWithArray: extern "C" fn(ctx: *mut RedisModuleCtx, len: c_long) -> Status;


    static RedisModule_ReplySetArrayLength: extern "C" fn(ctx: *mut RedisModuleCtx, len: c_long);
    static RedisModule_ReplyWithStringBuffer: extern "C" fn(ctx: *mut RedisModuleCtx,
                                                            buf: *const c_char,
                                                            len: usize)
                                                            -> Status;
    pub static RedisModule_ReplyWithString: extern "C" fn(ctx: *mut RedisModuleCtx,
                                                          str: *mut RedisModuleString)
                                                          -> Status;
    static RedisModule_ReplyWithNull: extern "C" fn(ctx: *mut RedisModuleCtx) -> Status;
    static RedisModule_ReplyWithDouble: extern "C" fn(ctx: *mut RedisModuleCtx) -> Status;
    static RedisModule_ReplyWithCallReply: extern "C" fn(ctx: *mut RedisModuleCtx,
                                                         reply: *mut RedisModuleCallReply)
                                                         -> c_int;
    static RedisModule_StringToLongLong: extern "C" fn(str: *const RedisModuleString,
                                                       ll: *mut c_longlong)
                                                       -> Status;
    static RedisModule_StringToDouble: extern "C" fn(str: *const RedisModuleString, d: *mut f64)
                                                     -> Status;
    static RedisModule_AutoMemory: extern "C" fn(ctx: *mut RedisModuleCtx);
    static RedisModule_Replicate: extern "C" fn(ctx: *mut RedisModuleCtx,
                                                cmdname: *const c_char,
                                                fmt: *const c_char,
                                                ...)
                                                -> Status;
    static RedisModule_ReplicateVerbatim: extern "C" fn(ctx: *mut RedisModuleCtx) -> Status;
    static RedisModule_CallReplyStringPtr: extern "C" fn(reply: *mut RedisModuleCallReply,
                                                         len: *mut usize)
                                                         -> *const c_char;
    static RedisModule_CreateStringFromCallReply: extern "C" fn(reply: *mut RedisModuleCallReply)
                                                                -> *mut RedisModuleString;
    static RedisModule_DeleteKey: extern "C" fn(key: *mut RedisModuleKey) -> Status;
    static RedisModule_StringSet: extern "C" fn(key: *mut RedisModuleKey,
                                                str: *mut RedisModuleString)
                                                -> Status;
    static RedisModule_StringDMA: extern "C" fn(key: *mut RedisModuleKey,
                                                len: *mut usize,
                                                mode: c_int)
                                                -> *mut c_char;


    static RedisModule_StringTruncate: extern "C" fn(key: *mut RedisModuleKey, newlen: usize)
                                                     -> Status;


    static RedisModule_GetExpire: extern "C" fn(key: *mut RedisModuleKey) -> c_longlong;

    static RedisModule_SetExpire: extern "C" fn(key: *mut RedisModuleKey, expire: c_longlong)
                                                -> Status;


    static RedisModule_ZsetAdd: extern "C" fn(key: *mut RedisModuleKey,
                                              score: f64,
                                              ele: *mut RedisModuleString,
                                              flagsptr: *mut c_int)
                                              -> Status;


    static RedisModule_ZsetIncrby: extern "C" fn(key: *mut RedisModuleKey,
                                                 score: f64,
                                                 ele: *mut RedisModuleString,
                                                 flagsptr: *mut c_int,
                                                 newscore: *mut f64)
                                                 -> Status;


    static RedisModule_ZsetScore: extern "C" fn(key: *mut RedisModuleKey,
                                                ele: *mut RedisModuleString,
                                                score: *mut f64)
                                                -> Status;


    static RedisModule_ZsetRem: extern "C" fn(key: *mut RedisModuleKey,
                                              ele: *mut RedisModuleString,
                                              deleted: *mut c_int)
                                              -> Status;


    static RedisModule_ZsetRangeStop: extern "C" fn(key: *mut RedisModuleKey);

    static RedisModule_ZsetFirstInScoreRange: extern "C" fn(key: *mut RedisModuleKey,
                                                            min: f64,
                                                            max: f64,
                                                            minex: c_int,
                                                            maxex: c_int)
                                                            -> Status;

    static RedisModule_ZsetLastInScoreRange: extern "C" fn(key: *mut RedisModuleKey,
                                                           min: f64,
                                                           max: f64,
                                                           minex: c_int,
                                                           maxex: c_int)
                                                           -> Status;


    static RedisModule_ZsetFirstInLexRange: extern "C" fn(key: *mut RedisModuleKey,
                                                          min: *mut RedisModuleString,
                                                          max: *mut RedisModuleString)
                                                          -> Status;

    static RedisModule_ZsetLastInLexRange: extern "C" fn(key: *mut RedisModuleKey,
                                                         min: *mut RedisModuleString,
                                                         max: *mut RedisModuleString)
                                                         -> Status;


    static RedisModule_ZsetRangeCurrentElement: extern "C" fn(key: *mut RedisModuleKey,
                                                              score: *mut f64)
                                                              -> *mut RedisModuleString;


    static RedisModule_ZsetRangeNext: extern "C" fn(key: *mut RedisModuleKey) -> c_int;


    static RedisModule_ZsetRangePrev: extern "C" fn(key: *mut RedisModuleKey) -> c_int;


    static RedisModule_ZsetRangeEndReached: extern "C" fn(key: *mut RedisModuleKey) -> c_int;


    static RedisModule_HashSet: extern "C" fn(key: *mut RedisModuleKey, flags: c_int, ...) -> c_int;


    static RedisModule_HashGet: extern "C" fn(key: *mut RedisModuleKey, flags: c_int, ...) -> Status;


    static RedisModule_IsKeysPositionRequest: extern "C" fn(ctx: *mut RedisModuleCtx) -> c_int;


    static RedisModule_KeyAtPos: extern "C" fn(ctx: *mut RedisModuleCtx, pos: c_int);


    static RedisModule_GetClientId: extern "C" fn(ctx: *mut RedisModuleCtx) -> c_ulonglong;


    static RedisModule_PoolAlloc: extern "C" fn(ctx: *mut RedisModuleCtx, bytes: usize)
                                                -> *mut c_void;


    static RedisModule_CreateDataType: extern "C" fn(ctx: *mut RedisModuleCtx,
                                                     name: *const c_char,
                                                     encver: c_int,
                                                     typemethods: *mut RedisModuleTypeMethods)
                                                     -> *mut RedisModuleType;


    static RedisModule_ModuleTypeSetValue: extern "C" fn(key: *mut RedisModuleKey,
                                                         mt: *mut RedisModuleType,
                                                         value: *mut c_void)
                                                         -> Status;


    static RedisModule_ModuleTypeGetType: extern "C" fn(key: *mut RedisModuleKey)
                                                        -> *mut RedisModuleType;


    static RedisModule_ModuleTypeGetValue: extern "C" fn(key: *mut RedisModuleKey) -> *mut c_void;


    static RedisModule_SaveUnsigned: extern "C" fn(io: *mut RedisModuleIO, value: u64);


    static RedisModule_LoadUnsigned: extern "C" fn(io: *mut RedisModuleIO) -> c_ulong;


    static RedisModule_SaveSigned: extern "C" fn(io: *mut RedisModuleIO, value: i64);


    static RedisModule_LoadSigned: extern "C" fn(io: *mut RedisModuleIO) -> c_long;


    static RedisModule_EmitAOF: extern "C" fn(io: *mut RedisModuleIO,
                                              cmdname: *const c_char,
                                              fmt: *const c_char,
                                              ...);


    static RedisModule_SaveString: extern "C" fn(io: *mut RedisModuleIO, s: *mut RedisModuleString);


    static RedisModule_SaveStringBuffer: extern "C" fn(io: *mut RedisModuleIO,
                                                       str: *const c_char,
                                                       len: usize);


    static RedisModule_LoadString: extern "C" fn(io: *mut RedisModuleIO) -> *mut RedisModuleString;


    static RedisModule_LoadStringBuffer: extern "C" fn(io: *mut RedisModuleIO, lenptr: *mut usize)
                                                       -> *mut c_char;


    static RedisModule_SaveDouble: extern "C" fn(io: *mut RedisModuleIO, value: f64);


    static RedisModule_LoadDouble: extern "C" fn(io: *mut RedisModuleIO) -> f64;


    static RedisModule_SaveFloat: extern "C" fn(io: *mut RedisModuleIO, value: f32);


    static RedisModule_LoadFloat: extern "C" fn(io: *mut RedisModuleIO) -> f32;


    static RedisModule_Log: extern "C" fn(ctx: *mut RedisModuleCtx,
                                          level: *const c_char,
                                          fmt: *const c_char,
                                          ...);


    static RedisModule_LogIOError: extern "C" fn(io: *mut RedisModuleIO,
                                                 levelstr: *const c_char,
                                                 fmt: *const c_char,
                                                 ...);


    static RedisModule_StringAppendBuffer: extern "C" fn(ctx: *mut RedisModuleCtx,
                                                         str: *mut RedisModuleString,
                                                         buf: *const c_char,
                                                         len: usize)
                                                         -> c_int;


    static RedisModule_RetainString: extern "C" fn(ctx: *mut RedisModuleCtx,
                                                   str: *mut RedisModuleString);


    static RedisModule_StringCompare: extern "C" fn(a: *mut RedisModuleString,
                                                    b: *mut RedisModuleString)
                                                    -> c_int;


    static RedisModule_GetContextFromIO: extern "C" fn(io: *mut RedisModuleIO) -> *mut RedisModuleCtx;
    static RedisModule_BlockClient: extern "C" fn(ctx: *mut RedisModuleCtx,
                                   reply_callback: RedisModuleCmdFunc,
                                   timeout_callback: RedisModuleCmdFunc,
                                   free_privdata: unsafe extern "C" fn(arg1: *mut c_void),
                                   timeout_ms: c_longlong)
                                   -> *mut RedisModuleBlockedClient;


    static RedisModule_UnblockClient: extern "C" fn(bc: *mut RedisModuleBlockedClient,
                                                    privdata: *mut c_void)
                                                    -> c_int;
    static RedisModule_IsBlockedReplyRequest: extern "C" fn(ctx: *mut RedisModuleCtx) -> c_int;
    static RedisModule_IsBlockedTimeoutRequest: extern "C" fn(ctx: *mut RedisModuleCtx) -> c_int;
    static RedisModule_GetBlockedClientPrivateData: extern "C" fn(ctx: *mut RedisModuleCtx)
                                                                  -> *mut c_void;
    static RedisModule_AbortBlock: extern "C" fn(bc: *mut RedisModuleBlockedClient) -> c_int;
    static RedisModule_Milliseconds: extern "C" fn() -> c_longlong;
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
