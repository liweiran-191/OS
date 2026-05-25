#![no_std]
#![no_main]

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod syscall;
mod trap;
mod batch;

core::arch::global_asm!(include_str!("entry.asm"));
core::arch::global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    let start = sbss as usize;
    let end = ebss as usize;
    for a in start..end {
        unsafe { (a as *mut u8).write_volatile(0); }
    }
}

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] Hello, world!");
    trap::init();
    batch::init();
    batch::run_next_app();
}
