#![no_std]
#![no_main]

use core::arch::naked_asm;
use core::panic::PanicInfo;

use buddy_system_allocator::LockedHeap;

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

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

const HEAP_SIZE: usize = 1024 * 16; // 16 KB heap
static mut HEAP_MEMORY: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn runtime_init() -> ! {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(&raw mut HEAP_MEMORY as usize, HEAP_SIZE);
    }
    main().expect("Failed to run main function");
    loop {}
}

fn main() -> anyhow::Result<()> {
    Ok(())
}
