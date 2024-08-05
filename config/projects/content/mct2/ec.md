## Introduction
To keep things more simple, I would make use of the popular STMicroelectronics STM32 series.

Until July 30, 2024, I planned to use two microcontrollers named "SMEC" and "PMEC" for System Management and Power Management, respectively. Now, I plan to use just one microcontroller. The name "SMEC" was reused for the one microcontroller on the carrier board. Hereinafter, the embedded controller would be refered to as just SMEC.

The STM32CubeMX software will be used to plan the IO layout for the device. 

## Microcontroller Selection
There was a overwhelmingly large amount of microcontrollers to select from though I eventually decided to use the STMicroelctronics STM32L4P5ZGT due to its high IO count, high performance and low power usage.

The STM32L4P5ZGT uses a single ARM Cortex-M4 CPU core with a clock speed up to 120MHz. 

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
*See [Battery and Power Management](../power/) page for details on system power management*

It is critical that SMEC uses very little power, especially when the system is powered off. 

## System Communication
It is expected that a custom daemon or Linux kernel module would have to be written for the system to make use of SMEC. It is most likely that I would not implement support for Microsoft Windows (but, of course, the project is open source so one can add support for Windows if they really wanted to).

This daemon or driver would send ACPI events for other daemons such as [acpid](https://linux.die.net/man/8/acpid) to make use of. 

## Misc
This section covers miscellaneous details

### Clock Source
Instead of using a conventional crystal oscillator, SMEC would make use of MEMS oscillators. Using this kind of oscillator has the benefits of MEMS-based oscillators along with easier implementation since they just need a single passive component being the power supply decoupling capacitor. I have had an interest in MEMS oscillator technology since late 2018 - early 2019 and this is not the first time I have used them in a design.

Currently, there are two external oscillators used for SMEC:
- HSE: 48MHz SiTime SiT1602BI-13-XXN-48.000000
- LSE: 32.768KHz SiTime SiT1533AI-H4-DCC-32.768

#### STM32 Clock Configuration
Internal clock generators HSI, MSI and LSI are left unused. 

The 48MHz clock input directly goes to the ARM core clock (SYSCLK), resulting in SMEC running at a clock speed of 48MHz instead of advertised maximum of 120MHz. As seen with the [Flipper Zero that runs its STM32WB55 at 64MHz (Cortex-M4 core)](https://docs.flipper.net/development/hardware/tech-specs), 48MHz should be plenty for what is planned to be done with SMEC.

<figure>
    <img src="/static/projects/mct2/smec_clock_config_20240804_anno.png">
    <figcaption>Clock Configuration in STM32CubeMX</figcaption>
</figure>

The 32.768KHz LSE clock input directly goes to the RTC (Real-Time Clock)