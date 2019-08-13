use std::ffi::OsString;
use std::io;
use std::os::unix::ffi::OsStringExt;

pub fn name(pid: libc::pid_t) -> io::Result<OsString> {
    let mut buffer: Vec<u8> =
        Vec::with_capacity(darwin_libproc_sys::PROC_PIDPATHINFO_MAXSIZE);

    let result = unsafe {
        darwin_libproc_sys::proc_name(
            pid,
            buffer.as_mut_ptr() as *mut libc::c_void,
            buffer.capacity() as u32,
        )
    };
    if result <= 0 {
        Err(io::Error::last_os_error())
    } else {
        unsafe {
            buffer.set_len(result as usize);
        }

        Ok(OsString::from_vec(buffer))
    }
}
