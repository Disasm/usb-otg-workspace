#![no_std]
#![no_main]

use panic_rtt_target as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::otg_fs::{UsbBus, USB};
use stm32f4xx_hal::{pac, prelude::*};
use usb_device::device::StringDescriptors;
use usb_device::prelude::*;

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(25.MHz())
        .sysclk(48.MHz())
        .pclk1(24.MHz())
        .require_pll48clk()
        .freeze();

    let gpioa = dp.GPIOA.split();

    let usb = USB {
        usb_global: dp.OTG_FS_GLOBAL,
        usb_device: dp.OTG_FS_DEVICE,
        usb_pwrclk: dp.OTG_FS_PWRCLK,
        pin_dm: gpioa.pa11.into(),
        pin_dp: gpioa.pa12.into(),
        hclk: clocks.hclk(),
    };

    let usb_bus = UsbBus::new(usb, unsafe { &mut EP_MEMORY });

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .strings(&[StringDescriptors::default()
            .manufacturer("Fake company")
            .product("Enumeration test")
            .serial_number("TEST")])
        .unwrap()
        .device_class(0)
        .build();

    loop {
        if usb_dev.poll(&mut []) {}
    }
}
