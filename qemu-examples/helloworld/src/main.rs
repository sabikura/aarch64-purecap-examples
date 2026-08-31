#![no_std]
#![no_main]

use core::ptr::NonNull;

use aarch64_purecap_rt::entry;
use pl011_uart::registers::Pl011Registers;
use pl011_uart::Pl011Uart;

#[entry(grant(
    uart: Mmio<0x0900_0000, Pl011Registers>,
))]
fn main(grant: Grant) -> ! {
    // SAFETY: the granted capability points to the PL011 register block of the QEMU
    // virt machine
    let mut uart = unsafe { Pl011Uart::new(NonNull::new(grant.uart).unwrap()) }.unwrap();
    uart.write_bytes(b"Hello, world!\r\n");

    helloworld_morello_qemu::qemu_exit(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    helloworld_morello_qemu::qemu_exit(1)
}
