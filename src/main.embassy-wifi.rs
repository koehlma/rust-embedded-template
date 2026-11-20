//! Blinks the on-board LED of the Raspberry Pi Pico{% if chip == "rp2350" %} 2{% endif %} W.
//!
//! The LED is connected to the CYW43439 Wi-Fi chip, so blinking it requires the Wi-Fi driver.

#![no_std]
#![no_main]

use cyw43::aligned_bytes;
{%- if chip == "rp2040" %}
use cyw43_pio::{DEFAULT_CLOCK_DIVIDER, PioSpi};
{%- else %}
use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};
{%- endif %}
use defmt::{info, unwrap};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{self, Pio};
use embassy_rp::{bind_interrupts, dma};
use embassy_time::Timer;
use panic_probe as _;
use static_cell::StaticCell;

/// GPIO of the Wi-Fi chip that drives the LED.
const LED_GPIO: u8 = 0;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => pio::InterruptHandler<PIO0>;
    DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let firmware = aligned_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = aligned_bytes!("../cyw43-firmware/43439A0_clm.bin");
    let nvram = aligned_bytes!("../cyw43-firmware/nvram_rp2040.bin");

    let power = Output::new(p.PIN_23, Level::Low);
    let chip_select = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        {%- if chip == "rp2040" %}
        DEFAULT_CLOCK_DIVIDER,
        {%- else %}
        // The Pico 2 W's RM2 module does not work at the default clock speed.
        RM2_CLOCK_DIVIDER,
        {%- endif %}
        pio.irq0,
        chip_select,
        p.PIN_24,
        p.PIN_29,
        dma::Channel::new(p.DMA_CH0, Irqs),
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (_net_device, mut control, runner) = cyw43::new(state, power, spi, firmware, nvram).await;
    spawner.spawn(unwrap!(cyw43_task(runner)));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    loop {
        info!("LED on");
        control.gpio_set(LED_GPIO, true).await;
        Timer::after_millis(500).await;

        info!("LED off");
        control.gpio_set(LED_GPIO, false).await;
        Timer::after_millis(500).await;
    }
}

/// Runs the Wi-Fi driver.
#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, cyw43::SpiBus<Output<'static>, PioSpi<'static, PIO0, 0>>>,
) -> ! {
    runner.run().await
}
