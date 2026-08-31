# `alloc-morello-qemu`

Heap allocation example for the CHERI Morello Qemu using the aarch64-purecap-rt runtime. The entry macro grants a heap arena (`Heap<SIZE>`) next to the UART MMIO region, and the `bump-alloc` feature of the runtime provides a `BumpAllocator` that is installed as the `#[global_allocator]` and initialized from the grant. The program reads a line from the UART into a heap-allocated `Vec`, echoes it back with `alloc::format!`, and exits. Reading runtime input makes sure the allocations are not optimized out.

With `-nographic` the terminal is connected to the UART, so run it and type a line, or pipe one in:

```bash
printf 'hello\r' | crability cargo run
```
