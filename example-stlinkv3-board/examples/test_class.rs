#![no_std]
#![no_main]

use panic_rtt_target as _;

use cortex_m_rt::entry;
use stm32f7xx_hal::prelude::*;
use stm32f7xx_hal::pac;
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
    let dp = pac::Peripherals::take().unwrap_or_else(|| loop { continue; });

    restore(&cp, &dp);

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        .hse(HSEClock::new(25.MHz(), HSEClockMode::Bypass))
        .sysclk(72.MHz())
        .freeze();

    #[cfg(feature = "fs")]
    let gpioa = dp.GPIOA.split();
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

    let mut test = TestClass::new(&usb_bus);

    let mut usb_dev = { test.make_device(&usb_bus) };

    loop {
        if usb_dev.poll(&mut [&mut test]) {
            test.poll();
        }
    }
}
