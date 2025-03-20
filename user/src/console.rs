use super::write;
use core::fmt::{self, Write};

struct Stdout;

const STDOUT: usize = 1;
//传入的 fd 参数设置为 1，它代表标准输出， 也就是输出到屏幕

impl Write for Stdout {//使 Stdout 支持格式化输出
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDOUT, s.as_bytes());
        Ok(())
    }
}
//Write 是 core::fmt::Write trait，它定义了如何将格式化的字符串写入输出目标

pub fn print(args: fmt::Arguments) {//实际执行格式化输出
    Stdout.write_fmt(args).unwrap();
}
//fmt::Arguments 是格式化字符串参数的类型(format_args!() 生成它)
//write_fmt(args) 是 Write trait 提供的默认实现
//作用: write_fmt(args) 接收格式化参数 args，然后转换为字符串，再调用 write_str(s)

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}
//$crate 代表 user_lib，即 Cargo.toml 里的 package.name

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
