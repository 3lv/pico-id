#![no_std]
#![no_main]

use panic_halt as _;

use rp235x_hal as hal;
use hal::Clock;

use embedded_hal::delay::DelayNs;
use embedded_hal::pwm::SetDutyCycle;

#[link_section = ".start_block"]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

struct PwmController<'a> {
    clock_freq: u32,
    pwm_slices: &'a mut hal::pwm::Slices,
}

impl<'a> PwmController<'a> {
    fn new(
        clocks: &hal::clocks::ClocksManager,
        pwm_slices: &'a mut hal::pwm::Slices,
        pin: hal::gpio::Pin<hal::gpio::bank0::Gpio26, hal::gpio::FunctionNull, hal::gpio::PullDown>,
    ) -> Self {
        let pwm = &mut pwm_slices.pwm5;

        pwm.set_ph_correct();
        pwm.enable();

        let channel = &mut pwm.channel_a;
        channel.output_to(pin);

        Self {
            clock_freq: clocks.system_clock.freq().to_Hz(),
            pwm_slices,
        }
    }

    // Min frequency seems to be around 4Hz
    fn set_frequency(&mut self, desired_frequency: u32) {
        let pwm = &mut self.pwm_slices.pwm5;

        /*
        pwm.set_div_int(0xFFu8);
        pwm.set_top(27058u16);
        return;
        */
        let mut prescaler = 1;
        let mut top_value = self.clock_freq / desired_frequency;

        while top_value > 0xFFFF {
            prescaler += 1;
            top_value = self.clock_freq / (prescaler * desired_frequency);
        }
        /*
        let mut step = 1 << 7;
        while step != 0 {
            let new_prescaler = prescaler + step;
            let new_top_value = self.clock_freq / (new_prescaler * desired_frequency);
            if new_prescaler <= 0xFF && new_top_value > 0xFFFF {
                prescaler = new_prescaler;
                top_value = new_top_value;
            }
            step >>= 1;
        }
        prescaler += 1;
        */

        pwm.set_div_int(prescaler as u8);
        pwm.set_top(top_value as u16);
    }

    fn set_duty(&mut self, duty_percent: u8) {
        let pwm = &mut self.pwm_slices.pwm5;
        
        let duty = ((pwm.channel_a.max_duty_cycle() as u32 * duty_percent as u32) / 100) as u16;

        let _ = pwm.channel_a.set_duty_cycle(duty);
    }
}

#[hal::entry]
fn main() -> ! {
    let mut pac = hal::pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        12_000_000,
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

    let pwm_pin = pins.gpio26.into_function();

    let mut pwm_controller = PwmController::new(&clocks, &mut pwm_slices, pwm_pin);

    // 69k-690k decent but requires something very close(with amplifier might be perfect)
    // 6.9M seems strange, it gets really strange triangles, but when it comes to be read throuw
    //   human body, amplifications may be required
    pwm_controller.set_frequency(20_000);
    pwm_controller.set_duty(50);
    
    loop {};
    loop {
        for i in 0..=100 {
            pwm_controller.set_duty(i);
            delay.delay_ms(5);
        }
        for i in (0..=100).rev() {
            pwm_controller.set_duty(i);
            delay.delay_ms(5);
        }
        delay.delay_ms(1000);
    }
    loop {
        pwm_controller.set_frequency(100000);
        pwm_controller.set_duty(50);
        delay.delay_ms(10000);
        pwm_controller.set_frequency(1000);
        pwm_controller.set_duty(50);
        delay.delay_ms(10000);
    }

    // Infinite loop, fading LED up and down
    let t = 5000;
    /*
    // Pair mode
    pwm_controller.set_frequency(100000);
    loop {
        for i in 0..=100 {
            pwm_controller.set_duty(i);
            delay.delay_ms(t/100);
        }
        for i in (0..=100).rev() {
            pwm_controller.set_duty(i);
            delay.delay_ms(t/100);
        }
        delay.delay_ms(1000);
    }
    */
}
