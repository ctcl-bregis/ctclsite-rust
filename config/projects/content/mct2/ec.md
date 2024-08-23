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

SMEC controls the two buck-boost regulators for USB source power. The TPS65988 would communicate with SMEC to set the output voltages of the buck-boost regulators.


## IO Usage 
This is the IO used on SMEC.

### USB
A USB link between SMEC and the module is provided. Implementing the USB interface is trivial as it requires just two pins on SMEC and one of the many USB 2.0 interfaces on the module. The maximum bus speed is 12Mbit/s.

A possible use of the USB bus is to have SMEC act as an HID keyboard for sending keypresses with the programmable buttons on the keypad. Another use of the interface is for debugging and possibly flashing firmware.

### I2C
Three I2C interfaces are used

Following device addresses are 7-bit. 

#### Bus 1
Bus 1 shall always be online and is used for communicating with various SMBus devices.

- SDA - SYS_SMB_SDA
- SCL - SYS_SMB_SCL
- SMBus Alert - SYS_SMB_ALT

- TCA8418 Keypad Controller - 0x34
- Battery pack fuel gauge - TBD
- LTC4162-LAD battery charge controller - 0x68
- TPS65988 Type-C PD controller - 0x20
- MP8859 Buck-Boost converter for TCP0 source - 0x60
- MP8859 Buck-Boost converter for TCP1 source - 0x66
- I210AT Ethernet Controller - 0x49
- BMP384 Digital pressure sensor - 0x76

#### Bus 2
Bus 2 is dedicated to communication between SMEC and the TCP controllers on the Intel N100. 

Label names:
- SDA - SML_SDA
- SCL - SML_SCL
- SMBus Alert - SML_ALT

Addresses:
- TCP0 - 0x??
- TCP1 - 0x??

#### Bus 3
Bus 3 is dedicated to communication between the OS and SMEC.

Label names:
- SDA - SOM_SMB_SDA
- SCL - SOM_SMB_SCL
- SMBus Altert - SOM_SMB_ALT

Addresses: 
- LattePanda Mu SoM - TBD

### GPIO

Input: Input to SMEC from device
Output: Output from SMEC to device

- USB Load Switches
  - EUSB0_EN - Output
  - EUSB0_FLT - Input
  - EUSB1_EN - Output
  - EUSB1_FLT - Input
  - EUSB2_EN - Output
  - EUSB2_FLT - Input
  - EUSB3_EN - Output
  - EUSB3_FLT - Input
- USB Power Enable
  - USB_5V_EN - Output
- VSYS Power Switch
  - VSYS_SW_IN - Output
  - VSYS_SW_DEN - Output
  - VSYS_SW_IS - Analog Input
- TCP_PPHV1 Regulator
  - TCP_PPHV1_EN - Output
- TCP_PPHV2 Regulator
  - TCP_PPHV2_EN - Output
- VSYS_LV Regulator
  - VSYS_LV_PG - Input
- USB_5V Regulator
  - USB_5V_PG - Input
- Keypad
  - KP_RST - Output
  - KP_SW_RAD - Input
  - KP_SW_CAM - Input
- OLED_VDD Regulator
  - OLED_VDD_EN - Output
- OLED_VOLED Regulator
  - OLED_VOLED_EN - Output
- System
  - SLS_S0 - Input
  - SLS_S3 - Input
  - PWRBTN - Output
  - RSTBTN - Output
- HDMI Companion IC
  - HDMI_CT_HPD - Output
  - HDMI_LS_OE - Output

### Flexible Memory Controller
The STM32 FMC interface is used by the OLED display on the side of the case. 

## Power
*See [Battery and Power Management](../power/) page for details on system power management*

It is critical that SMEC uses very little power, especially when the system is powered off. 

## System Communication
As mentioned above, there are two interfaces that SMEC can use to communicate with the OS: SMBus and USB.

It is expected that a custom daemon or Linux kernel module would have to be written for the system to make use of SMEC. I would not implement driver support for Microsoft Windows (but, of course, the project is open source so one can add support for Windows if they really wanted to).

This daemon or driver would send ACPI events for other daemons such as [acpid](https://linux.die.net/man/8/acpid) to make use of. 

## Software
As of August 21, 2024, I have not decided if SMEC would use FreeRTOS or the firmware would run "bare metal".

## Misc
This section covers miscellaneous details about the embedded controller.

### Clock Source

#### Oscillators
Instead of using a conventional crystal oscillator, SMEC would make use of MEMS oscillators. Using this kind of oscillator has the benefits of MEMS-based oscillators along with easier implementation since they just need a single passive component being the power supply decoupling capacitor. I have had an interest in MEMS oscillator technology since late 2018 and this is not the first time I have used them in a design.

Currently, there are two external oscillators used for SMEC:
- HSE: 48MHz SiTime SiT1602BI-13-XXN-48.000000
- LSE: 32.768KHz SiTime SiT1533AI-H4-DCC-32.768

Internal clock generators HSI, MSI and LSI are left unused. 

#### STM32 Clock Configuration
The 48MHz clock input directly goes to the ARM core clock (SYSCLK), resulting in SMEC running at a clock speed of 48MHz instead of advertised maximum of 80MHz. As seen with the [Flipper Zero that runs its STM32WB55 at 64MHz (Cortex-M4 core)](https://docs.flipper.net/development/hardware/tech-specs), 48MHz should be plenty for what is planned to be done with SMEC.

The 32.768KHz LSE clock input directly goes to the RTC (Real-Time Clock)

