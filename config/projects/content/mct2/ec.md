## Introduction
To keep things more simple, I would make use of the popular STMicroelectronics STM32 series.

Until July 30, 2024, I planned to use two microcontrollers named "SMEC" and "PMEC" for System Management and Power Management, respectively. Now, I plan to use just one microcontroller. The name "SMEC" was reused for the one microcontroller on the carrier board. Hereinafter, the embedded controller would be refered to as just SMEC.

The STM32CubeMX software will be used to plan the IO layout for the device. 

## Microcontroller Selection
There was a overwhelmingly large amount of microcontrollers to select from though I eventually decided to use the STMicroelctronics STM32L4P5ZGT due to its high IO count, high performance and low power usage.

The STM32L4P5ZGT uses a single ARM Cortex-M33 CPU core with a clock speed up to 160MHz. It has a large amount of SRAM and flash available that is most likely not going to be nearly used to its full extent. 

## Functions

### OLED display
The OLED display recently introduced to the design would be driven by SMEC. The display is the main reason why I chose a higher performance microcontroller over something like the STM32L4 series. 

### Button Pad
The button pad, outlined in the [OLED Display and Button Panel](../oled/) page.

### Battery Management

## IO Usage 

- I2C Bus 1: Dedicated to keypad controller and must be active at all times
  - Keypad controller
- I2C/SMBus Bus 2: General use
  - Battery pack fuel gauge
  - LTC4162-LAD battery charge controller
  - TPS65994AD Type-C PD controller
  - IO expander for USB load switch control
  - IO expander or driver for charge indicator LED
- I2C/SMBus Bus 3: Dedicated to SMLink communication between SMEC and the TCP (Type C) controllers on the Intel N100
  - TCPx controllers 

## Power
It is critical that SMEC uses very little power, especially when the system is powered off. 

SMEC is powered from an LDO using system power.

## System Communication
It is expected that a custom daemon or Linux kernel driver would have to be written for the system to make use of SMEC. 

## Misc

### Clock Source
Instead of using a conventional crystal oscillator, SMEC would make use of MEMS oscillators. Using this kind of oscillator has the benefits of MEMS-based oscillators along with easier implementation.

Internal clock generators HSI, MSI and LSI are left unused.

Currently, there are two external oscillators used for SMEC:
- HSE: 48MHz SiTime SiT1602BI-13-XXN-48.000000
- LSE: 32.768KHz SiTime SiT1533AI-H4-DCC-32.768
