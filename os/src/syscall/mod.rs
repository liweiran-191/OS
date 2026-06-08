// // use crate::syscall::{sys_yield, sys_exit};

// const SYSCALL_YIELD: usize = 141;
// const SYSCALL_EXIT: usize = 93;

// // 必须包在 match 里面！
// pub fn sys_call(sys_id: usize, args: [usize; 3]) -> isize {
//     match sys_id {
//         SYSCALL_YIELD => sys_yield(),
//         SYSCALL_EXIT => sys_exit(),
//         _ => panic!("unknown syscall id: {}", sys_id),
//     }
// }
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

const SYSCALL_YIELD: usize = 124;

mod fs;
mod process;

use fs::*;
use process::*;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}