//! Task 3       

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use cortex_m::asm::nop;
use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {

    loop {
        nop();
    }
}
