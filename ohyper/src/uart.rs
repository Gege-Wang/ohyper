use core::fmt::Write;
use x86_64::instructions::port::Port;

pub struct SerialPort {
    port: u16,
}

impl SerialPort {
    pub fn new(port: u16) -> Self {
        let mut port = SerialPort { port };

        port.write_register(1, 0x00); // disable interrupts
        port.write_register(3, 0x80); // enable DLAB
        port.write_register(0, 0x01); // set baud rate
        port.write_register(1, 0x00); // set baud rate
        port.write_register(3, 0x03); // 8 bits data, data checksum

        port
    }

    fn write_register(&mut self, reg: u16, value: u8) {
        unsafe {
            Port::new(self.port + reg).write(value);
        }
    }

    // send a data from port
    pub fn send(&mut self, data: u8) {
        while self.is_transmit_empty() == false {}
        self.write_register(0, data);
    }

    fn is_transmit_empty(&mut self) -> bool {
        unsafe {
            let lsb: u8 = Port::new(self.port + 5).read();
            lsb & 0x20 != 0
        }
    }
}

impl Write for SerialPort {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            self.send(byte);
        }
        Ok(())
    }
}
