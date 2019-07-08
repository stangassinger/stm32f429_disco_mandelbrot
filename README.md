# stm32f429_disco_mandelbrot
bare metal mandelbrot implementation on stm32f429




Prerequisit:

 rust stable is required 
 
 rustup update 1.36.0
 
 rustup target add  thumbv7em-none-eabihf
 
 openocd
 
 gdb-multiarch
 
 

build with:
>> cargo build --release

flash with:
>> . ./openocd_program.sh target/thumbv7em-none-eabihf/release/mandelbrot


Debugging:

for debug output:

    let mut debug_out = hio::hstdout().unwrap();
    
    writeln!(debug_out, "Hello, world!").unwrap();
    


Terminal-1

openocd -f discovery.cfg


Terminal-2

gdb-multiarch -q target/thumbv7em-none-eabihf/release/mandelbrot

(gdb) target remote :3333

(gdb) monitor arm semihosting on

(gdb) load

(gdb) break main

(gdb) continue


#########

For debugging with gdbgui:

install gdbgui (https://ferrous-systems.com/blog/graphical-embedded-rust-debugging )

cargo run


