#![no_std]
#![no_main]

use panic_halt as _;

use rp235x_hal as hal;

use embedded_hal::delay::DelayNs;
use embedded_hal::pwm::SetDutyCycle;

#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();
const LOW: u16 = 0;

const HIGH: u16 = 25000;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[hal::entry]
fn main() -> ! {
    let mut pac = hal::pac::Peripherals::take().unwrap();

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
    .ok()
    .unwrap();

    let sio = hal::Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut delay = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);

    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    let pwm = &mut pwm_slices.pwm5;
    pwm.set_ph_correct();
    pwm.enable();

    let channel = &mut pwm.channel_a;
    channel.output_to(pins.gpio26);

    loop {
        for i in LOW..=HIGH {
            delay.delay_us(8);
            let _ = channel.set_duty_cycle(i);
        }

        for i in (LOW..=HIGH).rev() {
            delay.delay_us(8);
            let _ = channel.set_duty_cycle(i);
        }

        delay.delay_ms(500);
    }
}
