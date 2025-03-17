use core::arch::asm;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    //将所有的系统调用都封装成 `syscall` 函数，可以看到它支持传入 syscall ID 和 3 个参数
    let mut ret: isize;
    unsafe {
        asm!(//asm! 宏可以获取上下文中的变量信息并允许嵌入的汇编代码对这些变量进行操作
            "ecall",//asm! 宏嵌入 ecall 指令来触发系统调用，
            inlateout("x10") args[0] => ret,//比较特殊的是 a0 寄存器，它同时作为输入和输出，因此我们将 in 改成 inlateout
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret//变量 ret 保存系统调用返回值，它也是函数 syscall 的输出/返回值
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}
// sys_write 使用一个 &[u8] 切片类型来描述缓冲区，
//这是一个 胖指针 (Fat Pointer)，
//里面既包含缓冲区的起始地址，还 包含缓冲区的长度
//通过 as_ptr 和 len 方法取出它们并独立地作为实际的系统调用参数
pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}

//将上述两个系统调用在用户库 user_lib 中进一步封装，
//从而更加接近在 Linux 等平台的实际系统调用接口：
