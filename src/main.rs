#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod qemu;
mod serial;
mod vga_buffer;
mod volatile;

use core::panic::PanicInfo;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    use crate::qemu::{QemuExitCode, exit_qemu};

    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World 中文 hey{}", "!");
    // panic!("Some panic message");

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[test_case]
fn trivial_assertion() {
    serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}
