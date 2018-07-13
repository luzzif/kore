# Kore

Kore is a minimal kernel written in Rust.

## Startup

In order to startup the kernel, be sure to have Rust installed
(you can find the instructions [here](https://www.rust-lang.org/en-US/install.html)).
The next step is intalling the `bootimage` crate. Since the kernel is in a super-early
development stage, it is currently not multiboot compliant. This tool simply ensures
that a bootloader gets appended to the kernel image when compiling with it.
The last step is installing `QEMU` (you can find the instructions
[here](https://www.qemu.org/download/)), a VM through which
the kernel will be run. That's it, compile and run the kernel by executing
the `bootimage run` command from the project's root.
