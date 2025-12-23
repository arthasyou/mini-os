use crate::{
    serial_println,
    test_harness::{
        qemu::{QemuExitCode, exit_qemu},
        testable::Testable,
    },
};

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}
