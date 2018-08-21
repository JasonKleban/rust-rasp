SET RUST_BACKTRACE=1
xargo rustc --target aarch64-raspi3-none-elf --release --verbose -- --emit=obj --emit=llvm-ir
SET base="./target/aarch64-raspi3-none-elf/release/deps/rust_rasp-50deeb0d4981ffa7"
aarch64-elf-nm %base%.o
aarch64-elf-ld -O0 -mfpu=vfp -maarch64elf -nostartfiles %base%.o -o %base%.elf
aarch64-elf-objcopy %base%.elf -O binary ./out/kernel.img