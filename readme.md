Technique for building a kernel for bare-metal raspberry pi 3 from Windows.

# To build the .o file

1. Install Rust
1. `cargo install xargo`
1. Download and extract the (linaro aarch64 tools)[https://releases.linaro.org/components/toolchain/binaries/latest/aarch64-elf/].  Add its bin directory path to your PATH environment variable, restart vscode or whatever external shell you're using so the new PATH is picked up.
1. `./build.bat` a first time which might prompt for missing components like `rustup component add rust-src`.
1. `./build.bat` a second time which might error out that the target file isn't found.  Discover the random output filename xargo decided on for your configuration, edit `build.bat`'s `%base%` to match. You hopefully only need to do this once.
1. `./build.bat`
1. Add the completed `out/kernel.img` to the sd card along with the standard `start.elf` and `bootcode.bin` and run in your rpi3.
1. Modify kernel.rs and repeat from step 6.

# Note

The code doesn't actually work now to DO anything at all (for a plain raspberry pi 3 with nothing hooked to it anyway) - the ACT led blinker code is not valid for it - I think we gotta port https://github.com/vanvught/rpidmx512/blob/master/lib-bcm2835/src/bcm2837_gpio_virt.c since "Pi3 uses a GPIO expander to drive the LEDs which can only be accessed from the VPU."

From some unknown readme I see references to:
```
Info:   Pi3 uses a GPIO expander to drive the LEDs which can only be accessed
        from the VPU. There is a special driver for this with a separate DT
        node, which has the unfortunate consequence of breaking the
        act_led_gpio and act_led_activelow dtparams.
        This overlay changes the GPIO controller back to the standard one and
        restores the dtparams.
Load:   dtoverlay=pi3-act-led,<param>=<val>
Params: activelow               Set to "on" to invert the sense of the LED
                                (default "off")
```

Perhaps blinking the ACT led is not necessary, but it would be nice to verify that we succeeded in getting control of the board without requiring any additional equipment at all.