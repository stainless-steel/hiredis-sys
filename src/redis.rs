use libc::{c_char, c_int, c_longlong, c_void, size_t, timeval};

use read::redisReader;

pub const REDIS_BLOCK: c_int = 0x1;
pub const REDIS_CONNECTED: c_int = 0x2;
pub const REDIS_DISCONNECTING: c_int = 0x4;
pub const REDIS_FREEING: c_int = 0x8;
pub const REDIS_IN_CALLBACK: c_int = 0x10;
pub const REDIS_SUBSCRIBED: c_int = 0x20;
pub const REDIS_MONITORING: c_int = 0x40;
pub const REDIS_REUSEADDR: c_int = 0x80;

#[repr(C)]
pub struct redisReply {
    pub kind: c_int, // type
    pub integer: c_longlong,
    pub len: c_int,
    pub string: *mut c_char, // str
    pub elements: size_t,
    pub element: *mut *mut redisReply,
}

extern "C" {
    pub fn redisReaderCreate() -> *mut redisReader;

    pub fn freeReplyObject(reply: *mut c_void);

    pub fn redisFormatCommandArgv(target: *mut *mut c_char, argc: c_int, argv: *mut *const c_char,
                                  argvlen: *const size_t) -> c_int;
    pub fn redisFreeCommand(cmd: *mut c_char);
}

#[repr(C)]
pub enum redisConnectionType {
    REDIS_CONN_TCP,
    REDIS_CONN_UNIX,
}
pub use self::redisConnectionType::*;

#[repr(C)]
pub struct redisContext {
    pub err: c_int,
    pub errstr: [c_char; 128],
    pub fd: c_int,
    pub flags: c_int,
    pub obuf: *mut c_char,
    pub reader: *mut redisReader,

    pub connection_type: redisConnectionType,
    pub timeout: *mut timeval,

    pub tcp: tcp,

    pub unix_sock: unix_sock,
}

#[repr(C)]
pub struct tcp {
    pub host: *mut c_char,
    pub source_addr: *mut c_char,
    pub port: c_int,
}

#[repr(C)]
pub struct unix_sock {
    pub path: *mut c_char,
}

extern "C" {
    pub fn redisConnect(ip: *const c_char, port: c_int) -> *mut redisContext;
    pub fn redisConnectWithTimeout(ip: *const c_char, port: c_int, tv: timeval)
                                   -> *mut redisContext;
    pub fn redisConnectNonBlock(ip: *const c_char, port: c_int) -> *mut redisContext;
    pub fn redisConnectBindNonBlock(ip: *const c_char, port: c_int, source_addr: *const c_char)
                                    -> *mut redisContext;
    pub fn redisConnectBindNonBlockWithReuse(ip: *const c_char, port: c_int,
                                             source_addr: *const c_char) -> *mut redisContext;
    pub fn redisConnectUnix(path: *const c_char) -> *mut redisContext;
    pub fn redisConnectUnixWithTimeout(path: *const c_char, tv: timeval) -> *mut redisContext;
    pub fn redisConnectUnixNonBlock(path: *const c_char) -> *mut redisContext;
    pub fn redisConnectFd(fd: c_int) -> *mut redisContext;

    pub fn redisReconnect(c: *mut redisContext) -> c_int;

    pub fn redisSetTimeout(c: *mut redisContext, tv: timeval) -> c_int;
    pub fn redisEnableKeepAlive(c: *mut redisContext) -> c_int;
    pub fn redisFree(c: *mut redisContext);
    pub fn redisFreeKeepFd(c: *mut redisContext) -> c_int;
    pub fn redisBufferRead(c: *mut redisContext) -> c_int;
    pub fn redisBufferWrite(c: *mut redisContext, done: *mut c_int) -> c_int;

    pub fn redisGetReply(c: *mut redisContext, reply: *mut *mut c_void) -> c_int;
    pub fn redisGetReplyFromReader(c: *mut redisContext, reply: *mut *mut c_void) -> c_int;

    pub fn redisAppendFormattedCommand(c: *mut redisContext, cmd: *const c_char, len: size_t)
                                       -> c_int;

    pub fn redisAppendCommandArgv(c: *mut redisContext, argc: c_int, argv: *mut *const c_char,
                                  argvlen: *const size_t) -> c_int;

    pub fn redisCommandArgv(c: *mut redisContext, argc: c_int, argv: *mut *const c_char,
                            argvlen: *const size_t) -> *mut c_void;
}
