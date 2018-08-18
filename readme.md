Trying to build a kernel for bare-metal raspberry pi 3 from Windows.

# To build the .o file

1. Install Rust
1. `cargo install xargo`
1. install missing components like `rustup component add rust-src` as prompted by build attempts ...
1. `xargo rustc --target aarch64-raspi3-none-elf --release --verbose -- --emit=obj` or `./build.bat` for the lazy

# To link the .elf file

maybe requires `rustup override set nightly-x86_64-pc-windows-gnu`?

1. ...

# Generate the kernel.img file

1. ...

# notes on building libcore - outdated by xargo
```
rustup override set nightly
rustc --version
git checkout #
git submodule update --init src/stdsimd
mkdir -p C:/Users/Jason/rustlib/arm-none-eabi/lib
rustc --crate-type=staticlib --target arm-none-eabi -O -Z no-landing-pads src/libcore/lib.rs --out-dir C:/Users/Jason/rustlib/arm-none-eabi/lib
```
