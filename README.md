# Vladutu Cristian-Vlad

## Development setup

### Arch Linux

> Considering you have yay installed

```bash
yay -S pico-sdk
yay -S picotool
```

```bash
#rustup target add thumbv8m.main-none-eabihf # ARM Cortex M33 (optional, not used)
rustup target add riscv32imac-unknown-none-elf # RISK-V hype is real!
```

## Build and flash

build

```bash
cargo build --example pwm_blink --target riscv32imac-unknown-none-elf --all-features
cargo build --example pwm_blink --target thumbv8m.main-none-eabihf --all-features
```

Hold the BOOTSEL button while connecting the Pico2W with the microUSB

flash
```bash
#picotool load -t elf ./target/thumbv8m.main-none-eabihf/debug/pico-id
picotool load -t elf ./target/riscv32imac-unknown-none-elf/debug/pico-id
```

reboot
```bash
picotool reboot
```


## (Optionals)

### Create the uf2 binary (used for drag and drop to the Pico2W)

```bash
picotool uf2 convert ./target/riscv32imac-unknown-none-elf/debug/pico-id -t elf ./pwm_blink.uf2
```

### Pico information

```bash
picotool info -d
```
