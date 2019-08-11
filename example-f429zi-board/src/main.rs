#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm32f4xx_hal::{prelude::*, stm32};

use stm32f429_usbd_fs::UsbBus;
use usb_device::prelude::*;
use log::info;

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

    let gpiod = dp.GPIOD.split();
    stm32_log::configure(dp.USART3, gpiod.pd8, gpiod.pd9, 115_200.bps(), clocks);
    log::set_max_level(log::LevelFilter::Trace);

    info!("starting");

    #[cfg(feature = "fs")]
    let usb_peripherals = (dp.OTG_FS_GLOBAL, dp.OTG_FS_DEVICE, dp.OTG_FS_PWRCLK);
    #[cfg(feature = "hs")]
    let usb_peripherals = (dp.OTG_HS_GLOBAL, dp.OTG_HS_DEVICE, dp.OTG_HS_PWRCLK);

    #[cfg(feature = "fs")]
    let usb_pins = {
        let gpioa = dp.GPIOA.split();
        let usb_dm = gpioa.pa11.into_alternate_af10();
        let usb_dp = gpioa.pa12.into_alternate_af10();
        (usb_dm, usb_dp)
    };
    #[cfg(feature = "hs")]
    let usb_pins = {
        let gpiob = dp.GPIOB.split();
        let usb_vbus = gpiob.pb13.into_alternate_af12();
        let usb_dm = gpiob.pb14.into_alternate_af12();
        let usb_dp = gpiob.pb15.into_alternate_af12();
        (usb_vbus, usb_dm, usb_dp)
    };

    let usb_bus = UsbBus::new(usb_peripherals, usb_pins, unsafe { &mut EP_MEMORY });

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x5824, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(0)
        .build();

    loop {
        log::logger().flush();

        if usb_dev.poll(&mut []) {
        }
    }
}
