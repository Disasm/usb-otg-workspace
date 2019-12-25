#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm32f4xx_hal::{prelude::*, stm32};
use stm32f4xx_hal::usb::{Peripheral, UsbBus};
use usb_device::test_class::TestClass;
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

    let gpioa = dp.GPIOA.split();

    let usb = Peripheral {
        usb_global: dp.OTG_FS_GLOBAL,
        usb_device: dp.OTG_FS_DEVICE,
        usb_pwrclk: dp.OTG_FS_PWRCLK,
        pin_dm: gpioa.pa11.into_alternate_af10(),
        pin_dp: gpioa.pa12.into_alternate_af10(),
    };

    let usb_bus = UsbBus::new(usb, unsafe { &mut EP_MEMORY });

    let mut test = TestClass::new(&usb_bus);

    let mut usb_dev = { test.make_device(&usb_bus) };

    loop {
        log::logger().flush();

        if usb_dev.poll(&mut [&mut test]) {
            test.poll();
        }
    }
}
