SET RUST_BACKTRACE=1
xargo rustc --target aarch64-raspi3-none-elf --release --verbose -- --emit=obj --emit=llvm-ir