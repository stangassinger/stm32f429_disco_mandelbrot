#![no_main]
#![no_std]

extern crate cortex-m;
extern crate cortex-m-rt;
extern crate panic_halt;


// working :-) just copy to main.rs and run 
#[ cortex_m_rt::entry ] 
fn main() -> ! {
   let p = Peripherals::take().unwrap();   

    let gpiog = p.GPIOG;
    let rcc = p.RCC;

    rcc.ahbenr.modify( |r, w| w.iopeen().set_bit() );    
    gpioe.moder.write( |w| w.moder26().output().moder28().output() );
    gpioe.bsrr.write( |w| w.bs13().set().bs14().set() );
    
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
