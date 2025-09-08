#![no_std]
#![no_main]

use panic_halt as _;

use gd32vf103xx_hal::pac;
use gd32vf103xx_hal::prelude::*;
use riscv_rt::entry;

use example_longan_nano_board::{UsbBus, USB};
use usb_device::device::StringDescriptors;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(96.mhz())
        .freeze();

    assert!(rcu.clocks.usbclk_valid());

    let gpioc = dp.GPIOC.split(&mut rcu);
    let mut led = gpioc.pc13.into_push_pull_output();
    led.set_high().unwrap(); // Turn off

    let gpioa = dp.GPIOA.split(&mut rcu);
    let usb = USB {
        usb_global: dp.USBFS_GLOBAL,
        usb_device: dp.USBFS_DEVICE,
        usb_pwrclk: dp.USBFS_PWRCLK,
        pin_dm: gpioa.pa11,
        pin_dp: gpioa.pa12,
        hclk: rcu.clocks.hclk(),
    };

    static mut EP_MEMORY: [u32; 1024] = [0; 1024];
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
                led.set_low().unwrap(); // Turn on

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

        led.set_high().unwrap(); // Turn off
    }
}
