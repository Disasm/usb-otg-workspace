//! CDC-ACM serial port example using polling in a busy loop.
#![no_std]
#![no_main]

use panic_rtt_target as _;

use cortex_m_rt::entry;
#[cfg(feature = "fs")]
use stm32f4xx_hal::otg_fs::{UsbBus, USB};
#[cfg(feature = "hs")]
use stm32f4xx_hal::otg_hs::{UsbBus, USB};
use stm32f4xx_hal::{pac, prelude::*};
use usb_device::device::StringDescriptors;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(48.MHz())
        .pclk1(24.MHz())
        .require_pll48clk()
        .freeze();

    #[cfg(feature = "fs")]
    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();

    let mut led = gpiob.pb7.into_push_pull_output();
    led.set_low(); // Turn off

    #[cfg(feature = "fs")]
    let usb = USB {
        usb_global: dp.OTG_FS_GLOBAL,
        usb_device: dp.OTG_FS_DEVICE,
        usb_pwrclk: dp.OTG_FS_PWRCLK,
        pin_dm: gpioa.pa11.into(),
        pin_dp: gpioa.pa12.into(),
        hclk: clocks.hclk(),
    };
    #[cfg(feature = "hs")]
    let usb = USB {
        usb_global: dp.OTG_HS_GLOBAL,
        usb_device: dp.OTG_HS_DEVICE,
        usb_pwrclk: dp.OTG_HS_PWRCLK,
        pin_dm: gpiob.pb14.into(),
        pin_dp: gpiob.pb15.into(),
        hclk: clocks.hclk(),
    };

    let usb_bus = UsbBus::new(usb, unsafe { &mut EP_MEMORY });

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .strings(&[StringDescriptors::default()
            .manufacturer("Fake company")
            .product("Serial port")
            .serial_number("TEST")])
        .unwrap()
        .device_class(USB_CLASS_CDC)
        .build();

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
