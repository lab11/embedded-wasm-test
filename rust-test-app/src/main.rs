#![no_std]
#![feature(start)]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
        loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
        let mut a = 1;

            for x in 2..10 {
                        a = x*a;
                            }

                return 0;
}
