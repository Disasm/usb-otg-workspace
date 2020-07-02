#![no_std]

#![allow(unused)]

use stm32f7xx_hal::device;
use rtt_target::rprintln;

pub fn restore(cp: &cortex_m::Peripherals, dp: &device::Peripherals) {
    for r in cp.NVIC.icer.iter() {
        unsafe { r.write(0xffffffff); }
    }

    let rcc = &dp.RCC;

    // Turn on HSI
    rcc.cr.modify(|_, w| w.hsion().set_bit());
    while rcc.cr.read().hsirdy().bit_is_clear() {}

    // Switch to HSI
    rcc.cfgr.modify(|_, w| w.sw().hsi());
}

fn setup_usbfs_clock() {
    let rcc = unsafe { &*device::RCC::ptr() };

    // Turn PLL off
    rcc.cr.modify(|_, w| w.pllsaion().off());
    // Wait till PLL is disabled
    while !rcc.cr.read().pllsairdy().is_not_ready() {}

    let cfg = rcc.pllcfgr.read();
    let pllm = cfg.pllm().bits() as u32;
    let plln = cfg.plln().bits() as u32;
    rprintln!("PLL settings: m={}, n={}, p={:#b}, q={}", pllm, plln, cfg.pllp().bits(), cfg.pllq().bits());
    rprintln!("VCO: {}", 25_000_000 * plln / pllm);
    //rprintln!("RCC pllm: {:#x}", rcc.pllcfgr.read().pllm().bits());

    // If HSE is provided
    // Configure PLL from HSE
    rcc.pllsaicfgr.write(|w| unsafe {
        w.pllsain().bits(100);
        w.pllsaip().div4()
    });
    // This setup gives 48.076923 MHz clock provided that pllm is 13

    // Enable PLL
    rcc.cr.modify(|_, w| w.pllsaion().on());
    // Wait for PLL to stabilise
    while rcc.cr.read().pllsairdy().is_not_ready() {}

    // Take 48MHz clock from PLLSAI
    rcc.dckcfgr2.modify(|_, w| w.ck48msel().pllsai());
}

fn setup_usbfs_clock_72_2() {
    let rcc = unsafe { &*device::RCC::ptr() };

    // let cfg = rcc.pllcfgr.read();
    // let pllm = cfg.pllm().bits() as u32;
    // let plln = cfg.plln().bits() as u32;
    // rprintln!("PLL settings: m={}, n={}, p={:#b}, q={}", pllm, plln, cfg.pllp().bits(), cfg.pllq().bits());
    // rprintln!("VCO: {}", 25_000_000 * plln / pllm);
    // //rprintln!("RCC pllm: {:#x}", rcc.pllcfgr.read().pllm().bits());
    //
    // rcc.pllcfgr.modify(|_, w| unsafe { w.pllq().bits(6) });
    // // 48.076923 MHz

    let cfg = rcc.pllcfgr.read();
    let pllm = cfg.pllm().bits() as u32;
    let plln = cfg.plln().bits() as u32;
    rprintln!("PLL settings: m={}, n={}, p={:#b}, q={}", pllm, plln, cfg.pllp().bits(), cfg.pllq().bits());

    // Take 48MHz clock from the main PLL
    rcc.dckcfgr2.modify(|_, w| w.ck48msel().pll());
}
