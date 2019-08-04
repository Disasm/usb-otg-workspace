# `usb-otg-workspace`

> Experimental [usb-device](https://github.com/mvirkkunen/usb-device) implementation for STM32F429 (OTG FS part)

## Running the examples

```bash
cd example-f429zi-board
openocd -f openocd.cfg &
cargo run --release --example serial
cargo run --release --example enumerate
cargo run --release --example test_class
```
