const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    if fd == FD_STDOUT {
        let s = unsafe { core::slice::from_raw_parts(buf, len) };
        let str = core::str::from_utf8(s).unwrap();
        print!("{}", str);
        len as _
    } else {
        -1
    }
}
