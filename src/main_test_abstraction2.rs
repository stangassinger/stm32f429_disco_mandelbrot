#![no_main]
#![no_std]

use stm32ral::{read_reg, write_reg, modify_reg, reset_reg};
//use stm32ral::{gpio, rcc, tim2, nvic, interrupt};
use stm32ral::{gpio, rcc };
extern crate panic_halt;



fn set_pin_14(gpio: &gpio::RegisterBlock) {
    write_reg!(gpio, gpio, BSRR, BS14: Set);
}


// working :-) just copy to main.rs and run 
#[ cortex_m_rt::entry ] 
fn main() -> ! {
  let rcc   = rcc::RCC::take().unwrap();
  let gpiog = gpio::GPIOG::take().unwrap();
  
   modify_reg!(rcc, rcc, AHB1ENR, GPIOGEN: Enabled);
  
   modify_reg!(gpio, gpiog, MODER, MODER13: Output,  MODER14: Output);
  
    
   loop {
        write_reg!(gpio, gpiog, BSRR, BR13: Reset, BR13: Reset);
        write_reg!(gpio, gpiog, BSRR, BR14: Reset, BR14: Reset);
        cortex_m::asm::delay(5_000_000);
        write_reg!(gpio, gpiog, BSRR, BS13: Set, BR13: Reset);
        write_reg!(gpio, gpiog, BSRR, BS14: Set, BR14: Reset);
        cortex_m::asm::delay(5_000_000);
    }
}




#[cortex_m_rt::exception]
fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[cortex_m_rt::exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
