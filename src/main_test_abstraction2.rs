#![no_main]
#![no_std]

use cortex_m;
use cortex_m_rt;
use stm32f4::stm32f429;
//use panic_halt;


// working :-) just copy to main.rs and run 
#[ cortex_m_rt::entry ] 
fn main() -> ! {
   let p = cortex_m::Peripherals::take().unwrap();   

    let gpiog = p.GPIOG;
    let rcc = p.RCC;

    rcc.ahbenr.modify( |r, w| w.iopeen().set_bit() );    
    gpiog.moder.write( |w| w.moder26().output().moder28().output() );
    gpiog.bsrr.write( |w| w.bs13().set().bs14().set() );
    
    loop {}
}




#[cortex_m_rt::exception]
fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[cortex_m_rt::exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
