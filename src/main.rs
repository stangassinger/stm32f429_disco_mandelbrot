#![no_main]
#![no_std]


pub extern crate stm32f4xx_hal as hal;

extern crate cortex_m;
extern crate cortex_m_rt;


extern crate panic_halt;

extern crate stm32f429i_disc as board;

use cortex_m_rt::entry;

use board::hal::delay::Delay;
use board::hal::prelude::*;
use board::hal::stm32;
use board::hal::time::*;
use board::hal::gpio::Speed;

use cortex_m::peripheral::Peripherals;


// Display
/// Width and height of visible screen.
const WIDTH: u16 = 240;
const HEIGHT: u16 = 320;


/// Horizontal display timing.
const H_SYNCPULSE:  u16 = 10;
const H_BACKPORCH:  u16 = 20;
const H_ACTIVE:     u16 = WIDTH;
const H_FRONTPORCH: u16 = 10;

/// Vertical display timing.
const V_SYNCPULSE:  u16 = 2;
const V_BACKPORCH:  u16 = 2;
const V_ACTIVE:     u16 = HEIGHT;  
const V_FRONTPORCH: u16 = 4;

/// Upper-left corner of screen for layer windows.
const H_WIN_START:  u16 = H_SYNCPULSE + H_BACKPORCH - 1;
const V_WIN_START:  u16 = V_SYNCPULSE + V_BACKPORCH - 1;

// Graphics framebuffer
const FB_GRAPHICS_SIZE: usize = (WIDTH as usize) * (HEIGHT as usize);

#[link_section = ".sram1bss"]
static mut FB_GRAPHICS: [u8; FB_GRAPHICS_SIZE] = [0; FB_GRAPHICS_SIZE];




#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        // Constrain clock registers
        let mut rcc = p.RCC.constrain();

        // Configure clock to 168 MHz  and freeze it
        rcc.cfgr = rcc.cfgr.sysclk(MegaHertz(168))
                        .hclk(MegaHertz(168))
                        .pclk1(MegaHertz(42))
                        .pclk2(MegaHertz(84));
        let clocks = rcc.cfgr.freeze();

// Set up pins
    let gpioa = p.GPIOA.split();
    let gpiob = p.GPIOB.split();
    let gpioc = p.GPIOC.split();
    let gpiod = p.GPIOD.split();
    let gpioe = p.GPIOE.split();   
    let gpiof = p.GPIOF.split();   
    let gpiog = p.GPIOG.split();

    // (Re-)configure PG13 (green LED) as output
    let mut led = gpiog.pg14.into_push_pull_output(); 

    // LCD enable: set it low first to avoid LCD bleed fl setting up timings
 //   let mut disp_on = gpioa.pa8.into_push_pull_output();
 //   disp_on.set_low();

    // LCD backlight enable
 //   let mut backlight = gpiod.pd12.into_push_pull_output();
 //   backlight.set_high();

    // Output pin connected to Boot0 + capacitor
 //   let mut bootpin = gpiob.pb7.into_push_pull_output();
  //  bootpin.set_low();

    // LCD pins
    gpioa.pa3 .into_alternate_af14().set_speed(Speed::VeryHigh);
    gpioa.pa4 .into_alternate_af14().set_speed(Speed::VeryHigh);
    gpioa.pa6 .into_alternate_af14().set_speed(Speed::VeryHigh);
    gpioa.pa11.into_alternate_af14().set_speed(Speed::VeryHigh);
    gpioa.pa12.into_alternate_af14().set_speed(Speed::VeryHigh);
    gpiob.pb0 .into_alternate_af9() .set_speed(Speed::VeryHigh);
    gpiob.pb1 .into_alternate_af9() .set_speed(Speed::VeryHigh);
    gpiog.pg10.into_alternate_af9().set_speed(Speed::VeryHigh);
    gpiog.pg12.into_alternate_af9().set_speed(Speed::VeryHigh);
    gpiob.pb8 .into_alternate_af14().set_speed(Speed::VeryHigh);
    gpiob.pb9 .into_alternate_af14().set_speed(Speed::VeryHigh);
    gpiob.pb10.into_alternate_af14().set_speed(Speed::VeryHigh);
    gpiob.pb11.into_alternate_af14().set_speed(Speed::VeryHigh);
    gpioc.pc6 .into_alternate_af14().set_speed(Speed::VeryHigh);
    gpioc.pc7 .into_alternate_af14().set_speed(Speed::VeryHigh);
    gpioc.pc10.into_alternate_af14().set_speed(Speed::VeryHigh);
    gpiod.pd3 .into_alternate_af14().set_speed(Speed::VeryHigh);
    gpiod.pd6 .into_alternate_af14().set_speed(Speed::VeryHigh);
    gpioe.pe11.into_alternate_af14().set_speed(Speed::VeryHigh);
    gpioe.pe12.into_alternate_af14().set_speed(Speed::VeryHigh);
    gpioe.pe13.into_alternate_af14().set_speed(Speed::VeryHigh);
    gpioe.pe14.into_alternate_af14().set_speed(Speed::VeryHigh);
    gpioe.pe15.into_alternate_af14().set_speed(Speed::VeryHigh);
    gpiof.pf10.into_alternate_af14().set_speed(Speed::VeryHigh);
    gpiog.pg6.into_alternate_af14().set_speed(Speed::VeryHigh);
    gpiog.pg7.into_alternate_af14().set_speed(Speed::VeryHigh);
    gpiog.pg11.into_alternate_af14().set_speed(Speed::VeryHigh);
    // until here ok with
    //http://www.lucadavidian.com/2017/10/02/stm32-using-the-ltdc-display-controller/



        // Get delay provider
        let mut delay = Delay::new(cp.SYST, clocks);

        loop {
            // Turn LED on
            led.set_high();

            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(1000_u32);

            // Turn LED off
            led.set_low();

            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(1000_u32);
        }
    }

    loop {
        continue;
    }
}
