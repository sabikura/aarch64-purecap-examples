# aarch64-purecap-examples

Examples that use the [`aarch64-purecap-rt`](https://github.com/sabikura/aarch64-purecap)
Rust runtime for the Morello platform.

- `fvp-examples/` boot on the Morello FVP as the BL33 payload of a TF-A firmware
  image
- `qemu-examples/` boot on CHERI QEMU's `virt` machine

## Toolchain

The Morello support is not upstreamed, therefore the examples should be built with the CHERI Rust
fork and the Morello LLVM SDK.

This relies on the [crability](https://github.com/sabikura/crability) tool being used to install
the prerequisites and wrap the custom `cargo`.

## Running the examples

In order to run the Morello FVP examples, you need to first run the `setup` command that pulls the submodules and builds the
TF-A:

```bash
crability cargo xtask setup # first step, should be done only once
crability cargo xtask fip <fvp-example> # builds the example and generates the fip.bin blob
crability cargo xtask fvp <fvp-example> [--clean] # run the example using the Morello FVP
```

After running the FVP example, to see the output you can listen with `telnet` on the port it opened for `terminal_uart_ap` (usually it's 5003).

(I will integrate this into the xtask at some point :/)

```bash
telnet localhost <terminal_uart_ap port>
```

In order to run the QEMU examples, simply go to the example crate and run it:

```bash
cd qemu-examples/<qemu-example>
crability cargo run
```

