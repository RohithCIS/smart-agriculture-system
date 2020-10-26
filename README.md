# SMART AGRICULTURE SYSTEM

This Embedded project was done as a part of ECE5042 course at VIT University to explore and learn the usage of Rust Lang on Embedded Platforms

### Authors
- ROHITH BALAJI [20MES0038]
- AJITH KUMAR R [20MES0052]
- SHRUTI GHADGE [20MES0050]

### Hardware
- STM32F103 [Blue Pill] based on ARM Cortex M3
- ST-LINKv2 Clone
- SSD1306 OLED Display
- DHT-11
- Capacitive Soli Moisture Sensor

### Software
- OpenOCD - Open On Chip Debugger
- GDB - GNU General Debugger for arm-none-eabi
- telnet (Instead of GDB. You don't need this if you are using GDB)

## Setup
1. Install Rustup Toolchain from https://rustup.rs/
2. Run `rustup --version` to verify installation
3. Run `rustup default nightly` to switch to nightly build of Rust
4. Verify Rust and Cargo with `rustc --version` and `cargo --version`
5. Install OpenOCD ( Download, Extract and Add to PATH from https://github.com/xpack-dev-tools/openocd-xpack/releases )
6. Install GDB ( Download, Extract and Add to PATH from https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads )

## Build
1. Ensure ARM Target is added with `rustup target add thumbv7m-none-eabi`
2. Run `cargo build` for debug build and `cargo build --release` for release build from the project root

## Run and Debug
1. Open a terminal in the project root and run `openocd -f openocd.cfg`
2. Open another terminal in the project root and run `arm-none-eabi-gdb -x loader.gdb`. This will start a debug session.
