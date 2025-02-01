#![no_std]
#![no_main]

// fn main() {
//     println!("Hello, world!");
// }


#[panic_handler]
fn panic(_info:&core::panic::PanicInfo) -> !{
    loop {}
}

#[unsafe(no_mangle)] //makes sure all the names here are unique. Dependencies in rust are called crates. Image = bootable version of the Operating System
pub extern "C" fn _start() -> ! {
    let framebuffer = 0xb8000 as *mut u8;
    /*unsafe {
    framebuffer.offset(1).write_volatile(0x30);
    }*/
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *framebuffer.offset(i as isize * 2) = byte;
            *framebuffer.offset(i as isize * 2 + 1) = 0xb;
            }
        }
    loop {}
}

static HELLO: &[u8] = b"Hello World! This is just a quick illustration";
// panic - when an error is encountered and causes program to stop executing - logical errors, causes program to abort, when it aborts, it has to unwind
// when you unwind, you're cleaning your call stack - call stack contains all called things in the error. 
// why do we have panics and errors?