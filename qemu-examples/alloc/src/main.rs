#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::ptr::NonNull;

use aarch64_purecap_rt::alloc::BumpAllocator;
use aarch64_purecap_rt::entry;
use pl011_uart::registers::Pl011Registers;
use pl011_uart::Pl011Uart;

const HEAP_SIZE: usize = 0x10_0000;

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator::new();

#[entry(grant(
    heap: Heap<HEAP_SIZE>,
    uart: Mmio<0x0900_0000, Pl011Registers>,
))]
fn main(grant: Grant) -> ! {
    // SAFETY: called once, before the first allocation, with the heap grant
    unsafe { ALLOCATOR.init(grant.heap, HEAP_SIZE) };

    // SAFETY: the granted capability points to the PL011 register block of the QEMU
    // virt machine
    let mut uart = unsafe { Pl011Uart::new(NonNull::new(grant.uart).unwrap()) }.unwrap();

    uart.write_bytes(b"type a line: ");

    // The line lives on the granted heap and grows with runtime input, so the
    // allocations can't be optimized out
    let mut line: Vec<u8> = Vec::new();
    loop {
        let byte = uart.read_byte_blocking();
        if byte == b'\r' || byte == b'\n' {
            break;
        }
        line.push(byte);
    }

    let line = String::from_utf8_lossy(&line);
    let message = alloc::format!("\r\nyou typed: {} ({} bytes)\r\n", line, line.len());
    uart.write_bytes(message.as_bytes());

    alloc_morello_qemu::qemu_exit(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    alloc_morello_qemu::qemu_exit(1)
}
