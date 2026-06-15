// const FD_STDOUT: usize = 1;

// pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
//     if fd == FD_STDOUT {
//         let s = unsafe { core::slice::from_raw_parts(buf, len) };
//         let str = core::str::from_utf8(s).unwrap();
//         print!("{}", str);
//         len as _
//     } else {
//         -1
//     }
// }

use crate::mm::translated_byte_buffer;
use crate::task::current_user_token;

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let buffers = translated_byte_buffer(current_user_token(), buf, len);
            for buffer in buffers {
                print!("{}", core::str::from_utf8(buffer).unwrap());
            }
            len as isize
        },
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}