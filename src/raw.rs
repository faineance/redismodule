
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
#[link(name = "redismodule")]
extern "C" {
    pub static mut RedisModule_Alloc: unsafe extern "C" fn(bytes: usize) -> *mut c_void;
}
extern "C" {
    #[link_name = "RedisModule_Realloc"]
    pub static mut RedisModule_Realloc: unsafe extern "C" fn(ptr: *mut c_void, bytes: usize)
                                                             -> *mut c_void;
}
extern "C" {
    #[link_name = "RedisModule_Free"]
    pub static mut RedisModule_Free: unsafe extern "C" fn(ptr: *mut c_void);
}
extern "C" {
    #[link_name = "RedisModule_Calloc"]
    pub static mut RedisModule_Calloc: unsafe extern "C" fn(nmemb: usize, size: usize) -> *mut c_void;
}
extern "C" {
    #[link_name = "RedisModule_Strdup"]
    pub static mut RedisModule_Strdup: unsafe extern "C" fn(str: *const c_char) -> *mut c_char;
}
// extern "C" {
//     #[link_name = "RedisModule_GetApi"]
//     pub static mut RedisModule_GetApi: unsafe extern "C" fn(arg1: *const c_char, arg2: *mut c_void)
//    -> Status;
// }
extern "C" {
    #[link_name = "RedisModule_CreateCommand"]
    pub static mut RedisModule_CreateCommand: unsafe extern "C" fn(ctx: *mut RedisModuleCtx,
                                                                   name: *const c_char,
                                                                   cmdfunc: RedisModuleCmdFunc,
                                                                   strflags: *const c_char,
                                                                   firstkey: c_int,
                                                                   lastkey: c_int,
                                                                   keystep: c_int)
                                                                   -> Status;
}
extern "C" {
    #[link_name = "RedisModule_SetModuleAttribs"]
    pub static mut RedisModule_SetModuleAttribs: unsafe extern "C" fn(ctx: *mut RedisModuleCtx,
                                                                      name: *const c_char,
                                                                      ver: c_int,
                                                                      apiver: c_int)
                                                                      -> c_int;
}
extern "C" {
    #[link_name = "RedisModule_WrongArity"]
    pub static mut RedisModule_WrongArity: unsafe extern "C" fn(ctx: *mut RedisModuleCtx) -> Status;
}
extern "C" {
    #[link_name = "RedisModule_ReplyWithLongLong"]
    pub static mut RedisModule_ReplyWithLongLong: unsafe extern "C" fn(ctx: *mut RedisModuleCtx,
                                                                       ll: c_longlong)
                                                                       -> Status;
}
extern "C" {
    #[link_name = "RedisModule_GetSelectedDb"]
    pub static mut RedisModule_GetSelectedDb: unsafe extern "C" fn(ctx: *mut RedisModuleCtx) -> c_int;
}
extern "C" {
    #[link_name = "RedisModule_SelectDb"]
    pub static mut RedisModule_SelectDb: unsafe extern "C" fn(ctx: *mut RedisModuleCtx,
                                                              newid: c_int)
                                                              -> c_int;
}
extern "C" {
    #[link_name = "RedisModule_OpenKey"]
    pub static mut RedisModule_OpenKey: unsafe extern "C" fn(ctx: *mut RedisModuleCtx,
                                                             keyname: *mut RedisModuleString,
                                                             mode: c_int)
                                                             -> *mut c_void;
}
extern "C" {
    #[link_name = "RedisModule_CloseKey"]
    pub static mut RedisModule_CloseKey: unsafe extern "C" fn(kp: *mut RedisModuleKey);
}
extern "C" {
    #[link_name = "RedisModule_KeyType"]
    pub static mut RedisModule_KeyType: unsafe extern "C" fn(kp: *mut RedisModuleKey) -> KeyType;
}
extern "C" {
    #[link_name = "RedisModule_ValueLength"]
    pub static mut RedisModule_ValueLength: unsafe extern "C" fn(kp: *mut RedisModuleKey) -> c_ulong;
}
extern "C" {
    #[link_name = "RedisModule_ListPush"]
    pub static mut RedisModule_ListPush: unsafe extern "C" fn(kp: *mut RedisModuleKey,
                                                              where_: c_int,
                                                              ele: *mut RedisModuleString)
                                                              -> Status;
}
extern "C" {
    #[link_name = "RedisModule_ListPop"]
    pub static mut RedisModule_ListPop: unsafe extern "C" fn(key: *mut RedisModuleKey,
                                                             where_: c_int)
                                                             -> *mut RedisModuleString;
}
extern "C" {
    #[link_name = "RedisModule_Call"]
    pub static mut RedisModule_Call: unsafe extern "C" fn(ctx: *mut RedisModuleCtx,
                                                          cmdname: *const c_char,
                                                          fmt: *const c_char,
                                                          ...)
                                                          -> *mut RedisModuleCallReply;
}
extern "C" {
    #[link_name = "RedisModule_CallReplyProto"]
    pub static mut RedisModule_CallReplyProto:
               unsafe extern "C" fn(reply: *mut RedisModuleCallReply, len: *mut usize)
                                         -> *const c_char;
}
extern "C" {
    #[link_name = "RedisModule_FreeCallReply"]
    pub static mut RedisModule_FreeCallReply:
               unsafe extern "C" fn(reply: *mut RedisModuleCallReply);
}
extern "C" {
    #[link_name = "RedisModule_CallReplyType"]
    pub static mut RedisModule_CallReplyType:
               unsafe extern "C" fn(reply: *mut RedisModuleCallReply)
                                         -> ReplyType;
}
extern "C" {
    #[link_name = "RedisModule_CallReplyInteger"]
    pub static mut RedisModule_CallReplyInteger:
               unsafe extern "C" fn(reply:*mut RedisModuleCallReply) -> c_longlong;
}
extern "C" {
    #[link_name = "RedisModule_CallReplyLength"]
    pub static mut RedisModule_CallReplyLength:
               unsafe extern "C" fn(reply: *mut RedisModuleCallReply)
                                         -> c_ulong;
}
extern "C" {
    #[link_name = "RedisModule_CallReplyArrayElement"]
    pub static mut RedisModule_CallReplyArrayElement:
               unsafe extern "C" fn(reply: *mut RedisModuleCallReply, idx: usize)
                                         -> *mut RedisModuleCallReply;
}
extern "C" {
    #[link_name = "RedisModule_CreateString"]
    pub static mut RedisModule_CreateString: unsafe extern "C" fn(ctx: *mut RedisModuleCtx,
                                                                  ptr: *const c_char,
                                                                  len: usize)
                                                                  -> *mut RedisModuleString;
}
extern "C" {
    #[link_name = "RedisModule_CreateStringFromLongLong"]
    pub static mut RedisModule_CreateStringFromLongLong:
               unsafe extern "C" fn(ctx: *mut RedisModuleCtx, ll: c_longlong)
                                         -> *mut RedisModuleString;
}
extern "C" {
    #[link_name = "RedisModule_CreateStringFromString"]
    pub static mut RedisModule_CreateStringFromString:
               unsafe extern "C" fn(ctx: *mut RedisModuleCtx, str: *const RedisModuleString)
                                         -> *mut RedisModuleString;
}
extern "C" {
    #[link_name = "RedisModule_CreateStringPrintf"]
    pub static mut RedisModule_CreateStringPrintf: unsafe extern "C" fn(ctx: *mut RedisModuleCtx,
                                                                        fmt: *const c_char,
                                                                        ...)
                                                                        -> *mut RedisModuleString;
}
extern "C" {
    #[link_name = "RedisModule_FreeString"]
    pub static mut RedisModule_FreeString: unsafe extern "C" fn(ctx: *mut RedisModuleCtx,
                                                                str: *mut RedisModuleString);
}
extern "C" {
    #[link_name = "RedisModule_StringPtrLen"]
    pub static mut RedisModule_StringPtrLen: unsafe extern "C" fn(str: *const RedisModuleString,
                                                                  len: *mut usize)
                                                                  -> *const c_char;
}
extern "C" {
    #[link_name = "RedisModule_ReplyWithError"]
    pub static mut RedisModule_ReplyWithError: unsafe extern "C" fn(ctx: *mut RedisModuleCtx,
                                                                    err: *const c_char)
                                                                    -> Status;
}
extern "C" {
    #[link_name = "RedisModule_ReplyWithSimpleString"]
    pub static mut RedisModule_ReplyWithSimpleString:
               unsafe extern "C" fn(ctx: *mut RedisModuleCtx, msg: *const c_char)
                                         -> Status;
}
extern "C" {
    #[link_name = "RedisModule_ReplyWithArray"]
    pub static mut RedisModule_ReplyWithArray: unsafe extern "C" fn(ctx: *mut RedisModuleCtx,
                                                                    len: c_long)
                                                                    -> Status;
}
extern "C" {
    #[link_name = "RedisModule_ReplySetArrayLength"]
    pub static mut RedisModule_ReplySetArrayLength: unsafe extern "C" fn(ctx: *mut RedisModuleCtx,
                                                                         len: c_long);
}
extern "C" {
    #[link_name = "RedisModule_ReplyWithStringBuffer"]
    pub static mut RedisModule_ReplyWithStringBuffer:
               unsafe extern "C" fn(ctx: *mut RedisModuleCtx, buf: *const c_char, len: usize)
                                         -> Status;
}
extern "C" {
    #[link_name = "RedisModule_ReplyWithString"]
    pub static mut RedisModule_ReplyWithString: unsafe extern "C" fn(ctx: *mut RedisModuleCtx,
                                                                     str: *mut RedisModuleString)
                                                                     -> Status;
}
extern "C" {
    #[link_name = "RedisModule_ReplyWithNull"]
    pub static mut RedisModule_ReplyWithNull: unsafe extern "C" fn(ctx: *mut RedisModuleCtx)
                                                                   -> Status;
}
extern "C" {
    #[link_name = "RedisModule_ReplyWithDouble"]
    pub static mut RedisModule_ReplyWithDouble: unsafe extern "C" fn(ctx: *mut RedisModuleCtx,
                                                                     d: f64)
                                                                     -> Status;
}
extern "C" {
    #[link_name = "RedisModule_ReplyWithCallReply"]
    pub static mut RedisModule_ReplyWithCallReply:
               unsafe extern "C" fn(ctx: *mut RedisModuleCtx, reply: *mut RedisModuleCallReply)
                                         -> c_int;
}
extern "C" {
    #[link_name = "RedisModule_StringToLongLong"]
    pub static mut RedisModule_StringToLongLong:
               unsafe extern "C" fn(str: *const RedisModuleString, ll: *mut c_longlong)
                                         -> Status;
}
extern "C" {
    #[link_name = "RedisModule_StringToDouble"]
    pub static mut RedisModule_StringToDouble: unsafe extern "C" fn(str: *const RedisModuleString,
                                                                    d: *mut f64)
                                                                    -> Status;
}
extern "C" {
    #[link_name = "RedisModule_AutoMemory"]
    pub static mut RedisModule_AutoMemory: unsafe extern "C" fn(ctx: *mut RedisModuleCtx);
}
extern "C" {
    #[link_name = "RedisModule_Replicate"]
    pub static mut RedisModule_Replicate: unsafe extern "C" fn(ctx: *mut RedisModuleCtx,
                                                               cmdname: *const c_char,
                                                               fmt: *const c_char,
                                                               ...)
                                                               -> Status;
}
extern "C" {
    #[link_name = "RedisModule_ReplicateVerbatim"]
    pub static mut RedisModule_ReplicateVerbatim: unsafe extern "C" fn(ctx: *mut RedisModuleCtx)
                                                                       -> Status;
}
extern "C" {
    #[link_name = "RedisModule_CallReplyStringPtr"]
    pub static mut RedisModule_CallReplyStringPtr:
               unsafe extern "C" fn(reply: *mut RedisModuleCallReply, len: *mut usize)
                                         -> *const c_char;
}
extern "C" {
    #[link_name = "RedisModule_CreateStringFromCallReply"]
    pub static mut RedisModule_CreateStringFromCallReply:
               unsafe extern "C" fn(reply: *mut RedisModuleCallReply)
                                         -> *mut RedisModuleString;
}
extern "C" {
    #[link_name = "RedisModule_DeleteKey"]
    pub static mut RedisModule_DeleteKey: unsafe extern "C" fn(key: *mut RedisModuleKey) -> Status;
}
extern "C" {
    #[link_name = "RedisModule_StringSet"]
    pub static mut RedisModule_StringSet: unsafe extern "C" fn(key: *mut RedisModuleKey,
                                                               str: *mut RedisModuleString)
                                                               -> Status;
}
extern "C" {
    #[link_name = "RedisModule_StringDMA"]
    pub static mut RedisModule_StringDMA: unsafe extern "C" fn(key: *mut RedisModuleKey,
                                                               len: *mut usize,
                                                               mode: c_int)
                                                               -> *mut c_char;
}
extern "C" {
    #[link_name = "RedisModule_StringTruncate"]
    pub static mut RedisModule_StringTruncate: unsafe extern "C" fn(key: *mut RedisModuleKey,
                                                                    newlen: usize)
                                                                    -> Status;
}
extern "C" {
    #[link_name = "RedisModule_GetExpire"]
    pub static mut RedisModule_GetExpire: unsafe extern "C" fn(key: *mut RedisModuleKey)
                                                               -> c_longlong;
}
extern "C" {
    #[link_name = "RedisModule_SetExpire"]
    pub static mut RedisModule_SetExpire: unsafe extern "C" fn(key: *mut RedisModuleKey,
                                                               expire: c_longlong)
                                                               -> Status;
}
extern "C" {
    #[link_name = "RedisModule_ZsetAdd"]
    pub static mut RedisModule_ZsetAdd: unsafe extern "C" fn(key: *mut RedisModuleKey,
                                                             score: f64,
                                                             ele: *mut RedisModuleString,
                                                             flagsptr: *mut c_int)
                                                             -> Status;
}
extern "C" {
    #[link_name = "RedisModule_ZsetIncrby"]
    pub static mut RedisModule_ZsetIncrby: unsafe extern "C" fn(key: *mut RedisModuleKey,
                                                                score: f64,
                                                                ele: *mut RedisModuleString,
                                                                flagsptr: *mut c_int,
                                                                newscore: *mut f64)
                                                                -> Status;
}
extern "C" {
    #[link_name = "RedisModule_ZsetScore"]
    pub static mut RedisModule_ZsetScore: unsafe extern "C" fn(key: *mut RedisModuleKey,
                                                               ele: *mut RedisModuleString,
                                                               score: *mut f64)
                                                               -> Status;
}
extern "C" {
    #[link_name = "RedisModule_ZsetRem"]
    pub static mut RedisModule_ZsetRem: unsafe extern "C" fn(key: *mut RedisModuleKey,
                                                             ele: *mut RedisModuleString,
                                                             deleted: *mut c_int)
                                                             -> Status;
}
extern "C" {
    #[link_name = "RedisModule_ZsetRangeStop"]
    pub static mut RedisModule_ZsetRangeStop: unsafe extern "C" fn(key: *mut RedisModuleKey);
}
extern "C" {
    #[link_name = "RedisModule_ZsetFirstInScoreRange"]
    pub static mut RedisModule_ZsetFirstInScoreRange:
               unsafe extern "C" fn(key: *mut RedisModuleKey, min: f64, max: f64, minex: c_int, maxex: c_int)
                                         -> Status;
}
extern "C" {
    #[link_name = "RedisModule_ZsetLastInScoreRange"]
    pub static mut RedisModule_ZsetLastInScoreRange: unsafe extern "C" fn(key: *mut RedisModuleKey,
                                                                          min: f64,
                                                                          max: f64,
                                                                          minex: c_int,
                                                                          maxex: c_int)
                                                                          -> Status;
}
extern "C" {
    #[link_name = "RedisModule_ZsetFirstInLexRange"]
    pub static mut RedisModule_ZsetFirstInLexRange:
               unsafe extern "C" fn(key: *mut RedisModuleKey, min: *mut RedisModuleString, max: *mut RedisModuleString)
                                         -> Status;
}
extern "C" {
    #[link_name = "RedisModule_ZsetLastInLexRange"]
    pub static mut RedisModule_ZsetLastInLexRange:
               unsafe extern "C" fn(key: *mut RedisModuleKey, min: *mut RedisModuleString, max: *mut RedisModuleString)
                                         -> Status;
}
extern "C" {
    #[link_name = "RedisModule_ZsetRangeCurrentElement"]
    pub static mut RedisModule_ZsetRangeCurrentElement:
               unsafe extern "C" fn(key: *mut RedisModuleKey, score: *mut f64)
                                         -> *mut RedisModuleString;
}
extern "C" {
    #[link_name = "RedisModule_ZsetRangeNext"]
    pub static mut RedisModule_ZsetRangeNext: unsafe extern "C" fn(key: *mut RedisModuleKey) -> c_int;
}
extern "C" {
    #[link_name = "RedisModule_ZsetRangePrev"]
    pub static mut RedisModule_ZsetRangePrev: unsafe extern "C" fn(key: *mut RedisModuleKey) -> c_int;
}
extern "C" {
    #[link_name = "RedisModule_ZsetRangeEndReached"]
    pub static mut RedisModule_ZsetRangeEndReached: unsafe extern "C" fn(key: *mut RedisModuleKey)
                                                                         -> c_int;
}
extern "C" {
    #[link_name = "RedisModule_HashSet"]
    pub static mut RedisModule_HashSet: unsafe extern "C" fn(key: *mut RedisModuleKey,
                                                             flags: c_int,
                                                             ...)
                                                             -> c_int;
}
extern "C" {
    #[link_name = "RedisModule_HashGet"]
    pub static mut RedisModule_HashGet: unsafe extern "C" fn(key: *mut RedisModuleKey,
                                                             flags: c_int,
                                                             ...)
                                                             -> Status;
}
extern "C" {
    #[link_name = "RedisModule_IsKeysPositionRequest"]
    pub static mut RedisModule_IsKeysPositionRequest:
               unsafe extern "C" fn(ctx: *mut RedisModuleCtx)
                                         -> c_int;
}
extern "C" {
    #[link_name = "RedisModule_KeyAtPos"]
    pub static mut RedisModule_KeyAtPos: unsafe extern "C" fn(ctx: *mut RedisModuleCtx, pos: c_int);
}
extern "C" {
    #[link_name = "RedisModule_GetClientId"]
    pub static mut RedisModule_GetClientId: unsafe extern "C" fn(ctx: *mut RedisModuleCtx)
                                                                 -> c_ulonglong;
}
extern "C" {
    #[link_name = "RedisModule_PoolAlloc"]
    pub static mut RedisModule_PoolAlloc: unsafe extern "C" fn(ctx: *mut RedisModuleCtx,
                                                               bytes: usize)
                                                               -> *mut c_void;
}
extern "C" {
    #[link_name = "RedisModule_CreateDataType"]
    pub static mut RedisModule_CreateDataType:
               unsafe extern "C" fn(ctx: *mut RedisModuleCtx, name: *const c_char, encver: c_int, typemethods: *mut RedisModuleTypeMethods)
                                         -> *mut RedisModuleType;
}
extern "C" {
    #[link_name = "RedisModule_ModuleTypeSetValue"]
    pub static mut RedisModule_ModuleTypeSetValue: unsafe extern "C" fn(key: *mut RedisModuleKey,
                                                                        mt: *mut RedisModuleType,
                                                                        value: *mut c_void)
                                                                        -> Status;
}
extern "C" {
    #[link_name = "RedisModule_ModuleTypeGetType"]
    pub static mut RedisModule_ModuleTypeGetType: unsafe extern "C" fn(key: *mut RedisModuleKey)
                                                                       -> *mut RedisModuleType;
}
extern "C" {
    #[link_name = "RedisModule_ModuleTypeGetValue"]
    pub static mut RedisModule_ModuleTypeGetValue: unsafe extern "C" fn(key: *mut RedisModuleKey)
                                                                        -> *mut c_void;
}
extern "C" {
    #[link_name = "RedisModule_SaveUnsigned"]
    pub static mut RedisModule_SaveUnsigned: unsafe extern "C" fn(io: *mut RedisModuleIO,
                                                                  value: u64);
}
extern "C" {
    #[link_name = "RedisModule_LoadUnsigned"]
    pub static mut RedisModule_LoadUnsigned: unsafe extern "C" fn(io: *mut RedisModuleIO) -> c_ulong;
}
extern "C" {
    #[link_name = "RedisModule_SaveSigned"]
    pub static mut RedisModule_SaveSigned: unsafe extern "C" fn(io: *mut RedisModuleIO, value: i64);
}
extern "C" {
    #[link_name = "RedisModule_LoadSigned"]
    pub static mut RedisModule_LoadSigned: unsafe extern "C" fn(io: *mut RedisModuleIO) -> c_long;
}
extern "C" {
    #[link_name = "RedisModule_EmitAOF"]
    pub static mut RedisModule_EmitAOF: unsafe extern "C" fn(io: *mut RedisModuleIO,
                                                             cmdname: *const c_char,
                                                             fmt: *const c_char,
                                                             ...);
}
extern "C" {
    #[link_name = "RedisModule_SaveString"]
    pub static mut RedisModule_SaveString: unsafe extern "C" fn(io: *mut RedisModuleIO,
                                                                s: *mut RedisModuleString);
}
extern "C" {
    #[link_name = "RedisModule_SaveStringBuffer"]
    pub static mut RedisModule_SaveStringBuffer: unsafe extern "C" fn(io: *mut RedisModuleIO,
                                                                      str: *const c_char,
                                                                      len: usize);
}
extern "C" {
    #[link_name = "RedisModule_LoadString"]
    pub static mut RedisModule_LoadString: unsafe extern "C" fn(io: *mut RedisModuleIO)
                                                                -> *mut RedisModuleString;
}
extern "C" {
    #[link_name = "RedisModule_LoadStringBuffer"]
    pub static mut RedisModule_LoadStringBuffer: unsafe extern "C" fn(io: *mut RedisModuleIO,
                                                                      lenptr: *mut usize)
                                                                      -> *mut c_char;
}
extern "C" {
    #[link_name = "RedisModule_SaveDouble"]
    pub static mut RedisModule_SaveDouble: unsafe extern "C" fn(io: *mut RedisModuleIO, value: f64);
}
extern "C" {
    #[link_name = "RedisModule_LoadDouble"]
    pub static mut RedisModule_LoadDouble: unsafe extern "C" fn(io: *mut RedisModuleIO) -> f64;
}
extern "C" {
    #[link_name = "RedisModule_SaveFloat"]
    pub static mut RedisModule_SaveFloat: unsafe extern "C" fn(io: *mut RedisModuleIO, value: f32);
}
extern "C" {
    #[link_name = "RedisModule_LoadFloat"]
    pub static mut RedisModule_LoadFloat: unsafe extern "C" fn(io: *mut RedisModuleIO) -> f32;
}
extern "C" {
    #[link_name = "RedisModule_Log"]
    pub static mut RedisModule_Log: unsafe extern "C" fn(ctx: *mut RedisModuleCtx,
                                                         level: *const c_char,
                                                         fmt: *const c_char,
                                                         ...);
}
extern "C" {
    #[link_name = "RedisModule_LogIOError"]
    pub static mut RedisModule_LogIOError: unsafe extern "C" fn(io: *mut RedisModuleIO,
                                                                levelstr: *const c_char,
                                                                fmt: *const c_char,
                                                                ...);
}
extern "C" {
    #[link_name = "RedisModule_StringAppendBuffer"]
    pub static mut RedisModule_StringAppendBuffer:
               unsafe extern "C" fn(ctx: *mut RedisModuleCtx, str: *mut RedisModuleString, buf: *const c_char, len: usize)
                                         -> c_int;
}
extern "C" {
    #[link_name = "RedisModule_RetainString"]
    pub static mut RedisModule_RetainString: unsafe extern "C" fn(ctx: *mut RedisModuleCtx,
                                                                  str: *mut RedisModuleString);
}
extern "C" {
    #[link_name = "RedisModule_StringCompare"]
    pub static mut RedisModule_StringCompare: unsafe extern "C" fn(a: *mut RedisModuleString,
                                                                   b: *mut RedisModuleString)
                                                                   -> c_int;
}
extern "C" {
    #[link_name = "RedisModule_GetContextFromIO"]
    pub static mut RedisModule_GetContextFromIO: unsafe extern "C" fn(io: *mut RedisModuleIO)
                                                                      -> *mut RedisModuleCtx;
}
extern "C" {
    #[link_name = "RedisModule_BlockClient"]
    pub static mut RedisModule_BlockClient:
               unsafe extern "C" fn(ctx: *mut RedisModuleCtx, reply_callback: RedisModuleCmdFunc, timeout_callback: RedisModuleCmdFunc, free_privdata: unsafe extern "C" fn(arg1:                                                *mut c_void), timeout_ms: c_longlong)
                                         -> *mut RedisModuleBlockedClient;
}
extern "C" {
    #[link_name = "RedisModule_UnblockClient"]
    pub static mut RedisModule_UnblockClient:
               unsafe extern "C" fn(bc: *mut RedisModuleBlockedClient, privdata: *mut c_void)
                                         -> c_int;
}
extern "C" {
    #[link_name = "RedisModule_IsBlockedReplyRequest"]
    pub static mut RedisModule_IsBlockedReplyRequest:
               unsafe extern "C" fn(ctx: *mut RedisModuleCtx)
                                         -> c_int;
}
extern "C" {
    #[link_name = "RedisModule_IsBlockedTimeoutRequest"]
    pub static mut RedisModule_IsBlockedTimeoutRequest:
               unsafe extern "C" fn(ctx: *mut RedisModuleCtx)
                                         -> c_int;
}
extern "C" {
    #[link_name = "RedisModule_GetBlockedClientPrivateData"]
    pub static mut RedisModule_GetBlockedClientPrivateData:
               unsafe extern "C" fn(ctx: *mut RedisModuleCtx)
                                         -> *mut c_void;
}
extern "C" {
    #[link_name = "RedisModule_AbortBlock"]
    pub static mut RedisModule_AbortBlock: unsafe extern "C" fn(bc: *mut RedisModuleBlockedClient)
                                                                -> c_int;
}
extern "C" {
    #[link_name = "RedisModule_Milliseconds"]
    pub static mut RedisModule_Milliseconds: unsafe extern "C" fn() -> c_longlong;
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
