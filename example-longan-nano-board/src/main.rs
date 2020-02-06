#![no_std]
#![no_main]

use panic_halt as _;

use riscv_rt::entry;
use gd32vf103xx_hal as hal;
use hal::pac as pac;
use gd32vf103xx_hal::prelude::*;
use gd32vf103xx_hal::gpio::{Input, Floating, gpioa::{PA11, PA12}};

use synopsys_usb_otg::{UsbBus, UsbPeripheral};
use usb_device::prelude::*;

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

#[allow(dead_code)]
struct Peripheral {
    pin_dm: PA11<Input<Floating>>,
    pin_dp: PA12<Input<Floating>>,
}

unsafe impl UsbPeripheral for Peripheral {
    const REGISTERS: *const () = 0x50000000 as *const ();

    const HIGH_SPEED: bool = false;
    const FIFO_DEPTH_WORDS: usize = 320;

    fn enable() {
        let rcu = unsafe { (&*pac::RCU::ptr()) };

        riscv::interrupt::free(|_| {
            // Enable USB peripheral
            rcu.ahben.modify(|_, w| w.usbfsen().set_bit());

            // Reset USB peripheral
            rcu.ahbrst.modify(|_, w| w.usbfsrst().set_bit());
            rcu.ahbrst.modify(|_, w| w.usbfsrst().clear_bit());
        });
    }
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();

    // Configure clocks
    let rcu = dp.RCU.constrain();
    let clocks = rcu.cctl
        .ext_hf_clock(8.mhz())
        .sysclk(96.mhz())
        .freeze();

    assert!(clocks.usbclk_valid());

    let usb = Peripheral {
        pin_dm: gpioa.pa11,
        pin_dp: gpioa.pa12,
    };

    let usb_bus = UsbBus::new(usb, unsafe { &mut EP_MEMORY });

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Enumeration test")
        .serial_number("TEST")
        .device_class(0)
        .build();

    loop {
        if usb_dev.poll(&mut []) {
        }
    }
}
