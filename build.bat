SET RUST_BACKTRACE=1
xargo rustc --target arm-raspi3-none-elf --release --verbose -- --emit=obj