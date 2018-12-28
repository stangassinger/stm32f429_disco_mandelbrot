#![no_main]
#![no_std]
#![feature(asm)]


pub extern crate stm32f4xx_hal as hal;

extern crate cortex_m;
extern crate cortex_m_rt;


extern crate panic_halt;

extern crate stm32f429i_disc as board;


use core::ptr;

use cortex_m_rt::entry;

use board::hal::delay::Delay;
use board::hal::prelude::*;
use board::hal::stm32;

use cortex_m::peripheral::Peripherals;


pub const RCC_AHB1ENR: *mut u32 = 0x4002_3830 as *mut u32;
pub const GPIOG_MODER: *mut u32 = 0x4002_1800 as *mut u32;
pub const GPIOG_BSRR:  *mut u32 = 0x4002_1818 as *mut u32;


#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
     //   let gpiod = p.GPIOG.split();

        // (Re-)configure PG13 (green LED) as output
     //   let mut led = gpiod.pg13.into_push_pull_output();

        // Constrain clock registers
        let mut rcc = p.RCC.constrain();

        // Configure clock to 180 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(180.mhz()).freeze();

        // Get delay provider
        let mut delay = Delay::new(cp.SYST, clocks);
/*
        loop {
            // Turn LED on
            led.set_high();

            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(500_u16);
            delay.delay_ms(500_u16);

            // Turn LED off
            led.set_low();

            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(500_u16);
            delay.delay_ms(500_u16);
        }*/



unsafe {
        // Enable GPIOD
        ptr::write_volatile(RCC_AHB1ENR, ptr::read_volatile(RCC_AHB1ENR) | 1 << 6);
        // Set PD13 Mode = Output
        ptr::write_volatile(GPIOG_MODER, ptr::read_volatile(GPIOG_MODER) | 1 << (13 * 2));
        loop {
            // Set PD13
            ptr::write_volatile(GPIOG_BSRR, 1 << (13 + 16));
            // Delay approx 1/2 second
            for _ in 0..2_000_000 { asm!("nop") }
            // Reset Set PD13
            ptr::write_volatile(GPIOG_BSRR, 1 << 13);
            // Delay approx 1/2 second
            for _ in 0..2_000_000 { asm!("nop") }
        }
}








    }

    loop {
        continue;
    }
}
