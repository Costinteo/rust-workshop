//! Task 1

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use cortex_m::asm::nop;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Level, Output, Pull};
use {defmt_rtt as _, panic_probe as _};

fn toggle_level(level: Level) -> Level {
    if level == Level::Low {
        Level::High
    } else {
        Level::Low
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut red_led = Output::new(p.PIN_0, Level::Low);
    let mut green_led = Output::new(p.PIN_1, Level::Low);
    let button_a = Input::new(p.PIN_12, Pull::Up);
    let button_b = Input::new(p.PIN_13, Pull::Up);

    loop {
        if button_a.is_low() {
            red_led.set_level(toggle_level(red_led.get_output_level()))
        }
        if button_b.is_low() {
            green_led.set_level(toggle_level(green_led.get_output_level()))
        }
    }
}
