




suggested build command: `xargo build --release --target aarch64-raspi3-none-elf --verbose`

or `./build.bat` for the lazy

but also maybe `xargo rustc --target aarch64-raspi3-none-elf -- --emit=obj --release --verbose`

#my notes
```
rustup override set nightly
rustc --version
git checkout #
git submodule update --init src/stdsimd
mkdir -p C:/Users/Jason/rustlib/arm-none-eabi/lib
rustc --crate-type=staticlib --target arm-none-eabi -O -Z no-landing-pads src/libcore/lib.rs --out-dir C:/Users/Jason/rustlib/arm-none-eabi/lib
```