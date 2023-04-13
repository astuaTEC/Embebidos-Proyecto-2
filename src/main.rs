// No usar la biblioteca std de linux
#![no_std]

//No buscar el main
#![no_main]

use core::panic::PanicInfo;

use core::arch::asm;

// Direcciones que debo  acceder 
// 0x F220 0008 fsel 2 1<<3, para colocarlo como output
// 0x F220 001C fset para encender el pin 21
// 0x F220 0028 fclear para apagar el pin 21

mod boot {
    use core::arch::global_asm;

    // Una macro que indica que el codigo se encuentra
    // en .text.start, hay que cambiarlo con el linker
    global_asm!(
        ".section .text._start"
    );
}

// Definir una funcion que no retorne nada
// para que lo visualice el linker e indique _start()
#[no_mangle]
pub extern "C" fn _start() -> !{
    unsafe{
        // Para colocar el pin 21
        core::ptr::write_volatile(0xF220_0008 as *mut u32, 1<<3);
        loop{
            //Encender y apagar un pin
            core::ptr::write_volatile(0xF220_001C as *mut u32, 1<<21);

            for _ in 1..50000 {
                asm!("nop");
            }

            core::ptr::write_volatile(0xF220_0028 as *mut u32, 1<<21);

            for _ in 1..50000 {
                asm!("nop");
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {
        
    }
}