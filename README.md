# `usb-otg-workspace`

> A collection of examples for [`synopsys-usb-otg`](https://github.com/stm32-rs/synopsys-usb-otg).

## Running the examples

### STM32F429ZI ([NUCLEO-F429ZI](https://www.st.com/en/evaluation-tools/nucleo-f429zi.html) board)
```bash
rustup target add thumbv7em-none-eabihf
cd example-f429zi-board
openocd -f openocd.cfg &
cargo run --release --features "fs" --example serial
cargo run --release --features "fs" --example test_class
```

### STM32F446RE ([NUCLEO-F446RE](https://www.st.com/en/evaluation-tools/nucleo-f446re.html) board)
```bash
rustup target add thumbv7em-none-eabihf
cd example-f446re-board
openocd -f openocd.cfg &
cargo run --release --features "fs" --example serial
cargo run --release --features "fs" --example test_class
```

### STM32F446ZE ([NUCLEO-F446ZE](https://www.st.com/en/evaluation-tools/nucleo-f446ze.html) board)
```bash
rustup target add thumbv7em-none-eabihf
cd example-f446ze-board
openocd -f openocd.cfg &
cargo run --release --features "fs" --example serial
cargo run --release --features "fs" --example test_class
```

### STM32F401CC ([STM32F401 Development Board](https://www.aliexpress.com/item/4000069263843.html) from AliExpress)
```bash
rustup target add thumbv7em-none-eabihf
cd example-f401-board
openocd -f openocd.cfg &
cargo run --release --example serial
cargo run --release --example test_class
```

### STM32F407VG ([STM32F407G-DISC1](https://www.st.com/en/evaluation-tools/stm32f4discovery.html) board)
```bash
rustup target add thumbv7em-none-eabihf
cd example-f407-board
openocd -f openocd.cfg &
cargo run --release --example serial
cargo run --release --example test_class
```

### STM32F407IG ([XCore407I](https://www.waveshare.com/xcore407i.htm) board)
```bash
rustup target add thumbv7em-none-eabihf
cd example-xcore407i-board
openocd -f openocd.cfg &
cargo run --release --example serial --features "fs"
cargo run --release --example test_class --features "fs"
```
