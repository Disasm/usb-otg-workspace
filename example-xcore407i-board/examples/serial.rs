//! CDC-ACM serial port example using polling in a busy loop.
#![no_std]
#![no_main]

use panic_rtt_target as _;

use cortex_m_rt::entry;
#[cfg(feature = "hs")]
use example_xcore407i_board::otg_hs::{UsbBus, USB};
#[cfg(feature = "fs")]
use stm32f4xx_hal::otg_fs::{UsbBus, USB};
use stm32f4xx_hal::{pac, prelude::*};
use usb_device::device::StringDescriptors;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

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

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .strings(&[StringDescriptors::default()
            .manufacturer("Fake company")
            .product("Serial port")
            .serial_number("TEST")])
        .unwrap()
        .device_class(USB_CLASS_CDC)
        .max_packet_size_0(64)
        .unwrap()
        .build();

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 512];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                let mut write_offset = 0;
                while write_offset < count {
                    match serial.write(&buf[write_offset..count]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
