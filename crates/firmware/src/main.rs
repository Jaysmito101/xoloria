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
        la sp, __stack_top
        auipc ra, 0x0
        ecall
        call runtime_init
        "#
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn runtime_init() -> ! {
    loop {}
}
