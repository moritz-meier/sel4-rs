use core::fmt::Write;

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

use crate::static_ref::StaticRef;

pub struct Uart {
    registers: StaticRef<UartRegisters>,
}

unsafe impl Send for Uart {}

impl Uart {
    pub const fn new(base: StaticRef<UartRegisters>) -> Self {
        Self { registers: base }
    }

    pub fn init(&mut self) {
        self.registers
            .cr
            .write(CR::TX_EN::EnableTransmit + CR::TX_DIS::EnableTransmit);
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.as_bytes() {
            while !self.registers.sr.is_set(SR::TXEMPTY) {}
            self.registers.fifo.write(FIFO::FIFO.val(*b as u32))
        }

        Ok(())
    }
}

register_structs! {
    /// Universal synchronous asynchronous receiver transmitter
    pub UartRegisters {
        (0x00 => cr: ReadWrite<u32, CR::Register>),
        (0x04 => mr: ReadWrite<u32, MR::Register>),
        (0x08 => ier: ReadWrite<u32, IER::Register>),
        (0x0C => idr: ReadWrite<u32, IDR::Register>),
        (0x10 => imr: ReadOnly<u32, IMR::Register>),
        (0x14 => isr: ReadWrite<u32, ISR::Register>),
        (0x18 => baudgen: ReadWrite<u32, BAUDGEN::Register>),
        (0x1C => rxtout: ReadWrite<u32, RXTOUT::Register>),
        (0x20 => rxwm: ReadWrite<u32, RXWM::Register>),
        (0x24 => modemcr: ReadWrite<u32, MODEMCR::Register>),
        (0x28 => modemsr: ReadWrite<u32, MODEMSR::Register>),
        (0x2C => sr: ReadOnly<u32, SR::Register>),
        (0x30 => fifo: ReadWrite<u32, FIFO::Register>),
        (0x34 => baud_div: ReadWrite<u32, BAUD_DIV::Register>),
        (0x38 => flow_delay: ReadWrite<u32, FLOW_DELAY::Register>),
        (0x3C => _reserved),
        (0x44 => tx_fifo_trig: ReadWrite<u32, TX_FIFO_TRIG::Register>),
        (0x48 => @END),
    }
}

register_bitfields![
    u32,
    CR [
        STOPBRK OFFSET(8) NUMBITS(1) [],
        STARTBRK OFFSET(7) NUMBITS(1) [],
        TORST OFFSET(6) NUMBITS(1) [],
        TX_DIS OFFSET(5) NUMBITS(1) [
            EnableTransmit = 0,
            DisableTransmit = 1,
        ],
        TX_EN OFFSET(4) NUMBITS(1) [
            DisableTransmit = 0,
            EnableTransmit = 1
        ],
        RX_DIS OFFSET(3) NUMBITS(1) [
            EnableReceive = 0,
            DisableReceive = 1
        ],
        RX_EN OFFSET(2) NUMBITS(1) [
            DisableReceive = 0,
            EnableReceive = 1
        ],
        TXRST OFFSET(1) NUMBITS(1) [
            NoAffect = 0,
            Reset = 1
        ],
        RXRST OFFSET(0) NUMBITS(1) [
            NoAffect = 0,
            Reset = 1
        ],
    ],
    MR [
        CHMODE OFFSET(8) NUMBITS(2) [],
        NBSTOP OFFSET(6) NUMBITS(2) [],
        PAR OFFSET(3) NUMBITS(3) [],
        CHRL OFFSET(1) NUMBITS(2) [],
        CLKSEL OFFSET(0) NUMBITS(1) [],
    ],
    IER [
        TOVR OFFSET(12) NUMBITS(1) [],
        TNFUL OFFSET(11) NUMBITS(1) [],
        TTRIG OFFSET(10) NUMBITS(1) [],
        IXR_DMS OFFSET(9) NUMBITS(1) [],
        IXR_TOUT OFFSET(8) NUMBITS(1) [],
        IXR_PARITY OFFSET(7) NUMBITS(1) [],
        IXR_FRAMING OFFSET(6) NUMBITS(1) [],
        IXR_OVER OFFSET(5) NUMBITS(1) [],
        IXR_TXFULL OFFSET(4) NUMBITS(1) [],
        IXR_TXEMPTY OFFSET(3) NUMBITS(1) [],
        IXR_RXFULL OFFSET(2) NUMBITS(1) [],
        IXR_RXEMPTY OFFSET(1) NUMBITS(1) [],
        IXR_RXOVR OFFSET(0) NUMBITS(1) [],
    ],
    IDR [
        TOVR OFFSET(12) NUMBITS(1) [],
        TNFUL OFFSET(11) NUMBITS(1) [],
        TTRIG OFFSET(10) NUMBITS(1) [],
        IXR_DMS OFFSET(9) NUMBITS(1) [],
        IXR_TOUT OFFSET(8) NUMBITS(1) [],
        IXR_PARITY OFFSET(7) NUMBITS(1) [],
        IXR_FRAMING OFFSET(6) NUMBITS(1) [],
        IXR_OVER OFFSET(5) NUMBITS(1) [],
        IXR_TXFULL OFFSET(4) NUMBITS(1) [],
        IXR_TXEMPTY OFFSET(3) NUMBITS(1) [],
        IXR_RXFULL OFFSET(2) NUMBITS(1) [],
        IXR_RXEMPTY OFFSET(1) NUMBITS(1) [],
        IXR_RXOVR OFFSET(0) NUMBITS(1) [],
    ],
    IMR [
        TOVR OFFSET(12) NUMBITS(1) [],
        TNFUL OFFSET(11) NUMBITS(1) [],
        TTRIG OFFSET(10) NUMBITS(1) [],
        IXR_DMS OFFSET(9) NUMBITS(1) [],
        IXR_TOUT OFFSET(8) NUMBITS(1) [],
        IXR_PARITY OFFSET(7) NUMBITS(1) [],
        IXR_FRAMING OFFSET(6) NUMBITS(1) [],
        IXR_OVER OFFSET(5) NUMBITS(1) [],
        IXR_TXFULL OFFSET(4) NUMBITS(1) [],
        IXR_TXEMPTY OFFSET(3) NUMBITS(1) [],
        IXR_RXFULL OFFSET(2) NUMBITS(1) [],
        IXR_RXEMPTY OFFSET(1) NUMBITS(1) [],
        IXR_RXOVR OFFSET(0) NUMBITS(1) [],
    ],
    ISR [
        TOVR OFFSET(12) NUMBITS(1) [],
        TNFUL OFFSET(11) NUMBITS(1) [],
        TTRIG OFFSET(10) NUMBITS(1) [],
        IXR_DMS OFFSET(9) NUMBITS(1) [],
        IXR_TOUT OFFSET(8) NUMBITS(1) [],
        IXR_PARITY OFFSET(7) NUMBITS(1) [],
        IXR_FRAMING OFFSET(6) NUMBITS(1) [],
        IXR_OVER OFFSET(5) NUMBITS(1) [],
        IXR_TXFULL OFFSET(4) NUMBITS(1) [],
        IXR_TXEMPTY OFFSET(3) NUMBITS(1) [],
        IXR_RXFULL OFFSET(2) NUMBITS(1) [],
        IXR_RXEMPTY OFFSET(1) NUMBITS(1) [],
        IXR_RXOVR OFFSET(0) NUMBITS(1) [],
    ],
    BAUDGEN [
        CD OFFSET(10) NUMBITS(16) [],
    ],
    RXTOUT [
        RTO OFFSET(0) NUMBITS(7) [],
    ],
    RXWM [
        RTRIG OFFSET(0) NUMBITS(5) [],
    ],
    MODEMCR [
        FCM OFFSET(5) NUMBITS(1) [],
        RTS OFFSET(1) NUMBITS(1) [],
        DTR OFFSET(0) NUMBITS(1) [],
    ],
    MODEMSR [
        FCMS OFFSET(8) NUMBITS(1) [],
        DCD OFFSET(7) NUMBITS(1) [],
        RI OFFSET(6) NUMBITS(1) [],
        DSR OFFSET(5) NUMBITS(1) [],
        CTS OFFSET(4) NUMBITS(1) [],
        MEDEMSR_DCDX OFFSET(3) NUMBITS(1) [],
        MEDEMSR_RIX OFFSET(2) NUMBITS(1) [],
        MEDEMSR_DSRX OFFSET(1) NUMBITS(1) [],
        MEDEMSR_CTSX OFFSET(0) NUMBITS(1) [],
    ],
    SR [
        TNFUL OFFSET(14) NUMBITS(1) [
            MoreThanOneByte = 0,
            OneByte = 1
        ],
        TTRIG OFFSET(13) NUMBITS(1) [
            Less = 0,
            Greater = 1,
        ],
        FLOWDEL OFFSET(12) NUMBITS(1) [
            Less = 0,
            Greater = 1,
        ],
        TACTIVE OFFSET(11) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],
        RACTIVE OFFSET(10) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],
        TXFULL OFFSET(4) NUMBITS(1) [
            NotFull = 0,
            Full = 1
        ],
        TXEMPTY OFFSET(3) NUMBITS(1) [
            NotEmpty = 0,
            Empty = 1
        ],
        RXFULL OFFSET(2) NUMBITS(1) [
            Full = 1,
            NotFull = 0
        ],
        RXEMPTY OFFSET(1) NUMBITS(1) [
            NotEmpty = 0,
            Empty = 1
        ],
        RXOVR OFFSET(0) NUMBITS(1) [
            Less = 0,
            Greater = 1,
        ],
    ],
    FIFO [
        FIFO OFFSET(0) NUMBITS(8) [],
    ],
    BAUD_DIV [
        BDIV OFFSET(0) NUMBITS(8) [],
    ],
    FLOW_DELAY [
        FDEL OFFSET(0) NUMBITS(6) [],
    ],
    TX_FIFO_TRIG [
        TTRIG OFFSET(0) NUMBITS(6) []
    ]
];
