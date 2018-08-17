Trying to build a kernel for bare-metal raspberry pi 3 from Windows.

# To attempt

1. Install Rust
1. `cargo install xargo`
1. ...

suggested build command: `xargo build --release --target aarch64-raspi3-none-elf --verbose`

or `./build.bat` for the lazy

but also maybe `xargo rustc --target aarch64-raspi3-none-elf -- --emit=obj --release --verbose`

But either way I get:

```
C:\Users\Jason\Repos\rust-rasp>xargo build --release --target aarch64-raspi3-none-elf --verbose
+ "rustc" "--print" "sysroot"
+ "rustc" "--print" "target-list"
+ RUSTFLAGS="-C emit=obj --sysroot C:\\Users\\Jason\\.xargo -Z force-unstable-if-unmarked"
+ "cargo" "build" "--release" "--manifest-path" "C:\\Users\\Jason\\AppData\\Local\\Temp\\xargo.slcTUhRFKyvT\\Cargo.toml" "--target" "aarch64-raspi3-none-elf" "-v" "-p" "core"
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `rustc - --crate-name ___ --print=file-names -C emit=obj --sysroot
C:\Users\Jason\.xargo -Z force-unstable-if-unmarked --target aarch64-raspi3-none-elf --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro` (exit code: 1)
--- stderr
error: unknown codegen option: `emit`
```

# notes on building libcore - outdated now with xargo
```
rustup override set nightly
rustc --version
git checkout #
git submodule update --init src/stdsimd
mkdir -p C:/Users/Jason/rustlib/arm-none-eabi/lib
rustc --crate-type=staticlib --target arm-none-eabi -O -Z no-landing-pads src/libcore/lib.rs --out-dir C:/Users/Jason/rustlib/arm-none-eabi/lib
```
