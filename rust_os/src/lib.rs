#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod serial;
pub mod vga_buffer;

use core::panic::PanicInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("[\x1b[32mpass\x1b[0m]\t");
        serial_println!("{}", core::any::type_name::<T>());
        self();
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    if tests.len() >= 2 {
        serial_println!("\nRunning {} tests:\n", tests.len());

        for test in tests {
            test.run();
        }
    } else if tests.len() == 1 {
        serial_println!("\nRunning {} test:\n", tests.len());

        for test in tests {
            test.run();
        }
    } else {
        serial_println!("\nRunning no tests");
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[\x1b[31mfailed\x1b[0m]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
