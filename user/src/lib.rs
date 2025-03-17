#![no_std]
#![feature(linkage)]

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;

//定义了用户库的入口点 _start ：
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
//使用 Rust 的宏将 _start 这段代码编译后的汇编代码中放在一个名为 .text.entry 的代码段中
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[unsafe(no_mangle)]
fn main() -> i32 {
    panic!("Cannot find main!");
}

fn clear_bss() {
    unsafe extern "C" {
        safe fn start_bss();
        safe fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}

//将user/src/syscall.rs中两个系统调用在用户库 user_lib 中进一步封装，
//从而更加接近在 Linux 等平台的实际系统调用接口：
//把 console 子模块中 Stdout::write_str 改成基于 write 的实现，
//且传入的 fd 参数设置为 1，它代表标准输出， 也就是输出到屏幕。
use syscall::*;

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}
