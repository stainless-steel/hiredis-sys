extern crate libc;

mod read;
mod redis;

pub use read::*;
pub use redis::*;

#[cfg(test)]
mod tests {
    macro_rules! ok(
        ($result:expr) => ($result.unwrap());
    );

    macro_rules! c_str(
        ($string:expr) => (ok!(::std::ffi::CString::new($string)).as_ptr());
    );

    #[test]
    fn connect() {
        unsafe {
            let context = ::redisConnect(c_str!("127.0.0.1"), 6379);
            assert!(!context.is_null());
            assert!((*context).err == ::REDIS_OK);
            ::redisFree(context);
        }
    }
}
