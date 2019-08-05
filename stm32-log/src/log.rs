use core::cell::RefCell;
use core::fmt::Write;
use cortex_m::interrupt::Mutex;
use log::{Metadata, Record};
use bbqueue::BBQueue;

static mut BUFFER: [u8; 1024] = [0u8; 1024];
static LOGBUF: Mutex<RefCell<Buffer>> = Mutex::new(RefCell::new(Buffer::new()));

pub struct Buffer(Option<BBQueue>);

impl Buffer {
    const fn new() -> Self {
        Buffer(None)
    }
}

unsafe impl Send for Buffer {}

impl Buffer {
    fn queue(&mut self) -> &mut BBQueue {
        if self.0.is_none() {
            unsafe {
                self.0 = Some(BBQueue::unpinned_new(&mut BUFFER));
            }
        }
        self.0.as_mut().unwrap()
    }

    fn flush(&mut self) {
        let queue = self.queue();

        if let Ok(r) = queue.read() {
            crate::write_bytes(&r);
            queue.release(r.len(), r);
        }
    }
}

impl core::fmt::Write for Buffer {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        let queue = self.queue();

        let bytes = s.as_bytes();
        if let Ok(mut w) = queue.grant(bytes.len() * 2) {
            let mut index = 0;
            for byte in bytes {
                if *byte == 0x0A {
                    w[index] = 0x0D;
                    index += 1;
                }
                w[index] = *byte;
                index += 1;
            }
            queue.commit(index, w);
        } else {
            panic!("can't write");
        }
        Ok(())
    }
}

pub struct BufferLogger;

impl log::Log for BufferLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            cortex_m::interrupt::free(|cs| {
                // make sure you are alone on the buffer
                let mut buffer = LOGBUF.borrow(cs).borrow_mut();
                writeln!(
                    buffer,
                    "{} {}: {}",
                    record.level(),
                    record.target(),
                    record.args()
                )
                .unwrap();
            });
        }
    }

    fn flush(&self) {
        cortex_m::interrupt::free(|cs| {
            let mut buffer = LOGBUF.borrow(cs).borrow_mut();
            buffer.flush();
        });
    }
}

pub fn init() {
    static LOGGER: BufferLogger = BufferLogger;
    let _ = log::set_logger(&LOGGER).unwrap();
}
