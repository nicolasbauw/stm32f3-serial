#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::entry;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer ) = aux11::init();

    //let instant = mono_timer.now();
    // Send a single character
    for byte in b"The quick brown fox jumps over the lazy dog.\n\r".iter() {
        // wait until it's safe to write to TDR
        while usart1.isr.read().txe().bit_is_clear() {}
        usart1
            .tdr
            .write(|w| w.tdr().bits(u16::from(*byte)) );
    }
        hprintln!("Data Sent ");

    panic!("Done !");
}