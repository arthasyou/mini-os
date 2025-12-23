use core::panic::PanicInfo;

use crate::{
    serial_println,
    test_harness::qemu::{QemuExitCode, exit_qemu},
};

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
}
