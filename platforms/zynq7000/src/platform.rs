use core::cell::RefCell;

use critical_section::Mutex;
use fdt::{node::FdtNode, Fdt};

use crate::{
    static_ref::StaticRef,
    uart::{Uart, UartRegisters},
};

pub static SERIAL: Mutex<RefCell<Option<Uart>>> = Mutex::new(RefCell::new(None));

pub fn platform_init(dt: &Fdt) {
    let stdout = dt.chosen().stdout();

    if let Some(stdout) = stdout {
        serial_init(stdout.node());
    }
}

fn serial_init(node: FdtNode) {
    let uart_base = node
        .reg()
        .and_then(|mut reg| reg.next())
        .map(|reg| unsafe { StaticRef::new(reg.starting_address as *const UartRegisters) });

    let uart = uart_base.map(Uart::new);

    if let Some(mut uart) = uart {
        critical_section::with(|cs| {
            let mut serial = SERIAL.borrow_ref_mut(cs);

            uart.init();
            serial.replace(uart);
        });
    }
}
