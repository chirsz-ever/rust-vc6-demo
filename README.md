# link Rust program with VC6.0's Link.exe

## 构建步骤

1. 安装 rustup; 安装某种 host toolchain; 安装 `i686-pc-windows-msvc` target: `rustup target install i686-pc-windows-msvc`.
2. 准备好 VC6.0 命令行环境. 安装 VC6.0 本体和 [Windows Server 2003 R2 Platform SDK](https://download.cnet.com/Windows-Server-2003-R2-Platform-SDK-ISO-Download/3000-10248_4-10731094.html), 在 CMD 中执行 VC6.0 安装目录下的 `VC98\Bin\VC\VCVARS32.bat`.
3. 执行 `cargo build` 编译, `cargo run` 运行

## Acknowledgement

[src/main.rs](src/main.rs) 来自 [TRPL 第二章](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).

[3rdparty/YY_Thunks_for_WinXP.obj](3rdparty/YY_Thunks_for_WinXP.obj) 来自 [YY-Truncks v1.0.7-Beta4](https://github.com/Chuyu-Team/YY-Thunks/releases/tag/v1.0.7-Beta4)
