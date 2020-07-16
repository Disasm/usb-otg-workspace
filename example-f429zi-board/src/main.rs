#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm32f4xx_hal::{prelude::*, stm32};
#[cfg(feature = "fs")]
use stm32f4xx_hal::otg_fs::{USB, UsbBus};
#[cfg(feature = "hs")]
use stm32f4xx_hal::otg_hs::{USB, UsbBus};
use usb_device::prelude::*;

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .require_pll48clk()
        .freeze();

    #[cfg(feature = "fs")]
    let gpioa = dp.GPIOA.split();
    #[cfg(feature = "hs")]
    let gpiob = dp.GPIOB.split();

    #[cfg(feature = "fs")]
    let usb = USB {
        usb_global: dp.OTG_FS_GLOBAL,
        usb_device: dp.OTG_FS_DEVICE,
        usb_pwrclk: dp.OTG_FS_PWRCLK,
        pin_dm: gpioa.pa11.into_alternate_af10(),
        pin_dp: gpioa.pa12.into_alternate_af10(),
        hclk: clocks.hclk(),
    };
    #[cfg(feature = "hs")]
    let usb = USB {
        usb_global: dp.OTG_HS_GLOBAL,
        usb_device: dp.OTG_HS_DEVICE,
        usb_pwrclk: dp.OTG_HS_PWRCLK,
        pin_dm: gpiob.pb14.into_alternate_af12(),
        pin_dp: gpiob.pb15.into_alternate_af12(),
        hclk: clocks.hclk(),
    };

    let usb_bus = UsbBus::new(usb, unsafe { &mut EP_MEMORY });

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(0)
        .build();

    loop {
        if usb_dev.poll(&mut []) {
        }
    }
}
