use core::panic::PanicInfo;
use crate::{println, sbi::shutdown};

#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    //loop{}
    if let Some(location) = info.location(){
        println!(
            "\u{1B}[31mPanicked at {}:{} {}\u{1B}[0m",
            //"Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message()
        );
    } else {
        println!("\u{1B}[31m Panicked at: {}\u{1B}[0m", info.message());
        //\u{1B}[31mPanicked: {} \u{1B}[0m
    }
    shutdown(true)
}