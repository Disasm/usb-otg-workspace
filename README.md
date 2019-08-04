# `usb-otg-workspace`

> Experimental [usb-device](https://github.com/mvirkkunen/usb-device) implementation for STM32F429 (OTG FS part)

## Running the examples

```bash
cd stm32f429-usbd-fs
cargo run --release --example serial
cargo run --release --example enumerate
cargo run --release --example test_class
```

Note that `serial_rtfm` and `serial_interrupt` examples are not ported at the moment.
