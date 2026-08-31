# `alloc-morello-fvp`

Heap allocation example for the Morello FVP using the aarch64-purecap-rt runtime. The entry macro grants a heap arena (`Heap<SIZE>`) next to the AP UART MMIO region, and the `bump-alloc` feature of the runtime provides a `BumpAllocator` that is installed as the `#[global_allocator]` and initialized from the grant. The program loops as an echo server: it reads a line from the UART into a heap-allocated `Vec` and echoes it back with `alloc::format!`. Reading runtime input makes sure the allocations are not optimized out.

Build and run it like the other FVP examples, then talk to it over the `terminal_uart_ap` telnet port:

```bash
crability cargo xtask fip alloc
crability cargo xtask fvp alloc
telnet localhost <terminal_uart_ap port>
```

Note: the bump allocator never frees, so every echoed line leaks a little of the arena. When it runs out the allocation error handler panics.
