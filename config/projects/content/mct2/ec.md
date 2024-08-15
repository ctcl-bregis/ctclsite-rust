## Introduction
To keep things more simple, I would make use of the popular STMicroelectronics STM32 series.

Until July 30, 2024, I planned to use two microcontrollers named "SMEC" and "PMEC" for System Management and Power Management, respectively. Now, I plan to use just one microcontroller. The name "SMEC" was reused for the one microcontroller on the carrier board. Hereinafter, the embedded controller would be refered to as just SMEC.

The STM32CubeMX software will be used to plan the IO layout for the device. 

## Microcontroller Selection
There was a overwhelmingly large amount of microcontrollers to select from though I eventually decided to use the STMicroelctronics STM32L496VGT6 due to its high IO count, high performance and low power usage while having a lower cost than other, similar options.

## Functions

### OLED display
The OLED display recently introduced to the design would be driven by SMEC. SMEC uses the Intel 8080 parallel interface for communication with the OLED display. The parallel interface can be used with the STM32's FMC (Flexible Memory Controller) feature.

### Button Pad
The button pad, outlined in the [OLED Display and Button Panel](../oled/) page.

### Battery Management
When the system is powered on or off, SMEC would constantly read data from the battery pack, USB PD IC and charger IC. The latter two likely not as often as they are just used during charging. 

### USB PD management
The critical functions of USB Power Delivery is handled by the TPS65988. SMEC communicates with this chip for port status and control. 

## IO Usage 
This is the IO used on SMEC.

### I2C
Three I2C interfaces are used

- I2C Bus 1: Dedicated to keypad controller and must be active at all times
  - Keypad controller
- I2C/SMBus Bus 2: General use
  - Battery pack fuel gauge
  - LTC4162-LAD battery charge controller
  - TPS65988 Type-C PD controller
  - 2x MP8859 Buck-Boost DC-DC converters
  - I210 Ethernet transceiver
- I2C/SMBus Bus 3: Dedicated to SMLink communication between SMEC and the TCP (Type C) controllers on the Intel N100
  - TCPx controllers 
- I2C Bus 4: Communication between the SoM and SMEC

Following device addresses are 7-bit. 

#### Bus 1

- SDA - SMB1_SDA
- SCL - SMB1_SCL
- SMBus Alert - SMB1_ALT

- TCA8418 Keypad Controller - 0x34

#### Bus 2

Label names:
- SDA - SMB2_SDA
- SCL - SMB2_SCL
- SMBus Alert - SMB2_ALT

Addresses:
- Battery pack fuel gauge - TBD
- LTC4162-LAD battery charge controller - 0x68
- TPS65988 Type-C PD controller - 0x20
- MP8859 Buck-Boost converter for TCP0 source - 0x60
- MP8859 Buck-Boost converter for TCP1 source - 0x66
- I210AT Ethernet Controller - 0x49

#### Bus 3

Label names:
- SDA - SMB3_SDA
- SCL - SMB3_SCL
- SMBus Alert - SMB3_ALT

Addresses:
- TCP0 - 0x??
- TCP1 - 0x??

#### Bus 4

Label names:
- SDA - SOM_SMB_SDA
- SCL - SOM_SMB_SCL
- SMBus Alert - SOM_SMB_ALT

Addresses: 
- LattePanda Mu SoM - TBD

### GPIO

Total: 23

- USB Load Switches
  - 4 Input
  - 4 Output
- USB Power Enable
  - 1 Output
- VSYS Power Switch
  - 2 Output
  - 1 Analog Input
- TCP_PPHV1 Regulator
  - 1 Input
  - 1 Output
- TCP_PPHV2 Regulator
  - 1 Input
  - 1 Output
- Keypad Controller
  - 1 Input
- OLED_VDD Regulator
  - 1 Output
- OLED_VOLED Regulator
  - 1 Output
- System sleep states
  - 2 Inputs
- HDMI Companion IC
  - 2 Outputs


### Flexible Memory Controller
The STM32 FMC interface is used by the OLED display on the side of the case. 

## Power
*See [Battery and Power Management](../power/) page for details on system power management*

It is critical that SMEC uses very little power, especially when the system is powered off. 

## System Communication
It is expected that a custom daemon or Linux kernel module would have to be written for the system to make use of SMEC. I would not implement driver support for Microsoft Windows (but, of course, the project is open source so one can add support for Windows if they really wanted to).

This daemon or driver would send ACPI events for other daemons such as [acpid](https://linux.die.net/man/8/acpid) to make use of. 

## Misc
This section covers miscellaneous details about the embedded controller.

### Clock Source
Instead of using a conventional crystal oscillator, SMEC would make use of MEMS oscillators. Using this kind of oscillator has the benefits of MEMS-based oscillators along with easier implementation since they just need a single passive component being the power supply decoupling capacitor. I have had an interest in MEMS oscillator technology since late 2018 and this is not the first time I have used them in a design.

Currently, there are two external oscillators used for SMEC:
- HSE: 48MHz SiTime SiT1602BI-13-XXN-48.000000
- LSE: 32.768KHz SiTime SiT1533AI-H4-DCC-32.768

#### STM32 Clock Configuration
Internal clock generators HSI, MSI and LSI are left unused. 

The 48MHz clock input directly goes to the ARM core clock (SYSCLK), resulting in SMEC running at a clock speed of 48MHz instead of advertised maximum of 80MHz. As seen with the [Flipper Zero that runs its STM32WB55 at 64MHz (Cortex-M4 core)](https://docs.flipper.net/development/hardware/tech-specs), 48MHz should be plenty for what is planned to be done with SMEC.

The 32.768KHz LSE clock input directly goes to the RTC (Real-Time Clock)