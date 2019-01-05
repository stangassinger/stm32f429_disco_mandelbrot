# stm32f429_disco_mandelbrot
bare metal mandelbrot implementation on stm32f429

Early stage not working yet!

Only LED blinking 

Prerequisit:
 rustup target add  thumbv7em-none-eabihf
 rust stable is required 
 

build with:
>> cargo build

flash with:
>> . ./openocd_program.sh target/thumbv7em-none-eabihf/debug/mandelbrot
