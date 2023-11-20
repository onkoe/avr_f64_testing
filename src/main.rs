#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;
use ufmt::uwriteln;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    for num in 0..100 {
        let calc = ((num >> 3) & 0x1FFF) as f64 * 0.25_f64;

        uwriteln!(
            &mut serial,
            "#{}, got {}",
            num,
            ufmt_float::uFmt_f64::Five(calc)
        )
        .void_unwrap();
    }

    loop {
        arduino_hal::delay_ms(65535);
    }
}
