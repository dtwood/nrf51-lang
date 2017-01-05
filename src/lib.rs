#![no_std]
#![feature(lang_items)]

extern crate nrf51_hardware;

#[macro_export]
macro_rules! print {
    ( $ ( $ arg : tt ) * ) => (
        let _ = ::core::fmt::Write::write_fmt(&mut *::nrf51_hardware::serial::SERIAL.lock(), format_args!($($arg)*));
    );
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (
        print!(concat!($fmt, "\n"));
    );
    ($fmt:expr, $($arg:tt)*) => (
        print!(concat!($fmt, "\n"), $($arg)*);
    );
}

#[lang = "start"]
fn lang_start(main: *const u8, _argc: isize, _argv: *const *const u8) -> isize {
    unsafe { core::mem::transmute::<_, fn()>(main)() };

    0
}

#[no_mangle]
pub extern "C" fn SystemInit() {
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(details: ::core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    print!("application panicked at '");
    use core::fmt::Write;
    let _ = nrf51_hardware::serial::SERIAL.lock().write_fmt(details);
    println!("', {}:{}", file, line);
    loop {
    }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn _exit() {
    panic!("Embedded applications cannot exit, entering infinite loop");
}

#[no_mangle]
pub extern "C" fn _kill() {
    panic!("Embedded applications cannot exit, entering infinite loop");
}

#[no_mangle]
pub extern "C" fn _getpid() {
    unimplemented!();
}
