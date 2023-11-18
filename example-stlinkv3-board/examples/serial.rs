//! CDC-ACM serial port example using polling in a busy loop.
#![no_std]
#![no_main]

use panic_rtt_target as _;

use cortex_m_rt::entry;
use example_stlinkv3_board::restore;
#[cfg(feature = "fs")]
use stm32f7xx_hal::otg_fs::{UsbBus, USB};
#[cfg(feature = "hs")]
use stm32f7xx_hal::otg_hs::{UsbBus, USB};
use stm32f7xx_hal::pac;
use stm32f7xx_hal::prelude::*;
use stm32f7xx_hal::rcc::{HSEClock, HSEClockMode};
use usb_device::device::StringDescriptors;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    let cp = cortex_m::Peripherals::take().unwrap_or_else(|| loop {
        continue;
    });
    let dp = pac::Peripherals::take().unwrap_or_else(|| loop {
        continue;
    });

    restore(&cp, &dp);

    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .hse(HSEClock::new(25.MHz(), HSEClockMode::Bypass))
        .sysclk(72.MHz())
        .freeze();

    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa10.into_push_pull_output();
    led.set_low(); // Turn off

    #[cfg(feature = "hs")]
    let gpiob = dp.GPIOB.split();

    #[cfg(feature = "fs")]
    let usb = USB::new(
        dp.OTG_FS_GLOBAL,
        dp.OTG_FS_DEVICE,
        dp.OTG_FS_PWRCLK,
        (gpioa.pa11.into_alternate(), gpioa.pa12.into_alternate()),
        clocks,
    );
    #[cfg(feature = "hs")]
    let usb = USB::new_with_internal_hs_phy(
        dp.OTG_HS_GLOBAL,
        dp.OTG_HS_DEVICE,
        dp.OTG_HS_PWRCLK,
        dp.USBPHYC,
        (gpiob.pb14.into_alternate(), gpiob.pb15.into_alternate()),
        clocks,
    );

    let usb_bus = UsbBus::new(usb, unsafe { &mut EP_MEMORY });

    let mut serial = SerialPort::new(&usb_bus);

    let builder = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .strings(&[StringDescriptors::default()
            .manufacturer("Fake company")
            .product("Serial port")
            .serial_number("TEST")])
        .unwrap()
        .device_class(USB_CLASS_CDC);
    #[cfg(feature = "hs")]
    let builder = builder.max_packet_size_0(64).unwrap();
    let mut usb_dev = builder.build();

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                led.set_high(); // Turn on

                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                let mut write_offset = 0;
                while write_offset < count {
                    match serial.write(&buf[write_offset..count]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        led.set_low(); // Turn off
    }
}
