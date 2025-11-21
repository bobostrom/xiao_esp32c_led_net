use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::rmt::config::TransmitConfig;
use esp_idf_hal::rmt::TxRmtDriver;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::hal::prelude::FromValueType;

use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

use log::*;

const SSID: &str = "YOUR_SSID";
const PASSWORD: &str = "YOUR_PASSWORD";
const PORT: u16 = 7777;
const LED_PIN: u32 = 2; // D0 on XIAO ESP32C3
const DEVICE_ID: usize = 0; // 0 = Lowest order digit
const LEDS_PER_DIGIT: usize = 7; // Example: 7 segments per digit (or more for WS2812 strips)

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Starting ESP32C3 LED Network Node. Device ID: {}", DEVICE_ID);

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    // --- WiFi Setup ---
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    wifi.set_configuration(&esp_idf_svc::wifi::Configuration::Client(
        esp_idf_svc::wifi::ClientConfiguration {
            ssid: SSID.try_into().unwrap(),
            password: PASSWORD.try_into().unwrap(),
            ..Default::default()
        },
    ))?;

    wifi.start()?;
    wifi.connect()?;
    wifi.wait_netif_up()?;

    info!("WiFi connected!");

    // --- LED Setup ---
    // Using GPIO 2 (D0 on XIAO)
    let led_pin = peripherals.pins.gpio2;
    let channel = peripherals.rmt.channel0;
    let config = TransmitConfig::new().clock_divider(2);
    let tx = TxRmtDriver::new(channel, led_pin, &config)?;
    
    let mut ws2812 = Ws2812Esp32Rmt::new(tx, ws2812_esp32_rmt_driver::LedPixelOrder::GRB).unwrap();

    // --- UDP Setup ---
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", PORT))?;
    info!("Listening on UDP port {}", PORT);

    let mut buf = [0u8; 1024];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, _src)) => {
                // info!("Received {} bytes from {}", amt, src);
                let data = &buf[..amt];
                
                // Packet Format: [R1, G1, B1, R2, G2, B2, ...] representing the whole string of digits
                // We need to extract the chunk relevant to this DEVICE_ID
                
                let start_led_index = DEVICE_ID * LEDS_PER_DIGIT;
                let start_byte_index = start_led_index * 3;
                let end_byte_index = start_byte_index + (LEDS_PER_DIGIT * 3);

                if data.len() >= end_byte_index {
                    let my_data = &data[start_byte_index..end_byte_index];
                    
                    let pixels: Vec<RGB8> = my_data.chunks(3)
                        .map(|chunk| {
                            if chunk.len() == 3 {
                                RGB8::new(chunk[0], chunk[1], chunk[2])
                            } else {
                                RGB8::default()
                            }
                        })
                        .collect();

                    ws2812.write(pixels.iter().cloned()).ok();
                } else {
                    // warn!("Packet too short: {} bytes, needed {}", data.len(), end_byte_index);
                }
            }
            Err(e) => {
                error!("Error receiving UDP: {}", e);
            }
        }
    }
}
