use libc::{c_char, c_int, c_longlong, c_void, size_t, timeval};

use read::redisReader;

#[repr(C)]
pub struct redisReply {
    pub kind: c_int, // type
    pub integer: c_longlong,
    pub len: c_int,
    pub string: *mut c_char, // str
    // ...
}

#[repr(C)]
pub struct redisContext {
    pub err: c_int,
    pub errstr: [c_char; 128],
    // ...
}

extern "C" {
    pub fn redisReaderCreate() -> *mut redisReader;
    pub fn freeReplyObject(reply: *mut c_void);
    pub fn redisConnect(ip: *const c_char, port: c_int) -> *mut redisContext;

    pub fn redisConnectWithTimeout(ip: *const c_char, port: c_int, tv: timeval)
                                   -> *mut redisContext;

    pub fn redisReconnect(c: *mut redisContext) -> c_int;
    pub fn redisFree(c: *mut redisContext);
    pub fn redisGetReply(c: *mut redisContext, reply: *mut *mut c_void) -> c_int;

    pub fn redisAppendCommandArgv(c: *mut redisContext, argc: c_int, argv: *const *const c_char,
                                  argvlen: *const size_t);

    pub fn redisCommandArgv(c: *mut redisContext, argc: c_int, argv: *const *const c_char,
                            argvlen: *const size_t) -> *mut c_void;
}
