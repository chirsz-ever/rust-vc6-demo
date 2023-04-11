# link Rust program with VC6.0's Link.exe

## 构建步骤

1. 安装 rustup; 安装某种 host toolchain; 安装 `i686-pc-windows-msvc` target: `rustup target install i686-pc-windows-msvc`.
2. [安装 pyinstaller](https://pyinstaller.org/en/stable/installation.html), 将 [link_wrapper.py](link_wrapper.py) 转换为可执行文件: `pyinstaller -F link_wrapper.py`.
3. 准备好 VC6.0 命令行环境. 安装 VC6.0 本体和 Windows 2003 February Platform SDK, 在 CMD 中执行安装目录下的 `VC98\Bin\VC\VCVARS32.bat`.
4. 执行 `cargo build` 编译, `cargo run` 运行

## Acknowledgement

[src/main.rs](src/main.rs) 来自 [TRPL 第二章](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html).

[3rdparty/YY_Thunks_for_WinXP.obj](3rdparty/YY_Thunks_for_WinXP.obj) 来自 [YY-Truncks v1.0.6](https://github.com/Chuyu-Team/YY-Thunks/releases/tag/v1.0.6)