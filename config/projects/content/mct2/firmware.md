

## SMEC
The development of SMEC's firmware is expected to be a great feat on its own.

### Programming Language
As of September 5, 2024, I still have not decided to use either Rust or C to write the firmware. I have more experience with the latter, Rust though C is officially supported for the STM32 series.

### Display Controller
The OLED display on the side of the case makes use of the FMC (Flexible Memory Controller) of the STM32L496. This exposes the display as a memory-mapped device. 

## Intel I210 Ethernet Controller
An SPI flash IC may be used with the Intel I210 for configuration data.

A Schottky diode is used on the power input of the SPI flash in order to allow the use of an SOIC-8 clip to program the flash directly as it is not socketed. 

## Texas Instruments TPS65988 PD Controller
The carrier board has the option for adding a SPI flash memory to the TPS65988 in case it is needed. The pads may be left unpopulated if the flash IC is not needed.

A Schottky diode is used on the power input of the SPI flash in order to allow the use of an SOIC-8 clip to program the flash directly as it is not socketed. 

## System BIOS
Since May 2024, I have expected to work on porting coreboot to the Mu.
