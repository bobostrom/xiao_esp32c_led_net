# Walkthrough - Rust ESP32C3 LED Network

## Prerequisites
- Rust and Cargo installed.
- `espup` installed (`cargo install espup`).
- `ldproxy` installed (`cargo install ldproxy`).
- Python 3 for the test script.

## 1. Configuration
1.  Open `src/main.rs`.
2.  Update `SSID` and `PASSWORD` with your WiFi credentials.
3.  Update `DEVICE_ID` if this is not the lowest order digit.
4.  Update `LEDS_PER_DIGIT` to match your hardware.

## 2. Build and Flash
1.  Connect your XIAO ESP32C3 via USB.
2.  Run the following command to build and flash:
    ```bash
    cargo run
    ```
    *Note: The first build will take a while as it compiles the ESP-IDF SDK.*

## 3. Testing
1.  Once flashed, the device logs will show "WiFi connected!" and the IP address (you might need to check your router if logs don't show IP explicitly, or add log for it).
2.  Open `test_sender.py`.
3.  Update `UDP_IP` with the IP address of your ESP32C3.
4.  Run the script:
    ```bash
    python test_sender.py
    ```
5.  You should see the LEDs lighting up with a rainbow pattern.

## Troubleshooting
- **Build Errors**: Ensure you have the correct target installed: `rustup target add riscv32imc-esp-espidf`.
- **WiFi Connection**: Double-check SSID and Password. Ensure 2.4GHz WiFi is available.
