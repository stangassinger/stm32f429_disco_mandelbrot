#![no_main]
#![no_std]
// https://flowdsp.io/blog/stm32f3-01-interrupts

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f4;

extern crate panic_halt;


// working :-) just copy to main.rs and run 
#[ cortex_m_rt::entry ] 
fn main() -> ! {


    unsafe {
        const RCC_AHB1ENR: u32  = 0x40023800 + 0x30;        
        const GPIOG_MODER: u32  = 0x40021800;
        const GPIOG_BSRR:  u32  = 0x40021818;
 
        //using PG13 PG14 as LED
        let x = core::ptr::read_volatile( RCC_AHB1ENR as *mut u32 );      
        core::ptr::write_volatile( RCC_AHB1ENR as *mut u32, x | ( 1 << 6 ) );       
        core::ptr::write_volatile( GPIOG_MODER as *mut u32, ( 1 << 26) | ( 1 << 28 ));    
        core::ptr::write_volatile( GPIOG_BSRR  as *mut u32,   1 << 13 );
        core::ptr::write_volatile( GPIOG_BSRR  as *mut u32,   1 << 14 );
    } 

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
