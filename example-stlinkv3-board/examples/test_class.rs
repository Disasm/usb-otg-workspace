#![no_std]
#![no_main]

use panic_rtt_target as _;

use cortex_m_rt::entry;
use stm32f7xx_hal::prelude::*;
use stm32f7xx_hal::device;
use stm32f7xx_hal::rcc::{HSEClock, HSEClockMode};
#[cfg(feature = "fs")]
use stm32f7xx_hal::otg_fs::{USB, UsbBus};
#[cfg(feature = "hs")]
use stm32f7xx_hal::otg_hs::{USB, UsbBus};
use usb_device::test_class::TestClass;
use example_stlinkv3_board::restore;

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    let cp = cortex_m::Peripherals::take().unwrap_or_else(|| loop { continue; });
    let dp = device::Peripherals::take().unwrap_or_else(|| loop { continue; });

    restore(&cp, &dp);

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        .hse(HSEClock::new(25.mhz(), HSEClockMode::Bypass))
        .sysclk(72_200.khz())
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

    let mut test = TestClass::new(&usb_bus);

    let mut usb_dev = { test.make_device(&usb_bus) };

    loop {
        if usb_dev.poll(&mut [&mut test]) {
            test.poll();
        }
    }
}
