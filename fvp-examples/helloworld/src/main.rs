#![no_std]
#![no_main]
#![feature(strict_provenance)]

use aarch64_purecap_rt::entry;
use cheri::{prelude::*, ptr::Perms};

#[link_section = ".el2_entry"]
#[used]
#[no_mangle]
pub static EL2_ENTRY_BIN: [u8; include_bytes!(env!("EL2_ENTRY_BIN")).len()] =
    *include_bytes!(env!("EL2_ENTRY_BIN"));

#[entry]
fn main() -> ! {
    let mut uart = unsafe { pl011_uart::Pl011Uart::from_address(0x2A40_0000) }.unwrap();
    uart.write_bytes(b"Hello, world!\r\n");

    loop {}
}

// /// Write a slice of bytes to the PL011 UART
// fn write_str(data: &[u8]) {
//     const UART_PL011_DATA_REGISTER: usize = 0x2A40_0000;
//
//     let ddc: *mut u32 = cheri::ptr::default_data_mut();
//     let uart_dr = ddc
//         .with_addr(UART_PL011_DATA_REGISTER)
//         .with_perms_clear_except(Perms::LOAD | Perms::STORE)
//         .with_bounds(4);
//
//     for byte in data {
//         // SAFETY: The ptr to the UART PL011 data register is valid for writes and properly aligned
//         unsafe {
//             core::ptr::write_volatile(uart_dr, *byte as u32);
//         }
//     }
// }

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
