use libc::{c_char, c_int, c_void, size_t, timeval};

#[repr(C)]
pub struct redisContext {
    pub err: c_int,
    pub errstr: [c_char; 128],
    // ...
}

extern "C" {
    pub fn redisConnect(ip: *const c_char, port: c_int) -> *mut redisContext;

    pub fn redisConnectWithTimeout(ip: *const c_char, port: c_int, tv: timeval)
                                  -> *mut redisContext;

    pub fn redisFree(c: *mut redisContext);
    pub fn redisGetReply(c: *mut redisContext, reply: *mut *mut c_void) -> c_int;
    pub fn freeReplyObject(reply: *mut c_void);

    pub fn redisAppendCommandArgv(c: *mut redisContext, argc: c_int, argv: *const *const c_char,
                                  argvlen: *const size_t);

    pub fn redisCommandArgv(c: *mut redisContext, argc: c_int, argv: *const *const c_char,
                            argvlen: *const size_t) -> *mut c_void;
}
