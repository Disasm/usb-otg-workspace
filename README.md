# `usb-otg-workspace`

> A collection of examples for [`synopsys-usb-otg`](https://github.com/stm32-rs/synopsys-usb-otg).

## Running the examples

```bash
cd example-f429zi-board
openocd -f openocd.cfg &
cargo run --release --features "fs" --example serial
cargo run --release --features "fs" --example test_class
```

```bash
cd example-f401-board
openocd -f openocd.cfg &
cargo run --release --example serial
cargo run --release --example test_class
```
