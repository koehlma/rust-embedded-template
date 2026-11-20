# Rust Embedded Template

A [cargo-generate](https://cargo-generate.github.io/cargo-generate/) template for Rust firmware on the Raspberry Pi Pico family. A generated project blinks the board's on-board LED and logs each change over a debug probe. It includes the dependencies, linker script, and Cargo configuration for the selected board, so you can replace the blinky code with your application.

## Choosing a Variant

The template asks up to three questions. The first two follow from your board:

| Board       | Chip   | Wi-Fi |
| ----------- | ------ | ----- |
| Pico        | RP2040 | No    |
| Pico W      | RP2040 | Yes   |
| Pico 2      | RP2350 | No    |
| Pico 2 W    | RP2350 | Yes   |

The third question selects the framework. With [Embassy](https://embassy.dev), the program runs as async tasks that yield with `.await` while they wait for timers or peripherals, so several activities can run concurrently without threads. With [rp-hal](https://github.com/rp-rs/rp-hal), `main` is a synchronous loop with blocking delays, which is easier to follow for programs that do one thing at a time.

On the Pico W and Pico 2 W, the LED is wired to the CYW43439 Wi-Fi chip rather than to the microcontroller, so even blinking it requires the Wi-Fi driver. The Rust driver for this chip, [`cyw43`](https://crates.io/crates/cyw43), is part of Embassy and works only with async code. Answering yes to the Wi-Fi question therefore selects Embassy without asking. The generated program starts the driver to blink the LED but does not connect to a network. For networking, add a network stack such as [`embassy-net`](https://crates.io/crates/embassy-net).

## Generating a Project

Install Rust through [rustup](https://rustup.rs), then install cargo-generate and run it against this repository:

```sh
cargo install cargo-generate
cargo generate gh:silitics/rust-embedded-template --name blinky
```

To skip the questions, for example in scripts, define the answers on the command line:

```sh
cargo generate gh:silitics/rust-embedded-template --name blinky \
    --define chip=rp2350 --define wifi=false --define embassy=true
```

The `chip` placeholder accepts `rp2040` or `rp2350`. The `wifi` and `embassy` placeholders accept `true` or `false`.

## Building and Flashing

The generated `rust-toolchain.toml` makes rustup install the stable toolchain and the compilation target for your chip on the first build. The `.cargo/config.toml` file selects this target, so a plain `cargo build` in the project directory produces firmware for the board:

```sh
cd blinky
cargo build --release
```

Build with `--release`. Debug builds are much larger and slower, which matters for the Wi-Fi driver in particular.

To flash and run the firmware, connect a debug probe, such as the [Raspberry Pi Debug Probe](https://www.raspberrypi.com/products/debug-probe/), to the board's SWD pins and install [probe-rs](https://probe.rs). `cargo run` then flashes the firmware and prints its log messages:

```sh
cargo run --release
```

The log messages use [defmt](https://defmt.ferrous-systems.com), a compact logging format that the probe transfers from the chip's memory. The `DEFMT_LOG` variable in `.cargo/config.toml` sets the log level.

Without a debug probe, you can flash over USB with [picotool](https://github.com/raspberrypi/picotool). Hold the board's BOOTSEL button while plugging it in, switch the `runner` in `.cargo/config.toml` to the commented picotool line, and run `cargo run --release` again. Log messages are not available this way.

## Project Layout

- `src/main.rs` contains the blinky program.
- `Cargo.toml` declares the dependencies of the selected variant.
- `memory.x` describes the flash and RAM of the board and the sections that the boot ROM expects. The RP2040 boot ROM loads a 256-byte second-stage bootloader from the start of flash, which configures the flash chip. The RP2350 boot ROM instead looks for an image definition block in the first 4 KiB of flash. Adjust the flash size if your board differs from the Raspberry Pi boards.
- `build.rs` puts `memory.x` on the linker search path and passes the linker scripts of `cortex-m-rt` and `defmt`.
- `.cargo/config.toml` sets the compilation target, the runner for `cargo run`, and the log level.
- `cyw43-firmware/`, present only in Wi-Fi projects, holds the firmware that the driver uploads to the Wi-Fi chip at startup.

## Developing the Template

The template files are at the root of this repository. cargo-generate renders them with [Liquid](https://shopify.github.io/liquid/) using the `chip`, `wifi`, and `embassy` placeholders defined in `cargo-generate.toml`. Files that Cargo or rustup would otherwise read inside this repository carry a `.liquid` suffix, which cargo-generate removes. Each programming model has its own entry point in `src/`. The hooks in `hooks/` set `embassy` for Wi-Fi projects and rename the selected entry point to `src/main.rs`.

The template has six variants: both chips with rp-hal, with Embassy, and with Embassy and Wi-Fi. To check a change, generate and build all of them from the repository root. The script needs cargo-generate, rustfmt, and Clippy:

```sh
scripts/check-variants.sh
```

The script runs `cargo fmt --check`, `cargo clippy` with warnings denied, and `cargo build --release` for each variant. CI runs it for pull requests and pushes to `main`, and weekly to catch breaking dependency releases.

## License

The template is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option. The Wi-Fi chip firmware in `cyw43-firmware/` comes from the [Embassy repository](https://github.com/embassy-rs/embassy/tree/main/cyw43-firmware) and is licensed under the [Infineon Permissive Binary License](cyw43-firmware/LICENSE-permissive-binary-license-1.0.txt).
