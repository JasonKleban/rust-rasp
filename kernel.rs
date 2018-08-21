// kernel.rs
#![feature(lang_items, asm)]
#![crate_type = "staticlib"]
#![no_std]

// Intended to blink the ACT led on rpi3, but it won't because the sample code
// is for the rpi2. See readme.md

const GPIO_BASE: u32 = 0x20200000;

fn sleep(value: u32) {
    for _ in 1..value {
        unsafe { asm!(""); }
    }
}

#[no_mangle]
pub extern fn _start() {
    let gpio = GPIO_BASE as *const u32;
    let led_on = unsafe { gpio.offset(8) as *mut u32 };
    let led_off = unsafe { gpio.offset(11) as *mut u32 };

    loop {
        unsafe { *(led_on) = 1 << 15; }
        sleep(500000);
        unsafe { *(led_off) = 1 << 15; }
        sleep(500000);
    }
}