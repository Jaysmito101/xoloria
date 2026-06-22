#![no_std]
#![no_main]

use core::arch::naked_asm;
use core::panic::PanicInfo;

use buddy_system_allocator::LockedHeap;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        core::arch::asm!("ebreak");
    };
    loop {}
}

#[unsafe(no_mangle)]
#[unsafe(naked)]
#[unsafe(link_section = ".text.start")]
pub unsafe extern "C" fn _start() -> ! {
    naked_asm!(
        r#"
        la sp, __stack_top
        auipc ra, 0x0
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
    main(); //.expect("Failed to run main function");
    loop {}
}

pub fn fibo_recursive(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibo_recursive(n - 1) + fibo_recursive(n - 2)
    }
}

pub fn factorial_recursive(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial_recursive(n - 1)
    }
}

pub fn fibo_iterative(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    a
}

fn main() -> anyhow::Result<u32> {
    for j in 0..100000 {
        unsafe {
            core::arch::asm!("mv t0, {0}", in(reg) j);
        }
        for i in 0..12 {
            let fibo = fibo_recursive(i);
            let fact = factorial_recursive(i);
            let fibo_iter = fibo_iterative(i);
            unsafe {
                // leak the values to prevent optimization
                core::ptr::read_volatile(&fibo);
                core::ptr::read_volatile(&fact);
                core::ptr::read_volatile(&fibo_iter);
            };
        }
    }
    Err(anyhow::anyhow!("This is a test error"))?;
    Ok(42)
}
