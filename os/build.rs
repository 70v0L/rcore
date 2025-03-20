use std::fs::{File, read_dir};
use std::io::{Result, Write};

/*自动生成 src/link_app.S 文件，该文件用于 链接所有用户态应用程序（即 user 目录下的 bin/ *.rs ）*/
//主要功能：
//监听 user 目录的变化，如果 user/src/bin/ 或 user/target/... 发生变动，重新运行 build.rs
//扫描 user/src/bin 目录，找到所有用户态应用的 .rs 文件。
/*按名字排序，并生成 src/link_app.S，其中包含：

应用数量 (_num_app)

所有应用程序的起始地址 (app_x_start) 和结束地址 (app_x_end)

.incbin 直接把 .bin 二进制文件嵌入到内核*/

//最终生成的 src/link_app.S 作为 汇编代码，
//在内核构建时被 链接进内核 ELF，从而让内核知道如何加载和运行用户态程序。

fn main() {
    println!("cargo:rerun-if-changed=../user/src/");
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
    insert_app_data().unwrap();
}

static TARGET_PATH: &str = "../user/target/riscv64gc-unknown-none-elf/release/";

fn insert_app_data() -> Result<()> {
    //生成一个新的 src/link_app.S 文件（汇编文件）
    let mut f = File::create("src/link_app.S").unwrap();
    //获取 user/src/bin 目录下的应用
    let mut apps: Vec<_> = read_dir("../user/src/bin")//读取 user/src/bin 目录中的所有文件，返回 Result<ReadDir>
        .unwrap()//解包 Result，如果目录不存在或读取失败，程序直接崩溃
        .into_iter()//将 ReadDir 迭代器转换为 Iterator<Item = Result<DirEntry>>，即逐个获取 DirEntry
        .map(|dir_entry| {
            let mut name_with_ext = dir_entry.unwrap().file_name().into_string().unwrap();
            name_with_ext.drain(name_with_ext.find('.').unwrap()..name_with_ext.len());
            name_with_ext
        })
        .collect();//处理后的文件名被 收集到 apps 这个 Vec<String> 中
    apps.sort();//排序，保证应用程序加载顺序固定

    //在 .data 段定义 _num_app，记录 用户态程序的数量,.quad 让 OS 知道有多少个应用
    writeln!(
        f,
        r#"
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad {}"#,
        apps.len()
    )?;

    //生成应用程序的地址表
    for i in 0..apps.len() {
        writeln!(f, r#"    .quad app_{}_start"#, i)?;
    }
    writeln!(f, r#"    .quad app_{}_end"#, apps.len() - 1)?;
    //app_x_start 是应用 x 的 起始地址，app_x_end 是 结束地址
    //因为 app_0_end == app_1_start，app_1_end == app_2_start，
    //所以 存储所有 app_X_end 没有意义，
    //只需记录 最后一个应用的 app_X_end 作为整个表的终止标志

    //生成 .incbin 指令，把应用二进制嵌入内核
    for (idx, app) in apps.iter().enumerate() {
        println!("app_{}: {}", idx, app);
        writeln!(
            f,
            r#"
    .section .data
    .global app_{0}_start
    .global app_{0}_end
app_{0}_start:
    .incbin "{2}{1}.bin"
app_{0}_end:"#,
            idx, app, TARGET_PATH
        )?;
    }//
    Ok(())
}



