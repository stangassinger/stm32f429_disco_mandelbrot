# stm32f429_disco_mandelbrot
bare metal mandelbrot implementation on stm32f429




Prerequisit:

 rust stable is required 
 
 rustup update
 
 rustup target add  thumbv7em-none-eabihf
 
 openocd
 
 gdb-multiarch
 
 

build with:
>> cargo build

flash with:
>> . ./openocd_program.sh target/thumbv7em-none-eabihf/debug/mandelbrot
