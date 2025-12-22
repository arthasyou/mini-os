#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // println!("Hello World 中文 hey{}", "!");
    // println!("looking out{}", "!");

    let p = 0xb8000 as *mut u16;

    for _ in 0..1_000_000 {
        unsafe {
            *p = 0x0f41; // 白色 'A'
        }
    }

    println!("Hello World 中文 hey{}", "!");

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
