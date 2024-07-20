## Feature Overview

This section covers what features are available to the device.

### System

The device makes use of the LattePanda Mu System on Module (SoM). The LattePanda Mu was released recently, in early-mid 2024 for US$139. The LattePanda also uses a
The specific unit that I received that is planned to be used in the first prototype has the following specifications:

- [Intel Processor N100](https://ark.intel.com/content/www/us/en/ark/products/231803/intel-processor-n100-6m-cache-up-to-3-40-ghz.html) x86-64 CPU with 6MB of cache and integrated Intel UHD Graphics
- 8GB of LPDDR5-4800 memory with what appears to be a single [Samsung K3KL3L30CM-BGCT](https://semiconductor.samsung.com/dram/lpddr/lpddr5x/k3kl3l30cm-bgct/) 64Gbit LPDDR5X-7500
- 64GB of storage provided by a single Samsung eMMC flash memory IC

The system on module provides more than enough processing power for what I expect to use the device for. The module has markings stating that it was made during Week 9 of 2024.

### Display

The current plan is to use an 11.6" 1920x1080 touchscreen display.

### Storage

Like many laptops and handhelds like the Valve Steam Deck, an M.2 SSD is used for storage. Unlike some laptops and handhelds there is support for SSD sizes up to 2280. One M.2 Key-M slot would be available for an NVMe SSD. The SSD would make use of four PCIe 3.0 lanes.

The 64GB eMMC on the module may be left unused in most use cases.

### Connectors

Summary of external connectors:

- 2x USB Type-A 2.0 connected directly to the SoM
- 4x USB Type-A 3.0 5Gb (provided by a PCIe-USB3 host IC; not from the CPU)
- 1x HDMI full-size port for external monitors
- 1x RJ-45 port for 10/100/1000 Ethernet
- 1x USB Type-C PD charging port
- 3.5mm audio output

Though details are to be determined, there may be the addition of a USB Type-C port with DisplayPort alternate mode capability alongside the USB Type-A ports.

### Wireless Connectivity

Wi-Fi and Bluetooth would be provided by an M.2 module just like many laptops.

The ability to remove this module and as result, have zero wireless connectivity, is a crucial privacy and security feature. Along with the ability to remove the module, there would be a switch to disable wireless connectivity during device operation.

### Cameras and Microphone

The device would have a front-facing camera in the form of a webcam and a back-facing camera likely using the on-module MIPI CSI interface.

Like wireless, a switch is provided to disable the webcam and microphone entirely. This may be done with

### Audio

A standard 3.5mm headphone jack would be provided. The addition of line-in and/or line-out would likely be determined by what audio IC is used.

There may be the addition of two speakers in the device for cases where this would be useful such as video calls.

### Power

MediaCow Touch 2 would contain a battery pack for mobile use.

## Hardware Details

This section goes into depth about how hardware features described in the above sections are implemented.

The PC-like hardware of the MediaCow Touch 2 has many features easy to implement as I can look at existing laptop and tablet PC designs while there are many ICs availble made specifically for what I am trying to achieve.

### Power


### Camera and Microphone

Like many modern smartphones, the device would have a front-facing camera and back-facing camera.

#### Front-Facing Camera and Microphone

The current plan is to use a webcam pulled from a laptop/Chromebook that has a microphone on the module. The camera would communicate with the module over USB while the microphone would have a PDM signal output that would be used by the audio CODEC. The use of an existing webcam simplifies the addition of a front-facing camera.

If possible, a power switch controlled by SMEC for the webcam would be added. This would shut off power to the webcam.

#### Back-Facing Camera

Unlike the front-facing camera, the back-facing camera would connect to the module through the MIPI CSI-2 interface.

The pinout of the MIPI CSI-2 FPC connector on the LattePanda Mu appears to be compatible with the 22-pin camera interface on some Raspberry Pi boards. This would have camera selection much easier as many Raspberry Pi cameras would be electrically compatible with the LattePanda Mu.

There are various camera modules that can be used. Preferably, the back camera would use a module with a resolution of 8MP or higher.

### Embedded Controllers

Like many laptops, including Chromebooks, MediaCow Touch 2 makes use of microcontrollers separate from the main CPU/SoC called an embedded controller. Though, the carrier board makes use of two microcontrollers.

PMEC was known as BMEC up to July 15, 2024. SMEC was known as IOEC up to July 17, 2024

#### SMEC

SMEC, known as System Management Embedded Controller, is a microcontroller that controls load switches for USB ports, reads from the button panel and controls power for wireless connectivity and the webcam.

As this microcontroller is expected to be online only when the rest of the system is online, power usage of the microcontroller is not much of a concern.

#### PMEC

PMEC, known as Power Management Embedded Controller, is a microcontroller that directly manages the battery charger IC, reads data from the battery pack and manages power states of the system.

A requirement of the specific microcontroller part used as PMEC is very low power usage as the microcontroller is always online even when the device is powered off. An STMicroelectronics STM32L4-series microcontroller may be used for this purpose as it is known to support very low power states.

Two of the buttons, Power and Reset are connected to PMEC instead of the IO expander connected to IOEC. This is because PMEC must know the state of these buttons at all times.

PMEC also monitors the state of SLS_S0 and SLS_S3 on the module. On the

According to the schematic for the Lite Carrier for LattePanda Mu, the power state is represented by the state of SLS_S0 and SLS_S3:


| SLS_S0 | SLS_S3 | Power State    |
| -------- | -------- | ---------------- |
| High   | High   | S0 - Online    |
| Low    | High   | S3 - Sleep     |
| Low    | Low    | S4 - Hibernate |
| Low    | Low    | S4 - Offline   |

Another thing to note is that SLS_S0 is also referred to as PWR_LED and PSON in the schematic for the Lite Carrier.

### External Connectivity

#### USB

Every USB port has a load switch that is controlled and monitored by IOEC.

##### USB 3.0 Type-A

Four USB 3.0 Type-A ports are made available with a [Renesas uPD720201 USB 3.0 Host Controller IC](https://www.renesas.com/us/en/products/interface/usb-switches-hubs/upd720201-usb-30-host-controller). I originally considered using the [Texas Instruments TUSB7340](https://www.ti.com/product/TUSB7340) though sourcing of that part is difficult and more costly. I likely have experience with the specific Renesas part with a USB 3.0 card used in the desktop PC, ["Polyethylene"](../wbpc_pe/), under a Linux environment where it was functional out-of-box without any concern about drivers.

##### USB Type-C

There is the possibility that more than one USB Type-C port would be made available to the system. These USB Type-C ports would make use of the two USB 3.2 interfaces provided by the Intel N100 itself.

### Internal Connectivity

The LattePanda Mu has nine PCIe 3.0 lanes, eight USB 2.0 interfaces and four I2C that is available to the carrier board.

#### PCIe

PCIe is the main connection standard used for high-speed connections between components on the carrier board.

This is the expected assignemnt of PCIe lanes:


| Pin Names | Lane   | Function |
| ----------- | -------- | ---------- |
| HSIO2     | Lane 0 | NVMe     |
| HSIO3     | Lane 1 | NVMe     |
| HSIO5     | Lane 2 | NVMe     |
| HSIO6     | Lane 3 | NVMe     |
| HSIO7     | Lane 4 | Wi-Fi    |
| HSIO8     | Lane 5 | USB 3    |
| HSIO9     | Lane 6 | Ethernet |
| HSIO10    | Lane 7 | N/A      |
| HSIO11    | Lane 8 | N/A      |

HSIO0-HSIO1 is used for USB 3.2 on the LattePanda Mu Lite Carrier Board while HSIO4 does not seem to exist.

#### USB

USB 2.0 connections are expected to be used for connections to some devices, namely the front-facing camera (webcam) and Bluetooth through the M.2 Key E slot.


| Interface | Function          |
| ----------- | ------------------- |
| USB2_P1   | USB Type-C Port 1 |
| USB2_P2   | USB Type-C Port 2 |
| USB2_P3   | USB 2 Type-A Port |
| USB2_P4   | USB 2 Type-A Port |
| USB2_P5   |                   |
| USB2_P6   |                   |
| USB2_P7   |                   |
| USB2_P8   |                   |

#### Other Connections

This section covers connections or pins with a specific purpose.

##### SUSCLK

The not so suspicious clock signal. Pin 131 on the Mu connector is SUSCLK. It appears to be used for a low-frequency clock signal. On the Lite Carrier schematic, it is described as: "32.76kHz clock for low power mode" and is connected to both M.2 slots through an optional resistor (0 ohm).

### Audio

A separate audio CODEC IC would be used. As of July 15, 2024, the CODEC IC currently selected is the Tempo Semiconductor 92HD65C. The audio codec would make use of the HD Audio interface provided by the Intel CPU.

It is currently unknown if the CODEC would be connected with HD Audio or with USB. Many devices with similar hardware use HD Audio or I2S for connecting the audio CODEC, for example, the Valve Steam Deck uses the [Cirrus Logic CS35L41B I2S/TDM audio DSP/Amplifier](https://www.cirrus.com/products/cs35l41/) according to an [iFixit guide](https://www.ifixit.com/Guide/Steam+Deck+Chip+ID/147811).

### Other

#### RTC Battery

A coin cell battery holder would be on the carrier board for powering the RTC. The battery format would be CR1210 3V.

## Software

This section covers software and firmware that the device would use.

### BIOS

As result of how module interfaces are used and the closed-source nature of the BIOS images provided by LattePanda, Coreboot is expected to be ported to the LattePanda Mu by myself.

For easier flashing of the BIOS, a SOIC-8 socket would be on the carrier board making use of the external SPI flash interface of the module. This would make loading BIOS easier as the SPI flash IC can be removed without desoldering and be flashed with another device.

### Operating System

Currently, the plan for the device is for it to use a customized Linux distribution.

