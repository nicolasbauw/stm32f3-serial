#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::entry;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer ) = aux11::init();

    // Send a single character
    usart1
        .tdr
        .write(|w| w.tdr().bits(u16::from(b'X')) );

        hprintln!("Sent X!");

    loop {}
}