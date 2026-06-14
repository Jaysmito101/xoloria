#![no_std]
#![no_main]

use core::arch::naked_asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn _start() -> ! {
    naked_asm!(
        r#"
    1:
        ecall
        j 1b
        "#
    );
}
