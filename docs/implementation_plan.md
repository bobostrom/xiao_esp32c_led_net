# Implementation Plan - Rust ESP32C3 LED Network

The goal is to create a Rust firmware for the Seeed Studio XIAO ESP32C3 that controls a string of WS2812B LEDs.
**Use Case:** A stopwatch application where the lowest digit updates rapidly.
**Protocol:** UDP is chosen for its low latency and ability to handle high frame rates (e.g., 30-60 FPS) required for smooth digit transitions. TCP retransmission delays would cause visible stuttering.

## User Review Required

> [!IMPORTANT]
> This plan assumes using the `std` framework (`esp-idf-svc`) for easier WiFi and networking implementation. If `no_std` is preferred, please let me know.

> [!NOTE]
> I will use UDP for the network communication as it is suitable for real-time lighting control (low latency, packet loss acceptable).

## Proposed Changes

### Project Initialization
#### [NEW] [Cargo.toml](file:///c:/Projects/xiao_esp32c_led_net/Cargo.toml)
- Initialize a new project using `esp-idf-template` (or manually set up if template tool is not available).
- Add dependencies:
    - `esp-idf-svc` (WiFi, System)
    - `esp-idf-hal` (Hardware Abstraction)
    - `smart-leds` (LED traits)
    - `ws2812-esp32-rmt-driver` (RMT driver for LEDs)
    - `log` (Logging)

### Application Logic
#### [NEW] [src/main.rs](file:///c:/Projects/xiao_esp32c_led_net/src/main.rs)
- **WiFi Setup**: Connect to a hardcoded SSID/Password (or provisioned).
- **Configuration**:
    - `DEVICE_ID`: The logical address of this device (0 = Lowest Order Digit / Fastest changing).
    - `LEDS_PER_DIGIT`: Number of LEDs this device controls.
- **UDP Socket**: Bind to a specific port (e.g., 7777).
- **LED Driver**: Initialize RMT driver.
- **Loop**:
    - Receive UDP packet.
    - **Filtering**: Calculate offset based on `DEVICE_ID` * `LEDS_PER_DIGIT`.
    - Extract relevant LED data for this device.
    - Update LEDs.

## Verification Plan

### Automated Tests
- Since this is embedded hardware, on-device automated testing is limited without a HIL setup.
- I will verify compilation `cargo build`.

### Manual Verification
- **Build**: Run `cargo build` to ensure dependencies and code are correct.
- **Flash**: (User action) `cargo run` to flash to the device.
- **Network Test**: Send UDP packets from the host computer using `netcat` or a python script to verify LED changes.
    - I will provide a python script `test_sender.py` to send color patterns.
