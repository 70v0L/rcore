#![no_std]
#![no_main]
mod sbi;
mod lang_items;

#[macro_use]
mod console;

//#![feature(panic_info_message)]

// fn main() {
//     //println!("Hello, world!");
// }
use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

//#[no_mangle]
#[unsafe(no_mangle)]
pub fn rust_main() -> !{
    //在内核初始化中，需要先完成对 `.bss` 段的清零
    clear_bss();
    sbi::console_putchar('O' as usize);  // 将字符 'O' 转换为 usize
    sbi::console_putchar('k' as usize);  // 将字符 'k' 转换为 usize
    sbi::console_putchar('\n' as usize);  // 将字符 'k' 转换为 usize

    //打印Hello workd!
    print!("Hello world!");//不换行
    println!("Hello world!!");//换行
    panic!("Shutdown machine!");
    //sbi::shutdown(false);
}

fn clear_bss(){
    unsafe extern "C"{
        unsafe fn sbss();
        unsafe fn ebss();
    }
    (sbss as usize .. ebss as usize).for_each(|a|{
        unsafe {
            (a as *mut u8).write_volatile(0);
        }
    });
}