#![no_std]
#![no_main]

pub mod memory;
pub mod scheduler;
pub mod syscalls;
pub mod ipc;
pub mod isolation;
pub mod temporal;
pub mod axiarchy;

use core::panic::PanicInfo;

// Entry point from bootloader
#[no_mangle]
pub extern "C" fn kmain() -> ! {
    // 1. Initialize Memory Manager
    memory::init();

    // 2. Initialize IPC with Kyber-1024
    ipc::init();

    // 3. Initialize Kernel Isolation Engine
    isolation::init();

    // 4. Initialize TemporalChain Integration
    temporal::init();

    // 5. Initialize Scheduler with Theosis metric
    scheduler::init();

    // Start scheduling tasks
    scheduler::start();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
