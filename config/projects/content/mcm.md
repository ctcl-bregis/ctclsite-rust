MediaCow Micro is a series of projects that I came up with in July 2021. It is a challenge of designing 26 different portable media player devices with each device using a microcontroller from a different vendor.

Each design is named in alphabetical order, from MediaCow Micro A to MediaCow Micro Z. The order that these devices are designed in are arbitrary and designs using more popular microcontrollers may be developed first for some ease in development as I would be learning while I work on these projects.

Due to lack of funding and the difficulty of sourcing some chips, a project would be considered "finished" when the entire design including the PCB and case is finalized instead of having the project finished when there is a working, physical prototype.

As covered in the [Codename Guide](../codenames/), these devices would use codenames of Central New York State.

# Hardware
Hardware greatly varies by device; some may have external DRAM, some may have coprocessors that handle audio decode, some may have displays while some do not.

In cases where these devices use a coprocessor, the "Microcontroller" is what is considered the main processor.


If a table cell is empty, the value is to be determined. 

Fields:

- Device: Device name
- Codename: Project codename
- Microcontroller: Main processor that loads media
- Architecture: CPU architecture
- MicroMemory: Support for MicroMemory modules
  - SPI PSRAM: Serial PSRAM module
  - SDRAM: Standard SDRAM module
  - N/A: None
- Status: Project status

| Device           | Codename | Microcontroller                   | Architecture   | MicroMemory    | Status         |
| ---------------- | -------- | --------------------------------- | -------------- | -------------- | -------------- |
| MediaCow Micro A |          | Atmel/Microchip ATSAME7/V7 series | ARM Cortex-M7  |                | Not Started    |
| MediaCow Micro B |          |                                   |                |                | Not Started    |
| MediaCow Micro C |          |                                   |                |                | Not Started    |
| MediaCow Micro D |          |                                   |                |                | Not Started    |
| MediaCow Micro E |          | Espressif ESP32-S3 series         | Xtensa LX7     |                | Not Started    |
| MediaCow Micro F |          | Freescale/NXP i.MXRT series       | ARM Cortex-M7  | SDRAM          | Not Started    |
| MediaCow Micro G |          | GigaDevice GD32H737ZIT6           | ARM Cortex-M7  | SDRAM          | Not Started    |
| MediaCow Micro H |          |                                   |                |                | Not Started    |
| MediaCow Micro I |          |                                   |                |                | Not Started    |
| MediaCow Micro J |          |                                   |                |                | Not Started    |
| MediaCow Micro K |          |                                   |                |                | Not Started    |
| MediaCow Micro L |          |                                   |                |                | Not Started    |
| MediaCow Micro M |          | Microchip PIC32MZ series          | MIPS32         |                | Not Started    |
| MediaCow Micro N |          | Nordic Semiconductor nRF5340      | ARM Cortex-M33 |                | Not Started    |
| MediaCow Micro O |          |                                   |                |                | Not Started    |
| MediaCow Micro P |          | Parallax Propeller 2              | Propeller      | SPI PSRAM      | Not Started    |
| MediaCow Micro Q | Verona   | Infineon XMC4400 series           | ARM Cortex-M4  | SDRAM          | Not Started    |
| MediaCow Micro R |          | Raspberry Pi RP2040               | ARM Cortex-M0+ | N/A            | Not Started    |
| MediaCow Micro S |          | STMicroelectronics STM32F767ZIT6  | ARM Cortex-M7  | SDRAM          | On Hold        |
| MediaCow Micro T |          | Texas Instruments TM4C series     | ARM Cortex-M4  | SDRAM          | Not Started    |
| MediaCow Micro U |          | Generalplus/Sunplus 8051          | 8051           | N/A            | Not Started    |
| MediaCow Micro V |          | VLSI Solution VS1005(g)           | ???            | N/A            | Not Started    |
| MediaCow Micro W |          | Nuvoton NUC505 series             | ARM Cortex-M4F |                | Not Started    |
| MediaCow Micro X |          | Xilinx/AMD Spartan FPGA           |                | DDR?           | Not Started    |
| MediaCow Micro Y |          | SyncMOS 8051                      | 8051           |                | Not Started    |
| MediaCow Micro Z |          | Zilog Z80 series                  | Z80            |                | Not Started    |
