//! USB OTG high-speed peripheral
//!
//! Requires the `usb_hs` feature.
//! Only one of the `usb_fs`/`usb_hs` features can be selected at the same time.
//!
//! Note that only full-speed mode is supported,
//! external high-speed PHY is not supported.

use stm32f4xx_hal::pac;

use stm32f4xx_hal::gpio::alt::otg_hs as alt;
use stm32f4xx_hal::time::Hertz;

pub use synopsys_usb_otg::UsbBus;
use synopsys_usb_otg::{PhyType, UsbPeripheral};

pub struct USB {
    pub usb_global: pac::OTG_HS_GLOBAL,
    pub usb_device: pac::OTG_HS_DEVICE,
    pub usb_pwrclk: pac::OTG_HS_PWRCLK,
    pub phy_data0: alt::UlpiD0,
    pub phy_data1: alt::UlpiD1,
    pub phy_data2: alt::UlpiD2,
    pub phy_data3: alt::UlpiD3,
    pub phy_data4: alt::UlpiD4,
    pub phy_data5: alt::UlpiD5,
    pub phy_data6: alt::UlpiD6,
    pub phy_data7: alt::UlpiD7,
    pub phy_stp: alt::UlpiStp,
    pub phy_nxt: alt::UlpiNxt,
    pub pin_dir: alt::UlpiDir,
    pub pin_clk: alt::UlpiCk,
    pub hclk: Hertz,
}

unsafe impl Sync for USB {}

unsafe impl UsbPeripheral for USB {
    const REGISTERS: *const () = pac::OTG_HS_GLOBAL::ptr() as *const ();

    const HIGH_SPEED: bool = true;
    const FIFO_DEPTH_WORDS: usize = 1024;

    const ENDPOINT_COUNT: usize = 6;

    fn enable() {
        let rcc = unsafe { &*pac::RCC::ptr() };

        cortex_m::interrupt::free(|_| {
            // Enable USB peripheral
            rcc.ahb1enr().modify(|_, w| w.otghsen().set_bit());

            // Enable ULPI clock
            rcc.ahb1enr().modify(|_, w| w.otghsulpien().set_bit());

            // Reset USB peripheral
            rcc.ahb1rstr().modify(|_, w| w.otghsrst().set_bit());
            rcc.ahb1rstr().modify(|_, w| w.otghsrst().clear_bit());
        });
    }

    fn ahb_frequency_hz(&self) -> u32 {
        self.hclk.raw()
    }

    fn phy_type(&self) -> PhyType {
        PhyType::ExternalHighSpeed
    }
}
