//! USB OTG high-speed peripheral
//!
//! Requires the `usb_hs` feature.
//! Only one of the `usb_fs`/`usb_hs` features can be selected at the same time.
//!
//! Note that only full-speed mode is supported,
//! external high-speed PHY is not supported.

use stm32f4xx_hal::pac;

use stm32f4xx_hal::gpio::{
    gpioa::{PA3, PA5},
    gpiob::{PB0, PB1, PB10, PB11, PB12, PB13, PB5},
    gpioc::PC0,
    gpioh::PH4,
    gpioi::PI11,
    Alternate, PushPull,
};
use stm32f4xx_hal::time::Hertz;

pub use synopsys_usb_otg::UsbBus;
use synopsys_usb_otg::{PhyType, UsbPeripheral};

pub struct USB {
    pub usb_global: pac::OTG_HS_GLOBAL,
    pub usb_device: pac::OTG_HS_DEVICE,
    pub usb_pwrclk: pac::OTG_HS_PWRCLK,
    pub phy_data0: PA3<Alternate<PushPull, 10>>,
    pub phy_data1: PB0<Alternate<PushPull, 10>>,
    pub phy_data2: PB1<Alternate<PushPull, 10>>,
    pub phy_data3: PB10<Alternate<PushPull, 10>>,
    pub phy_data4: PB11<Alternate<PushPull, 10>>,
    pub phy_data5: PB12<Alternate<PushPull, 10>>,
    pub phy_data6: PB13<Alternate<PushPull, 10>>,
    pub phy_data7: PB5<Alternate<PushPull, 10>>,
    pub phy_stp: PC0<Alternate<PushPull, 10>>,

    pub phy_nxt: PH4<Alternate<PushPull, 10>>,
    pub pin_dir: PI11<Alternate<PushPull, 10>>,

    pub pin_clk: PA5<Alternate<PushPull, 10>>,
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
            rcc.ahb1enr.modify(|_, w| w.otghsen().set_bit());

            // Enable ULPI clock
            rcc.ahb1enr.modify(|_, w| w.otghsulpien().set_bit());

            // Reset USB peripheral
            rcc.ahb1rstr.modify(|_, w| w.otghsrst().set_bit());
            rcc.ahb1rstr.modify(|_, w| w.otghsrst().clear_bit());
        });
    }

    fn ahb_frequency_hz(&self) -> u32 {
        self.hclk.0
    }

    fn phy_type(&self) -> PhyType {
        PhyType::ExternalHighSpeed
    }
}
