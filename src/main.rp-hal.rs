//! Blinks the on-board LED of the Raspberry Pi Pico{% if chip == "rp2350" %} 2{% endif %}.

#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use panic_probe as _;
{%- if chip == "rp2040" %}
use rp2040_hal as hal;
{%- else %}
use rp235x_hal as hal;
{%- endif %}
{% if chip == "rp2040" %}
/// Second-stage bootloader for the Pico's W25Q080 flash chip.
#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
{%- else %}
/// Image definition telling the boot ROM how to start the program.
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();
{%- endif %}

/// Frequency of the Pico's crystal oscillator.
const XTAL_FREQ_HZ: u32 = 12_000_000;

#[hal::entry]
fn main() -> ! {
    let mut pac = hal::pac::Peripherals::take().expect("peripherals are taken only once");
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .expect("failed to initialize clocks");
    {%- if chip == "rp2040" %}
    let mut timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    {%- else %}
    let mut timer = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);
    {%- endif %}

    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let mut led = pins.gpio25.into_push_pull_output();

    loop {
        info!("LED on");
        led.set_high().expect("GPIO writes are infallible");
        timer.delay_ms(500);

        info!("LED off");
        led.set_low().expect("GPIO writes are infallible");
        timer.delay_ms(500);
    }
}
