use libc::c_int;

pub const REDIS_ERR: c_int = -1;
pub const REDIS_OK: c_int = 0;

pub const REDIS_ERR_IO: c_int = 1;
pub const REDIS_ERR_EOF: c_int = 3;
pub const REDIS_ERR_PROTOCOL: c_int = 4;
pub const REDIS_ERR_OOM: c_int = 5;
pub const REDIS_ERR_OTHER: c_int = 2;
