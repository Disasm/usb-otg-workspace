/* Linker script for building examples for the STM32F446RE */
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}
