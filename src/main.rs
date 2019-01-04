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
use board::hal::timer::{Timer, Event};
use board::hal::gpio::Speed;

use cortex_m::peripheral::Peripherals;

#[macro_use]
mod util;


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


static mut marker : bool = false;




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


    // Set up blinking timer
    let mut led_blink_timer = Timer::tim3(p.TIM3, Hertz(4), clocks);

    // (Re-)configure PG13 (green LED) as output
    let mut led_green = gpiog.pg13.into_push_pull_output(); 
    let mut led_red   = gpiog.pg14.into_push_pull_output(); 

    // LCD enable: set it low first to avoid LCD bleed fl setting up timings
    let mut disp_on = gpioa.pa8.into_push_pull_output();
    disp_on.set_low();

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
    gpioc.pc7  .into_alternate_af14().set_speed(Speed::VeryHigh);
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



    // Enable interrupts
    let mut nvic = cp.NVIC;
    nvic.enable(board::hal::stm32::Interrupt::TIM3);
    led_blink_timer.listen(Event::TimeOut);

    // until here ok with
    //http://www.lucadavidian.com/2017/10/02/stm32-using-the-ltdc-display-controller/


// Enable clocks
    modif!(RCC.apb2enr: ltdcen = true);
    modif!(RCC.ahb1enr: dma2den = true);
    // Enable PLLSAI for LTDC
    //   PLLSAI_VCO Input = HSE_VALUE/PLL_M = 1 Mhz
    //   PLLSAI_VCO Output = PLLSAI_VCO Input * PLLSAI_N = 216 Mhz (f=100..432 MHz)
    //   PLLLCDCLK = PLLSAI_VCO Output/PLLSAI_R = 216/3 = 72 Mhz  (r=2..7)
    //   LTDC clock frequency = PLLLCDCLK / RCC_PLLSAIDivR = 72/8 = 9 Mhz (/2 /4 /8 /16)
    write!(RCC.pllsaicfgr: pllsain = 216, pllsaiq = 7, pllsair = 3);
    write!(RCC.dckcfgr: pllsaidivr = 0b10);  // divide by 8
    // Enable PLLSAI and wait for it
    modif!(RCC.cr: pllsaion = true);
    wait_for!(RCC.cr: pllsairdy);

    // Basic ChromArt configuration
    write!(DMA2D.fgpfccr: cm = 0b0101);  // L8 in/out

    // Configure LCD timings
    write!(LTDC.sscr: hsw = H_SYNCPULSE - 1, vsh = V_SYNCPULSE - 1); // -1 required by STM
    write!(LTDC.bpcr: ahbp = H_WIN_START, avbp = V_WIN_START);
    write!(LTDC.awcr: aav = H_WIN_START + H_ACTIVE, aah = V_WIN_START + V_ACTIVE);
    write!(LTDC.twcr: totalw = H_WIN_START + H_ACTIVE + H_FRONTPORCH,
           totalh = V_WIN_START + V_ACTIVE + V_FRONTPORCH);

    // Configure layer 1 (main framebuffer)

    // Horizontal and vertical window (coordinates include porches)
    write!(LTDC.l1whpcr: whstpos = H_WIN_START + 1, whsppos = H_WIN_START + WIDTH);
    write!(LTDC.l1wvpcr: wvstpos = V_WIN_START + 1, wvsppos = V_WIN_START + HEIGHT);
    // Pixel format
    write!(LTDC.l1pfcr: pf = 0b101);  // 8-bit (CLUT enabled below)
    // Constant alpha value
    write!(LTDC.l1cacr: consta = 0xFF);
    // Default color values
    write!(LTDC.l1dccr: dcalpha = 0, dcred = 0, dcgreen = 0, dcblue = 0);
    // Blending factors
    write!(LTDC.l1bfcr: bf1 = 4, bf2 = 5);  // Constant alpha
    // Color frame buffer start address
 //   write!(LTDC.l1cfbar: cfbadd = FB_CONSOLE.as_ptr() as u32);
    // Color frame buffer line length (active*bpp + 3), and pitch
    write!(LTDC.l1cfblr: cfbll = WIDTH + 3, cfbp = WIDTH);
    // Frame buffer number of lines
    write!(LTDC.l1cfblnr: cfblnbr = HEIGHT);
    // Set up 256-color LUT
 /*   for (i, (r, g, b)) in Console::get_lut_colors().enumerate() {
        write!(LTDC.l1clutwr: clutadd = i as u8, red = r, green = g, blue = b);
    }*/

    // Configure layer 2 (cursor)

    // Initial position: top left character
    write!(LTDC.l2whpcr: whstpos = H_WIN_START + 1, whsppos = H_WIN_START );
    write!(LTDC.l2wvpcr: wvstpos = V_WIN_START, wvsppos = V_WIN_START);
    write!(LTDC.l2pfcr: pf = 0b101);  // L-8 without CLUT
    write!(LTDC.l2cacr: consta = 0xFF);
    write!(LTDC.l2dccr: dcalpha = 0, dcred = 0, dcgreen = 0, dcblue = 0);
    write!(LTDC.l2bfcr: bf1 = 6, bf2 = 7);  // Constant alpha * Pixel alpha
//    write!(LTDC.l2cfbar: cfbadd = CURSORBUF.as_ptr() as u32);
    write!(LTDC.l2cfblr: cfbll =  3, cfbp = 3);
    write!(LTDC.l2cfblnr: cfblnbr = 1);  // Cursor is one line of 6 pixels

    // Enable layer1, disable layer2 initially
    modif!(LTDC.l1cr: cluten = true, len = true);
    modif!(LTDC.l2cr: len = false);

    // Reload config (immediate)
    write!(LTDC.srcr: imr = true);

    // Dither on, display on
    modif!(LTDC.gcr: den = true, ltdcen = true);

    // Reload config to show display
    write!(LTDC.srcr: imr = true);

    // Enable display via GPIO too
    disp_on.set_high();
        // Get delay provider
        let mut delay = Delay::new(cp.SYST, clocks);

        loop {
            // Turn LED on
            led_green.set_high();
           // blink( &mut true);

            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(100_u32);

            // Turn LED off
            led_green.set_low();
           // blink( &mut false );

            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(100_u32);

unsafe{ 
            if marker == true{
                led_red.set_low();
            } else{
                led_red.set_high();
            } 
       }


        }
    }

    loop {
        continue;
    }
}




board::hal::stm32f4::interrupt!(TIM3, led_blink, state: bool = false);


fn led_blink(visible: &mut bool) {
    *visible = !*visible;
    unsafe{ 
    marker = !marker;
    } 
    /*
    if *visible == true{
        //led_green.set_low();   
        modif!( )
    } else{

    } */  

    // Reset timer
    modif!(TIM3.sr: uif = false);
    modif!(TIM3.cr1: cen = true);
}
