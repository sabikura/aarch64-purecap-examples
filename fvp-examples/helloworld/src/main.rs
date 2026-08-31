#![no_std]
#![no_main]

use core::ptr::NonNull;

use aarch64_purecap_rt::entry;
use pl011_uart::registers::Pl011Registers;
use pl011_uart::Pl011Uart;

#[link_section = ".el2_entry"]
#[used]
#[no_mangle]
pub static EL2_ENTRY_BIN: [u8; include_bytes!(env!("EL2_ENTRY_BIN")).len()] =
    *include_bytes!(env!("EL2_ENTRY_BIN"));

#[entry(grant(
    uart: Mmio<0x2A40_0000, Pl011Registers>,
))]
fn main(grant: Grant) -> ! {
    // SAFETY: the granted capability points to the AP UART register block of the
    // Morello FVP
    let mut uart = unsafe { Pl011Uart::new(NonNull::new(grant.uart).unwrap()) }.unwrap();
    uart.write_bytes(b"Hello, world!\r\n");

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
