#![no_std]
#![no_main]

use panic_rtt_target as _;

use cortex_m_rt::entry;
#[cfg(feature = "hs")]
use example_xcore407i_board::otg_hs::{UsbBus, USB};
#[cfg(feature = "fs")]
use stm32f4xx_hal::otg_fs::{UsbBus, USB};
use stm32f4xx_hal::{pac, prelude::*};
use usb_device::test_class::TestClass;

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

#[entry]
fn main() -> ! {
    rtt_target::rtt_init_print!();

    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(168.MHz())
        .require_pll48clk()
        .freeze();

    let gpioa = dp.GPIOA.split();
    #[cfg(feature = "hs")]
    let gpiob = dp.GPIOB.split();
    #[cfg(feature = "hs")]
    let gpioc = dp.GPIOC.split();
    #[cfg(feature = "hs")]
    let gpioh = dp.GPIOH.split();
    #[cfg(feature = "hs")]
    let gpioi = dp.GPIOI.split();

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
        phy_data0: gpioa.pa3.into(),
        phy_data1: gpiob.pb0.into(),
        phy_data2: gpiob.pb1.into(),
        phy_data3: gpiob.pb10.into(),
        phy_data4: gpiob.pb11.into(),
        phy_data5: gpiob.pb12.into(),
        phy_data6: gpiob.pb13.into(),
        phy_data7: gpiob.pb5.into(),
        phy_stp: gpioc.pc0.into(),
        phy_nxt: gpioh.ph4.into(),
        pin_dir: gpioi.pi11.into(),
        pin_clk: gpioa.pa5.into(),
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
