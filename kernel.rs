// kernel.rs
#![feature(lang_items, asm)]
#![crate_type = "staticlib"]
#![no_std]

/**
 * This is the address of the General Purpose I/O Function Selector 2.
 * We need this one because we need to configure GPIO #29 perform the Output function.
 * GPIO #0..9 are configured by GPFSEL0 in three-bit,
 *      #10..19 are configured by GPFSEL1, 
 *      #20..29 are configured by GPFSEL2, 
 *      ... and so on up to GPFSEL5 for #50..54
 * (Yes, stopping at 54 because there are only 54 GPIO)
 * 
 * This literal is cast as a pointer to a u32 that we must 
 * mutate, the memory location mapped to the RPI's GPFSEL2 register.
 */
const GPFSEL2: *mut u32 = 0x7E20_0008 as *mut u32;

/** 
 * We need to change only the three bits 27, 28, and 29
 * of the GPFSEL2 memory location without disturbing the
 * rest of the bits there, so this mask will be useful
 * for both zeroing out those bits and then applying 
 * GPF_OUTPUT configuration only to that register
 */
const FSEL9_MASK: u32 = 111 << 27;

const GPF_OUTPUT: u32 = 0b00001001001001001001001001001001;

/** 
 * For GPIO pins configured as an Output, 
 * GPIO pins 0..31 are set HIGH by setting bits 0..31 on GPSET0 (?)
 * GPIO pins 32..54 are set HIGH by setting bits 0..22 on GPSET1 (?)
 * 
 * ? is it to 31 or are some unused?
 * 
 * But anyway, we're after GPIO pin 29, so we'll need GPSET0
 */
const GPSET0: *mut u32 = 0x7E20_001C as *mut u32;

/** 
 * For GPIO pins configured as an Output, 
 * GPIO pins 0..31 are set LOW by setting bits 0..31 on GPCLR0 (?)
 * GPIO pins 32..54 are set LOW by setting bits 0..22 on GPCLR1 (?)
 */
const GPCLR0: *mut u32 = 0x7E20_0028 as *mut u32;

// an easy delay used between pin updates to be able to perceive the 
// Activity LED switching on and off.
fn sleep(value: u32) {
    for _ in 1..value {
        // Apparently this will stop the compiler from optimizing away the loop
        // and skipping the delay we intend.
        unsafe { asm!(""); }
    }
}

/** `_start` is aarch64's `main`, I think */
#[no_mangle]
pub extern fn _start() {

    // unsafe because we're doing pointer manipulation
    unsafe {
        // &= the inverse of the mask to zero the bits
        *GPFSEL2 &= !FSEL9_MASK;
        // |= the desired configuration masked to the 
        // desired pin to switch those specific bits back on
        *GPFSEL2 |= GPF_OUTPUT & FSEL9_MASK;
        // I don't think that can be done in a single step
    };

    // forever ...
    loop {
        // SET the 29th bit of GPSET0 to turn the led on
        unsafe { *GPSET0 |= 1 << 29; }
        // count to 500,000 - however long that takes - with the led already on
        sleep(500_000);
        // SET the 29th bit of GPCLR0 to turn the led off
        unsafe { *GPCLR0 |= 1 << 29; }
        // count to 500,000 again with the led already off
        sleep(500_000);
        //... repeat
    }

    // I assume that setting GPSET0 or GPCLR0 are either 
    // immediately cleared by the hardware upon recognition 
    // of the change or have the effect of clearing the 
    // opposite register's bit so that a later SET can be detected?
    // and that we're not responsible for clearing them.
}