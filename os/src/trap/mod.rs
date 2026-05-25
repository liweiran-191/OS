use core::arch::global_asm;
use riscv::register::{stvec, scause::Trap, stval};
use crate::syscall::syscall;
use crate::batch::run_next_app;

global_asm!(include_str!("trap.S"));

mod context;
pub use context::TrapContext;

pub fn init() {
    unsafe extern "C" { fn __alltraps(); };
    unsafe { stvec::write(__alltraps as usize, stvec::TrapMode::Direct); }
}

#[unsafe(no_mangle)]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let cause = riscv::register::scause::read().cause();
    let stval = stval::read();
    match cause {
        Trap::Exception(riscv::register::scause::Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(riscv::register::scause::Exception::StoreFault) |
        Trap::Exception(riscv::register::scause::Exception::StorePageFault) => {
            println!("[kernel] PageFault, kill app");
            run_next_app();
        }
        Trap::Exception(riscv::register::scause::Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction, kill app");
            run_next_app();
        }
        _ => {
            panic!("Unsupported trap: {:?}, stval = {:#x}", cause, stval);
        }
    }
    cx
}
