/* Linker script for building examples for the STM32F407IG */
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 1M
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}
