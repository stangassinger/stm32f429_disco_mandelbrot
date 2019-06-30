#![no_std]
#![feature(used)]
#![feature(lang_items)]

extern crate cortex_m;
extern crate cortex_m_rt;

use cortex_m::asm;
use core::ptr;


#[entry]
fn main() -> ! {


    unsafe {
        const RCC_AHB1ENR: u32  = 0x40023800 + 0x30;
        const GPIOG_BSRR:  u32  = 0x48001018;
        const GPIOG_MODER: u32  = 0x48001000;
 
        //using PG13 PG14 as LED
        let x = ptr::read_volatile( RCC_AHB1ENR as *mut u32 );      
        ptr::write_volatile(RCC_AHB1ENR as *mut u32, x | (1 << 6));       
        ptr::write_volatile(GPIOE_MODER as *mut u32, (1 << 18) | (1 << 22));    
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);
    }

    loop {}
}



#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C"  fn default_handler() {
    asm::bkpt();
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    loop {}
}