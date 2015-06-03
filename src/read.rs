use libc::{c_char, c_int, c_void, size_t};

pub const REDIS_ERR: c_int = -1;
pub const REDIS_OK: c_int = 0;

pub const REDIS_ERR_IO: c_int = 1;
pub const REDIS_ERR_EOF: c_int = 3;
pub const REDIS_ERR_PROTOCOL: c_int = 4;
pub const REDIS_ERR_OOM: c_int = 5;
pub const REDIS_ERR_OTHER: c_int = 2;

#[repr(C)]
pub struct redisReader {
    pub err: c_int,
    pub errstr: [c_char; 128],
    // ...
}

extern "C" {
    pub fn redisReaderFree(reader: *mut redisReader);
    pub fn redisReaderFeed(reader: *mut redisReader, buf: *const c_char, len: size_t) -> c_int;
    pub fn redisReaderGetReply(reader: *mut redisReader, reply: *mut *mut c_void) -> c_int;
}
