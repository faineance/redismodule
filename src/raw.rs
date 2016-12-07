
use libc::{c_int, c_uint};
use std::f32;
pub const REDISMODULE_APIVER_1: c_int = 1;

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
pub type RedisModuleCmdFunc =
    unsafe extern "C" fn(ctx: *mut RedisModuleCtx,
                                                 argv: *mut *mut RedisModuleString,
                                                 argc: ::std::os::raw::c_int)
                                                 -> Status;
pub type RedisModuleTypeLoadFunc =
    ::std::option::Option<unsafe extern "C" fn(rdb: *mut RedisModuleIO,
                                                 encver: ::std::os::raw::c_int)
                                                 -> *mut ::std::os::raw::c_void>;
pub type RedisModuleTypeSaveFunc =
    ::std::option::Option<unsafe extern "C" fn(rdb: *mut RedisModuleIO,
                                                 value: *mut ::std::os::raw::c_void)>;
pub type RedisModuleTypeRewriteFunc =
    ::std::option::Option<unsafe extern "C" fn(aof: *mut RedisModuleIO,
                                                 key: *mut RedisModuleString,
                                                 value: *mut ::std::os::raw::c_void)>;
pub type RedisModuleTypeMemUsageFunc =
    ::std::option::Option<unsafe extern "C" fn(value: *mut ::std::os::raw::c_void)
                                                 -> ::std::os::raw::c_ulong>;
pub type RedisModuleTypeDigestFunc =
    ::std::option::Option<unsafe extern "C" fn(digest: *mut RedisModuleDigest,
                                                 value: *mut ::std::os::raw::c_void)>;
pub type RedisModuleTypeFreeFunc =
    ::std::option::Option<unsafe extern "C" fn(value: *mut ::std::os::raw::c_void)>;
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
extern "C" {
    #[link_name = "RedisModule_Alloc"]
    pub static mut RedisModule_Alloc:
               ::std::option::Option<unsafe extern "C" fn(bytes: usize)
                                         -> *mut ::std::os::raw::c_void>;
}
extern "C" {
    #[link_name = "RedisModule_Realloc"]
    pub static mut RedisModule_Realloc:
               ::std::option::Option<unsafe extern "C" fn(ptr:
                                                              *mut ::std::os::raw::c_void,
                                                          bytes: usize)
                                         -> *mut ::std::os::raw::c_void>;
}
extern "C" {
    #[link_name = "RedisModule_Free"]
    pub static mut RedisModule_Free:
               ::std::option::Option<unsafe extern "C" fn(ptr:
                                                              *mut ::std::os::raw::c_void)>;
}
extern "C" {
    #[link_name = "RedisModule_Calloc"]
    pub static mut RedisModule_Calloc:
               ::std::option::Option<unsafe extern "C" fn(nmemb: usize,
                                                          size: usize)
                                         -> *mut ::std::os::raw::c_void>;
}
extern "C" {
    #[link_name = "RedisModule_Strdup"]
    pub static mut RedisModule_Strdup:
               ::std::option::Option<unsafe extern "C" fn(str:
                                                              *const ::std::os::raw::c_char)
                                         -> *mut ::std::os::raw::c_char>;
}
extern "C" {
    #[link_name = "RedisModule_GetApi"]
    pub static mut RedisModule_GetApi:
               ::std::option::Option<unsafe extern "C" fn(arg1:
                                                              *const ::std::os::raw::c_char,
                                                          arg2:
                                                              *mut ::std::os::raw::c_void)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_CreateCommand"]
    pub static mut RedisModule_CreateCommand:
               unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          name:
                                                              *const ::std::os::raw::c_char,
                                                          cmdfunc:
                                                              RedisModuleCmdFunc,
                                                          strflags:
                                                              *const ::std::os::raw::c_char,
                                                          firstkey:
                                                              ::std::os::raw::c_int,
                                                          lastkey:
                                                              ::std::os::raw::c_int,
                                                          keystep:
                                                              ::std::os::raw::c_int)
                                         -> Status;
}
extern "C" {
    #[link_name = "RedisModule_SetModuleAttribs"]
    pub static mut RedisModule_SetModuleAttribs:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          name:
                                                              *const ::std::os::raw::c_char,
                                                          ver:
                                                              ::std::os::raw::c_int,
                                                          apiver:
                                                              ::std::os::raw::c_int)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_WrongArity"]
    pub static mut RedisModule_WrongArity:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ReplyWithLongLong"]
    pub static mut RedisModule_ReplyWithLongLong:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          ll:
                                                              ::std::os::raw::c_longlong)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_GetSelectedDb"]
    pub static mut RedisModule_GetSelectedDb:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_SelectDb"]
    pub static mut RedisModule_SelectDb:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          newid:
                                                              ::std::os::raw::c_int)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_OpenKey"]
    pub static mut RedisModule_OpenKey:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          keyname:
                                                              *mut RedisModuleString,
                                                          mode:
                                                              ::std::os::raw::c_int)
                                         -> *mut ::std::os::raw::c_void>;
}
extern "C" {
    #[link_name = "RedisModule_CloseKey"]
    pub static mut RedisModule_CloseKey:
               ::std::option::Option<unsafe extern "C" fn(kp:
                                                              *mut RedisModuleKey)>;
}
extern "C" {
    #[link_name = "RedisModule_KeyType"]
    pub static mut RedisModule_KeyType:
               ::std::option::Option<unsafe extern "C" fn(kp:
                                                              *mut RedisModuleKey)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ValueLength"]
    pub static mut RedisModule_ValueLength:
               ::std::option::Option<unsafe extern "C" fn(kp:
                                                              *mut RedisModuleKey)
                                         -> ::std::os::raw::c_ulong>;
}
extern "C" {
    #[link_name = "RedisModule_ListPush"]
    pub static mut RedisModule_ListPush:
               ::std::option::Option<unsafe extern "C" fn(kp:
                                                              *mut RedisModuleKey,
                                                          where_:
                                                              ::std::os::raw::c_int,
                                                          ele:
                                                              *mut RedisModuleString)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ListPop"]
    pub static mut RedisModule_ListPop:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey,
                                                          where_:
                                                              ::std::os::raw::c_int)
                                         -> *mut RedisModuleString>;
}
extern "C" {
    #[link_name = "RedisModule_Call"]
    pub static mut RedisModule_Call:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          cmdname:
                                                              *const ::std::os::raw::c_char,
                                                          fmt:
                                                              *const ::std::os::raw::c_char, ...)
                                         -> *mut RedisModuleCallReply>;
}
extern "C" {
    #[link_name = "RedisModule_CallReplyProto"]
    pub static mut RedisModule_CallReplyProto:
               ::std::option::Option<unsafe extern "C" fn(reply:
                                                              *mut RedisModuleCallReply,
                                                          len: *mut usize)
                                         -> *const ::std::os::raw::c_char>;
}
extern "C" {
    #[link_name = "RedisModule_FreeCallReply"]
    pub static mut RedisModule_FreeCallReply:
               ::std::option::Option<unsafe extern "C" fn(reply:
                                                              *mut RedisModuleCallReply)>;
}
extern "C" {
    #[link_name = "RedisModule_CallReplyType"]
    pub static mut RedisModule_CallReplyType:
               ::std::option::Option<unsafe extern "C" fn(reply:
                                                              *mut RedisModuleCallReply)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_CallReplyInteger"]
    pub static mut RedisModule_CallReplyInteger:
               ::std::option::Option<unsafe extern "C" fn(reply:
                                                              *mut RedisModuleCallReply)
                                         -> ::std::os::raw::c_longlong>;
}
extern "C" {
    #[link_name = "RedisModule_CallReplyLength"]
    pub static mut RedisModule_CallReplyLength:
               ::std::option::Option<unsafe extern "C" fn(reply:
                                                              *mut RedisModuleCallReply)
                                         -> ::std::os::raw::c_ulong>;
}
extern "C" {
    #[link_name = "RedisModule_CallReplyArrayElement"]
    pub static mut RedisModule_CallReplyArrayElement:
               ::std::option::Option<unsafe extern "C" fn(reply:
                                                              *mut RedisModuleCallReply,
                                                          idx: usize)
                                         -> *mut RedisModuleCallReply>;
}
extern "C" {
    #[link_name = "RedisModule_CreateString"]
    pub static mut RedisModule_CreateString:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          ptr:
                                                              *const ::std::os::raw::c_char,
                                                          len: usize)
                                         -> *mut RedisModuleString>;
}
extern "C" {
    #[link_name = "RedisModule_CreateStringFromLongLong"]
    pub static mut RedisModule_CreateStringFromLongLong:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          ll:
                                                              ::std::os::raw::c_longlong)
                                         -> *mut RedisModuleString>;
}
extern "C" {
    #[link_name = "RedisModule_CreateStringFromString"]
    pub static mut RedisModule_CreateStringFromString:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          str:
                                                              *const RedisModuleString)
                                         -> *mut RedisModuleString>;
}
extern "C" {
    #[link_name = "RedisModule_CreateStringPrintf"]
    pub static mut RedisModule_CreateStringPrintf:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          fmt:
                                                              *const ::std::os::raw::c_char, ...)
                                         -> *mut RedisModuleString>;
}
extern "C" {
    #[link_name = "RedisModule_FreeString"]
    pub static mut RedisModule_FreeString:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          str:
                                                              *mut RedisModuleString)>;
}
extern "C" {
    #[link_name = "RedisModule_StringPtrLen"]
    pub static mut RedisModule_StringPtrLen:
               ::std::option::Option<unsafe extern "C" fn(str:
                                                              *const RedisModuleString,
                                                          len: *mut usize)
                                         -> *const ::std::os::raw::c_char>;
}
extern "C" {
    #[link_name = "RedisModule_ReplyWithError"]
    pub static mut RedisModule_ReplyWithError:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          err:
                                                              *const ::std::os::raw::c_char)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ReplyWithSimpleString"]
    pub static mut RedisModule_ReplyWithSimpleString:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          msg:
                                                              *const ::std::os::raw::c_char)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ReplyWithArray"]
    pub static mut RedisModule_ReplyWithArray:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          len:
                                                              ::std::os::raw::c_long)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ReplySetArrayLength"]
    pub static mut RedisModule_ReplySetArrayLength:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          len:
                                                              ::std::os::raw::c_long)>;
}
extern "C" {
    #[link_name = "RedisModule_ReplyWithStringBuffer"]
    pub static mut RedisModule_ReplyWithStringBuffer:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          buf:
                                                              *const ::std::os::raw::c_char,
                                                          len: usize)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ReplyWithString"]
    pub static mut RedisModule_ReplyWithString:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          str:
                                                              *mut RedisModuleString)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ReplyWithNull"]
    pub static mut RedisModule_ReplyWithNull:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ReplyWithDouble"]
    pub static mut RedisModule_ReplyWithDouble:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          d: f64)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ReplyWithCallReply"]
    pub static mut RedisModule_ReplyWithCallReply:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          reply:
                                                              *mut RedisModuleCallReply)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_StringToLongLong"]
    pub static mut RedisModule_StringToLongLong:
               ::std::option::Option<unsafe extern "C" fn(str:
                                                              *const RedisModuleString,
                                                          ll:
                                                              *mut ::std::os::raw::c_longlong)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_StringToDouble"]
    pub static mut RedisModule_StringToDouble:
               ::std::option::Option<unsafe extern "C" fn(str:
                                                              *const RedisModuleString,
                                                          d: *mut f64)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_AutoMemory"]
    pub static mut RedisModule_AutoMemory:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx)>;
}
extern "C" {
    #[link_name = "RedisModule_Replicate"]
    pub static mut RedisModule_Replicate:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          cmdname:
                                                              *const ::std::os::raw::c_char,
                                                          fmt:
                                                              *const ::std::os::raw::c_char, ...)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ReplicateVerbatim"]
    pub static mut RedisModule_ReplicateVerbatim:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_CallReplyStringPtr"]
    pub static mut RedisModule_CallReplyStringPtr:
               ::std::option::Option<unsafe extern "C" fn(reply:
                                                              *mut RedisModuleCallReply,
                                                          len: *mut usize)
                                         -> *const ::std::os::raw::c_char>;
}
extern "C" {
    #[link_name = "RedisModule_CreateStringFromCallReply"]
    pub static mut RedisModule_CreateStringFromCallReply:
               ::std::option::Option<unsafe extern "C" fn(reply:
                                                              *mut RedisModuleCallReply)
                                         -> *mut RedisModuleString>;
}
extern "C" {
    #[link_name = "RedisModule_DeleteKey"]
    pub static mut RedisModule_DeleteKey:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_StringSet"]
    pub static mut RedisModule_StringSet:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey,
                                                          str:
                                                              *mut RedisModuleString)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_StringDMA"]
    pub static mut RedisModule_StringDMA:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey,
                                                          len: *mut usize,
                                                          mode:
                                                              ::std::os::raw::c_int)
                                         -> *mut ::std::os::raw::c_char>;
}
extern "C" {
    #[link_name = "RedisModule_StringTruncate"]
    pub static mut RedisModule_StringTruncate:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey,
                                                          newlen: usize)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_GetExpire"]
    pub static mut RedisModule_GetExpire:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey)
                                         -> ::std::os::raw::c_longlong>;
}
extern "C" {
    #[link_name = "RedisModule_SetExpire"]
    pub static mut RedisModule_SetExpire:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey,
                                                          expire: ::std::os::raw::c_longlong)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ZsetAdd"]
    pub static mut RedisModule_ZsetAdd:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey,
                                                          score: f64,
                                                          ele:
                                                              *mut RedisModuleString,
                                                          flagsptr:
                                                              *mut ::std::os::raw::c_int)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ZsetIncrby"]
    pub static mut RedisModule_ZsetIncrby:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey,
                                                          score: f64,
                                                          ele:
                                                              *mut RedisModuleString,
                                                          flagsptr:
                                                              *mut ::std::os::raw::c_int,
                                                          newscore: *mut f64)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ZsetScore"]
    pub static mut RedisModule_ZsetScore:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey,
                                                          ele:
                                                              *mut RedisModuleString,
                                                          score: *mut f64)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ZsetRem"]
    pub static mut RedisModule_ZsetRem:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey,
                                                          ele:
                                                              *mut RedisModuleString,
                                                          deleted:
                                                              *mut ::std::os::raw::c_int)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ZsetRangeStop"]
    pub static mut RedisModule_ZsetRangeStop:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey)>;
}
extern "C" {
    #[link_name = "RedisModule_ZsetFirstInScoreRange"]
    pub static mut RedisModule_ZsetFirstInScoreRange:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey,
                                                          min: f64, max: f64,
                                                          minex:
                                                              ::std::os::raw::c_int,
                                                          maxex:
                                                              ::std::os::raw::c_int)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ZsetLastInScoreRange"]
    pub static mut RedisModule_ZsetLastInScoreRange:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey,
                                                          min: f64, max: f64,
                                                          minex:
                                                              ::std::os::raw::c_int,
                                                          maxex:
                                                              ::std::os::raw::c_int)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ZsetFirstInLexRange"]
    pub static mut RedisModule_ZsetFirstInLexRange:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey,
                                                          min:
                                                              *mut RedisModuleString,
                                                          max:
                                                              *mut RedisModuleString)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ZsetLastInLexRange"]
    pub static mut RedisModule_ZsetLastInLexRange:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey,
                                                          min:
                                                              *mut RedisModuleString,
                                                          max:
                                                              *mut RedisModuleString)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ZsetRangeCurrentElement"]
    pub static mut RedisModule_ZsetRangeCurrentElement:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey,
                                                          score: *mut f64)
                                         -> *mut RedisModuleString>;
}
extern "C" {
    #[link_name = "RedisModule_ZsetRangeNext"]
    pub static mut RedisModule_ZsetRangeNext:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ZsetRangePrev"]
    pub static mut RedisModule_ZsetRangePrev:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ZsetRangeEndReached"]
    pub static mut RedisModule_ZsetRangeEndReached:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_HashSet"]
    pub static mut RedisModule_HashSet:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey,
                                                          flags:
                                                              ::std::os::raw::c_int, ...)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_HashGet"]
    pub static mut RedisModule_HashGet:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey,
                                                          flags:
                                                              ::std::os::raw::c_int, ...)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_IsKeysPositionRequest"]
    pub static mut RedisModule_IsKeysPositionRequest:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_KeyAtPos"]
    pub static mut RedisModule_KeyAtPos:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          pos:
                                                              ::std::os::raw::c_int)>;
}
extern "C" {
    #[link_name = "RedisModule_GetClientId"]
    pub static mut RedisModule_GetClientId:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx)
                                         -> ::std::os::raw::c_ulonglong>;
}
extern "C" {
    #[link_name = "RedisModule_PoolAlloc"]
    pub static mut RedisModule_PoolAlloc:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          bytes: usize)
                                         -> *mut ::std::os::raw::c_void>;
}
extern "C" {
    #[link_name = "RedisModule_CreateDataType"]
    pub static mut RedisModule_CreateDataType:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          name:
                                                              *const ::std::os::raw::c_char,
                                                          encver:
                                                              ::std::os::raw::c_int,
                                                          typemethods:
                                                              *mut RedisModuleTypeMethods)
                                         -> *mut RedisModuleType>;
}
extern "C" {
    #[link_name = "RedisModule_ModuleTypeSetValue"]
    pub static mut RedisModule_ModuleTypeSetValue:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey,
                                                          mt:
                                                              *mut RedisModuleType,
                                                          value:
                                                              *mut ::std::os::raw::c_void)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_ModuleTypeGetType"]
    pub static mut RedisModule_ModuleTypeGetType:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey)
                                         -> *mut RedisModuleType>;
}
extern "C" {
    #[link_name = "RedisModule_ModuleTypeGetValue"]
    pub static mut RedisModule_ModuleTypeGetValue:
               ::std::option::Option<unsafe extern "C" fn(key:
                                                              *mut RedisModuleKey)
                                         -> *mut ::std::os::raw::c_void>;
}
extern "C" {
    #[link_name = "RedisModule_SaveUnsigned"]
    pub static mut RedisModule_SaveUnsigned:
               ::std::option::Option<unsafe extern "C" fn(io:
                                                              *mut RedisModuleIO,
                                                          value: u64)>;
}
extern "C" {
    #[link_name = "RedisModule_LoadUnsigned"]
    pub static mut RedisModule_LoadUnsigned:
               ::std::option::Option<unsafe extern "C" fn(io:
                                                              *mut RedisModuleIO)
                                         -> ::std::os::raw::c_ulong>;
}
extern "C" {
    #[link_name = "RedisModule_SaveSigned"]
    pub static mut RedisModule_SaveSigned:
               ::std::option::Option<unsafe extern "C" fn(io:
                                                              *mut RedisModuleIO,
                                                          value: i64)>;
}
extern "C" {
    #[link_name = "RedisModule_LoadSigned"]
    pub static mut RedisModule_LoadSigned:
               ::std::option::Option<unsafe extern "C" fn(io:
                                                              *mut RedisModuleIO)
                                         -> ::std::os::raw::c_long>;
}
extern "C" {
    #[link_name = "RedisModule_EmitAOF"]
    pub static mut RedisModule_EmitAOF:
               ::std::option::Option<unsafe extern "C" fn(io:
                                                              *mut RedisModuleIO,
                                                          cmdname:
                                                              *const ::std::os::raw::c_char,
                                                          fmt:
                                                              *const ::std::os::raw::c_char, ...)>;
}
extern "C" {
    #[link_name = "RedisModule_SaveString"]
    pub static mut RedisModule_SaveString:
               ::std::option::Option<unsafe extern "C" fn(io:
                                                              *mut RedisModuleIO,
                                                          s:
                                                              *mut RedisModuleString)>;
}
extern "C" {
    #[link_name = "RedisModule_SaveStringBuffer"]
    pub static mut RedisModule_SaveStringBuffer:
               ::std::option::Option<unsafe extern "C" fn(io:
                                                              *mut RedisModuleIO,
                                                          str:
                                                              *const ::std::os::raw::c_char,
                                                          len: usize)>;
}
extern "C" {
    #[link_name = "RedisModule_LoadString"]
    pub static mut RedisModule_LoadString:
               ::std::option::Option<unsafe extern "C" fn(io:
                                                              *mut RedisModuleIO)
                                         -> *mut RedisModuleString>;
}
extern "C" {
    #[link_name = "RedisModule_LoadStringBuffer"]
    pub static mut RedisModule_LoadStringBuffer:
               ::std::option::Option<unsafe extern "C" fn(io:
                                                              *mut RedisModuleIO,
                                                          lenptr: *mut usize)
                                         -> *mut ::std::os::raw::c_char>;
}
extern "C" {
    #[link_name = "RedisModule_SaveDouble"]
    pub static mut RedisModule_SaveDouble:
               ::std::option::Option<unsafe extern "C" fn(io:
                                                              *mut RedisModuleIO,
                                                          value: f64)>;
}
extern "C" {
    #[link_name = "RedisModule_LoadDouble"]
    pub static mut RedisModule_LoadDouble:
               ::std::option::Option<unsafe extern "C" fn(io:
                                                              *mut RedisModuleIO)
                                         -> f64>;
}
extern "C" {
    #[link_name = "RedisModule_SaveFloat"]
    pub static mut RedisModule_SaveFloat:
               ::std::option::Option<unsafe extern "C" fn(io:
                                                              *mut RedisModuleIO,
                                                          value: f32)>;
}
extern "C" {
    #[link_name = "RedisModule_LoadFloat"]
    pub static mut RedisModule_LoadFloat:
               ::std::option::Option<unsafe extern "C" fn(io:
                                                              *mut RedisModuleIO)
                                         -> f32>;
}
extern "C" {
    #[link_name = "RedisModule_Log"]
    pub static mut RedisModule_Log:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          level:
                                                              *const ::std::os::raw::c_char,
                                                          fmt:
                                                              *const ::std::os::raw::c_char, ...)>;
}
extern "C" {
    #[link_name = "RedisModule_LogIOError"]
    pub static mut RedisModule_LogIOError:
               ::std::option::Option<unsafe extern "C" fn(io:
                                                              *mut RedisModuleIO,
                                                          levelstr:
                                                              *const ::std::os::raw::c_char,
                                                          fmt:
                                                              *const ::std::os::raw::c_char, ...)>;
}
extern "C" {
    #[link_name = "RedisModule_StringAppendBuffer"]
    pub static mut RedisModule_StringAppendBuffer:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          str:
                                                              *mut RedisModuleString,
                                                          buf:
                                                              *const ::std::os::raw::c_char,
                                                          len: usize)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_RetainString"]
    pub static mut RedisModule_RetainString:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          str:
                                                              *mut RedisModuleString)>;
}
extern "C" {
    #[link_name = "RedisModule_StringCompare"]
    pub static mut RedisModule_StringCompare:
               ::std::option::Option<unsafe extern "C" fn(a:
                                                              *mut RedisModuleString,
                                                          b:
                                                              *mut RedisModuleString)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_GetContextFromIO"]
    pub static mut RedisModule_GetContextFromIO:
               ::std::option::Option<unsafe extern "C" fn(io:
                                                              *mut RedisModuleIO)
                                         -> *mut RedisModuleCtx>;
}
extern "C" {
    #[link_name = "RedisModule_BlockClient"]
    pub static mut RedisModule_BlockClient:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx,
                                                          reply_callback:
                                                              RedisModuleCmdFunc,
                                                          timeout_callback:
                                                              RedisModuleCmdFunc,
                                                          free_privdata:
                                                              ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                                                             *mut ::std::os::raw::c_void)>,
                                                          timeout_ms:
                                                              ::std::os::raw::c_longlong)
                                         -> *mut RedisModuleBlockedClient>;
}
extern "C" {
    #[link_name = "RedisModule_UnblockClient"]
    pub static mut RedisModule_UnblockClient:
               ::std::option::Option<unsafe extern "C" fn(bc:
                                                              *mut RedisModuleBlockedClient,
                                                          privdata:
                                                              *mut ::std::os::raw::c_void)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_IsBlockedReplyRequest"]
    pub static mut RedisModule_IsBlockedReplyRequest:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_IsBlockedTimeoutRequest"]
    pub static mut RedisModule_IsBlockedTimeoutRequest:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_GetBlockedClientPrivateData"]
    pub static mut RedisModule_GetBlockedClientPrivateData:
               ::std::option::Option<unsafe extern "C" fn(ctx:
                                                              *mut RedisModuleCtx)
                                         -> *mut ::std::os::raw::c_void>;
}
extern "C" {
    #[link_name = "RedisModule_AbortBlock"]
    pub static mut RedisModule_AbortBlock:
               ::std::option::Option<unsafe extern "C" fn(bc:
                                                              *mut RedisModuleBlockedClient)
                                         -> ::std::os::raw::c_int>;
}
extern "C" {
    #[link_name = "RedisModule_Milliseconds"]
    pub static mut RedisModule_Milliseconds:
               ::std::option::Option<unsafe extern "C" fn()
                                         -> ::std::os::raw::c_longlong>;
}
extern "C" {
    pub fn Export_RedisModule_Init(ctx: *mut RedisModuleCtx,
                                   name: *const ::std::os::raw::c_char,
                                   ver: ::std::os::raw::c_int,
                                   apiver: ::std::os::raw::c_int)
                                   -> Status;
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
