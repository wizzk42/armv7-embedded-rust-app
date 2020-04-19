#![no_std]
#![no_main]
#![feature(core_intrinsics)]

use core::intrinsics;

use cortex_m_semihosting::{debug, hio::{self, HStdout}};

use rt::{entry, interrupt};
use log::{debug, info, warning, error, Log, GlobalLog};

entry!(main);

fn main() -> ! {
    let hstdout: HStdout = hio::hstdout().unwrap();
    let mut logger: Logger = Logger { hstdout };

    let _ = debug!(logger, "DEBUG ENTER main entry point");

    let _ = info!(logger, "INFO ENTER main entry point");

    let _ = warning!(logger, "WARN ENTER main entry point");

    let _ = error!(logger, "ERROR ENTER main entry point");

    debug::exit(debug::EXIT_SUCCESS);

    unsafe {
        // this triggers the default exception handler to be called
        // since no other exception handler has been registered
        intrinsics::abort()
    }
}

#[no_mangle]
pub extern "C" fn hard_fault() -> ! {
    loop {}
}

struct Logger {
    hstdout: HStdout,
}

impl Log for Logger {
    type Error = ();

    fn log(&mut self, address: u8) -> Result<(), ()> {
        self.hstdout.write_all(&[address])
    }
}

impl GlobalLog for Logger {
    fn log(&self, address: u8) {
        interrupt::free(|_| unsafe {
            static mut HSTDOUT: Option<HStdout> = None;

            // lazy initialization
            if HSTDOUT.is_none() {
                HSTDOUT = Some(hio::hstdout()?);
            }

            let hstdout = HSTDOUT.as_mut().unwrap();

            hstdout.write_all(&[address])
        }).ok();
    }
}
