# Stepper Motor Controller (28BYJ-48)

Firmware for controlling a 28BYJ-48 stepper motor using the ULN2003 driver board. Implements 8-step half-stepping sequence for smooth rotation. Demonstrates GPIO control and precise timing in Rust.

## Features

- Half-stepping mode (8 steps per revolution segment)
- Smooth motor rotation with precise timing
- Direct GPIO control of stepper driver
- Low-level Rust firmware (no_std)
- Continuous rotation loop

## Hardware

- **Board:** YD-RP2040
- **Microcontroller:** RP2040 (Dual ARM Cortex-M0+ @ 125 MHz)
- **Components:**
  - 28BYJ-48 Stepper Motor (5V, unipolar)
  - ULN2003 Stepper Motor Driver Board
  - Repurposed phone charger for motor power supply

## Power Supply

- **Microcontroller (YD-RP2040):** Powered via USB Type-C from laptop
- **Stepper Motor:** Powered by repurposed phone charger
  - Output Voltage: - V
  - Output Current: - A
  - Power: - W

**Important:** Motor and microcontroller share common GND but have separate power sources.

## Wiring Diagram

| ULN2003 Driver | YD-RP2040 Pin | Notes              |
|----------------|---------------|--------------------|
| IN1            | GPIO0         | Coil phase 1       |
| IN2            | GPIO1         | Coil phase 2       |
| IN3            | GPIO2         | Coil phase 3       |
| IN4            | GPIO3         | Coil phase 4       |
| VCC (+)        | Charger (+)   | Motor power (5V from charger) |
| GND (-)        | Charger (-) + YD-RP2040 GND | Shared ground |

**Motor Connection:**
- Connect 28BYJ-48 motor to ULN2003 driver board via 5-pin connector
- Connect ULN2003 VCC to phone charger positive output
- **CRITICAL:** Connect ULN2003 GND to both charger negative AND YD-RP2040 GND (shared ground)
- YD-RP2040 powered separately via USB Type-C

## Stepper Sequence

Half-stepping sequence (8 steps):

| Step | IN1 | IN2 | IN3 | IN4 |
|------|-----|-----|-----|-----|
| 0    | 1   | 0   | 0   | 0   |
| 1    | 1   | 1   | 0   | 0   |
| 2    | 0   | 1   | 0   | 0   |
| 3    | 0   | 1   | 1   | 0   |
| 4    | 0   | 0   | 1   | 0   |
| 5    | 0   | 0   | 1   | 1   |
| 6    | 0   | 0   | 0   | 1   |
| 7    | 1   | 0   | 0   | 1   |

- **Step delay:** 1ms per step
- **Direction:** Clockwise (reverse sequence for counter-clockwise)

## How to Build

1. Install Rust and the ARM Cortex-M0+ target:
```bash
rustup target add thumbv6m-none-eabi
```

2. Build the project:
```bash
cargo build --release
```

3. Convert ELF to UF2:
```bash
cargo install elf2uf2-rs
elf2uf2-rs target/thumbv6m-none-eabi/release/hello_rust hello_rust.uf2
```

## How to Flash

1. Hold **BOOT** button on YD-RP2040
2. Press **RESET** button (or connect USB while holding BOOT)
3. Board appears as USB drive **RPI-RP2**
4. Copy **hello_rust.uf2** to the drive
5. Board will reboot automatically and motor will start rotating

## Built With

**Core Dependencies:**
- **cortex-m** v0.7.7 - Low-level ARM Cortex-M primitives and delay
- **cortex-m-rt** v0.7.5 - Startup code and runtime
- **panic-halt** v1.0.0 - Panic handler
- **rp2040-hal** v0.11.0 - Hardware abstraction layer
- **vcc-gnd-yd-rp2040** v0.6.0 - Board support package
- **embedded-hal** v1.0.0 - Hardware abstraction traits

## How it Works

1. **GPIO Initialization:** Configures GPIO0-GPIO3 as push-pull outputs for motor control
2. **Sequence Definition:** 8-step half-stepping pattern stored in array
3. **Main Loop:**
   - Iterates through all 8 steps in sequence
   - Sets GPIO states according to step pattern
   - Waits 1ms between steps for motor response
   - Repeats infinitely for continuous rotation

**Half-stepping provides:**
- Smoother motion compared to full-stepping
- Better torque at low speeds
- Finer position control (4096 steps per revolution)

## Motor Specifications (28BYJ-48)

- **Type:** Unipolar stepper motor
- **Voltage:** 5V DC
- **Steps per revolution:** 4096 (with 64:1 gear ratio)
- **Step angle:** 5.625° / 64 (full step)
- **Frequency:** Max 1000 Hz (1ms delay = 1000 steps/sec)

## Speed Control

To change motor speed, adjust the delay value:

```rust
delay.delay_ms(1);  // Fast (current)
delay.delay_ms(2);  // Medium
delay.delay_ms(5);  // Slow
```

**Formula:** RPM = (60 × 1000) / (delay_ms × 4096)

## Direction Control

To reverse rotation direction, iterate sequence backwards:

```rust
for step in sequence.iter().rev() {
    // Apply reversed sequence
}
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Motor doesn't move | Check charger output (5V), verify all 4 GPIO connections, ensure shared GND |
| Motor vibrates/hums | Reduce speed (increase delay), check wiring order |
| Motor skips steps | Increase step delay, verify charger can supply enough current |
| Motor overheats | Add cooling, reduce duty cycle or speed, check charger voltage |
| Erratic behavior | Verify common ground between charger and YD-RP2040 |

## Project Structure
stepper_motor/
Cargo.toml          # Project dependencies
Cargo.lock          # Locked dependency versions
build.rs            # Build script
memory.x            # Linker script
README.md           # This file
src/
main.rs         # Firmware source code

## Learning Points

This project demonstrates:
- GPIO output control for motor driving
- Stepper motor sequencing and timing
- Half-stepping vs full-stepping techniques
- Embedded timing with cortex_m::delay
- State machine implementation for motor control
- Separate power domains with shared ground

## Safety Notes

- **Do not** connect charger output directly to YD-RP2040 - only to motor driver
- Verify charger output voltage (should be ~5V for 28BYJ-48)
- **Always** connect shared GND between charger and microcontroller
- Ensure proper current limiting (ULN2003 handles this)
- Motor may get warm during continuous operation
- Use insulated connections to prevent shorts

## License

MIT
