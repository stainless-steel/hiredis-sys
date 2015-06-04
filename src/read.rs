use libc::{c_char, c_int, c_longlong, c_void, size_t};

pub const REDIS_ERR: c_int = -1;
pub const REDIS_OK: c_int = 0;

pub const REDIS_ERR_IO: c_int = 1;
pub const REDIS_ERR_EOF: c_int = 3;
pub const REDIS_ERR_PROTOCOL: c_int = 4;
pub const REDIS_ERR_OOM: c_int = 5;
pub const REDIS_ERR_OTHER: c_int = 2;

pub const REDIS_REPLY_STRING: c_int = 1;
pub const REDIS_REPLY_ARRAY: c_int = 2;
pub const REDIS_REPLY_INTEGER: c_int = 3;
pub const REDIS_REPLY_NIL: c_int = 4;
pub const REDIS_REPLY_STATUS: c_int = 5;
pub const REDIS_REPLY_ERROR: c_int = 6;

#[repr(C)]
pub struct redisReadTask {
    pub kind: c_int, // type
    pub elements: c_int,
    pub idx: c_int,
    pub obj: *mut c_void,
    pub parent: *mut redisReadTask,
    pub privdata: *mut c_void,
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct redisReplyObjectFunctions {
    pub createString: Option<extern fn(*const redisReadTask, *mut c_char, size_t) -> *mut c_void>,
    pub createArray: Option<extern fn(*const redisReadTask, c_int) -> *mut c_void>,
    pub createInteger: Option<extern fn(*const redisReadTask, c_longlong) -> *mut c_void>,
    pub createNil: Option<extern fn(*const redisReadTask) -> *mut c_void>,
    pub freeObject: Option<extern fn(*mut c_void)>,
}

#[repr(C)]
pub struct redisReader {
    pub err: c_int,
    pub errstr: [c_char; 128],

    pub buf: *mut c_char,
    pub pos: size_t,
    pub len: size_t,
    pub maxbuf: size_t,

    pub rstack: [redisReadTask; 9],
    pub ridx: c_int,
    pub reply: *mut c_void,

    pub fns: *mut redisReplyObjectFunctions, // fn
    pub privdata: *mut c_void,
}

extern "C" {
    pub fn redisReaderCreateWithFunctions(fns: *mut redisReplyObjectFunctions) -> *mut redisReader;
    pub fn redisReaderFree(reader: *mut redisReader);
    pub fn redisReaderFeed(reader: *mut redisReader, buf: *const c_char, len: size_t) -> c_int;
    pub fn redisReaderGetReply(reader: *mut redisReader, reply: *mut *mut c_void) -> c_int;
}
