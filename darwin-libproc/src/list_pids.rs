use std::io;
use std::mem;
use std::ptr;

fn list_pids(r#type: u32, typeinfo: u32) -> io::Result<Vec<libc::pid_t>> {
    let size = unsafe {
        darwin_libproc_sys::proc_listpids(r#type, typeinfo, ptr::null_mut(), 0)
    };
    if size <= 0 {
        return Err(io::Error::last_os_error());
    }

    let capacity = size as usize / mem::size_of::<libc::pid_t>();
    let mut buffer: Vec<libc::pid_t> = Vec::with_capacity(capacity);

    let result = unsafe {
        darwin_libproc_sys::proc_listpids(
            r#type,
            typeinfo,
            buffer.as_mut_ptr() as *mut libc::c_void,
            size,
        )
    };
    if result <= 0 {
        return Err(io::Error::last_os_error());
    }

    let pids_count = result as usize / mem::size_of::<libc::pid_t>() - 1;
    unsafe {
        buffer.set_len(pids_count);
    }

    Ok(buffer)
}

pub fn all_pids() -> io::Result<Vec<libc::pid_t>> {
    list_pids(darwin_libproc_sys::PROC_ALL_PIDS, 0)
}

pub fn pgrp_only_pids(group: u32) -> io::Result<Vec<libc::pid_t>> {
    list_pids(darwin_libproc_sys::PROC_PGRP_ONLY, group)
}

pub fn tty_only_pids(tty: u32) -> io::Result<Vec<libc::pid_t>> {
    list_pids(darwin_libproc_sys::PROC_TTY_ONLY, tty)
}

pub fn uid_only_pids(uid: u32) -> io::Result<Vec<libc::pid_t>> {
    list_pids(darwin_libproc_sys::PROC_UID_ONLY, uid)
}

pub fn ruid_only_pids(ruid: u32) -> io::Result<Vec<libc::pid_t>> {
    list_pids(darwin_libproc_sys::PROC_RUID_ONLY, ruid)
}

pub fn ppid_only_pids(ppid: u32) -> io::Result<Vec<libc::pid_t>> {
    list_pids(darwin_libproc_sys::PROC_PPID_ONLY, ppid)
}
