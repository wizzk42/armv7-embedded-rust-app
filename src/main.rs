#![no_std]
#![no_main]


use cortex_m_semihosting::{hprintln, hio::{self, HStdout}};

use rt::{self, entry};
use log::{debug, info, warning, error, Log, GlobalLog};

entry!(main);

fn main() -> ! {
    let hstdout: HStdout = hio::hstdout().unwrap();
    let mut logger: Logger = Logger { hstdout };

    let _ = debug!(logger, "DEBUG ENTER main entry point");

    let _ = info!(logger, "INFO ENTER main entry point");

    let _ = warning!(logger, "WARN ENTER main entry point");

    let _ = error!(logger, "ERROR ENTER main entry point");

    loop {
        let _ = hprintln!("Looping around... macro");
        let _ = debug!(logger, "Looping around");
    }
}


// exception handlers

rt::exception!(hard_fault, hard_fault_handler);
rt::exception!(sys_tick, sys_tick_handler, state: u32 = 0);

fn hard_fault_handler(_ef: &rt::ExceptionFrame) -> ! {
    loop {}
}

fn sys_tick_handler(state: &mut u32) {
    *state += 1;
    let _ = hprintln!("sys_tick received: state {}", state);
}

// logger

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
        rt::free(|_| unsafe {
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
