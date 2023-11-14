#![no_std]
#![no_main]

use panic_halt as _;

use gd32vf103xx_hal::pac;
use gd32vf103xx_hal::prelude::*;
use riscv_rt::entry;

use example_longan_nano_board::{UsbBus, USB};
use usb_device::prelude::*;

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

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

    let gpioa = dp.GPIOA.split(&mut rcu);
    let usb = USB {
        usb_global: dp.USBFS_GLOBAL,
        usb_device: dp.USBFS_DEVICE,
        usb_pwrclk: dp.USBFS_PWRCLK,
        pin_dm: gpioa.pa11,
        pin_dp: gpioa.pa12,
        hclk: rcu.clocks.hclk(),
    };

    let usb_bus = UsbBus::new(usb, unsafe { &mut EP_MEMORY });

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Enumeration test")
        .serial_number("TEST")
        .device_class(0)
        .build();

    loop {
        if usb_dev.poll(&mut []) {}
    }
}
