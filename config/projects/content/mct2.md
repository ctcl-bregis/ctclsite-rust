
## Introduction 
MediaCow Touch 2, codenamed "Paris", is a project idea about designing and building a tablet computer device. 

The theming and codename is inspired by a certain online friend that I met in late 2020.

### Disclaimer
The following is preliminary data and may change at any time during development. This document is used both to showcase my ideas for the device along being used as the plan.

### History
For an in-depth history on MediaCow, see the [MediaCow page](../mediacow/).

#### 2021-2022
In March 2021, I came up with ideas to develop another tablet device after the success with MediaCow Touch "Nashville" in November 2020. At first, I had some overcomplicated, unrelastic ideas at the beginning. The initial idea at the time surrounded the use of a soldered-down Rockchip RK3588 SoC with ten DDR4 DRAM ICs, specifically Nanya NT5AD1024M8A3, for 8GB of memory with ECC. This would have been extremely difficult if not impossible for me to design at the time so I later decided to use a System on Module, a format of computers I just heard of recently at that time.

Block diagrams and physical layout diagrams have been attempted throughout the rest of 2021. 

In late 2021 to early 2022, I decided to use the Graperain GR3399 System on Module with the Rockchip RK3399. In February 2022, I ordered the GR3399 Gold-Finger development kit from Graperain. The module was quite large and had 2GB of DDR3 RAM (4x Samsung K4B4G1649E-BCMA). I preferred to use the MXM3.0 ("Gold Finger") format of the System on Module over the "Stamp hole" G3399 so during the assembly stage, I could just remove a couple screws to remove the module from the development kit and install the module into the prototype without having to desolder and risk damaging the module and/or development board.

I attempted to start working on the device's design in late 2021 to early 2022. The global chip shortage at the time made the project much more difficult to plan and develop. Along with difficulties with component sourcing, the project mainly suffered from my overestimation of my skills. Basically, when I tried to start working on the project, I had no idea what I was doing. 

#### 2023
In 2023, for the most part, I decided to stop working on hardware and focus on software development due to the failed attempts at the project in prior years and there being no need for the device.

#### 2024
In May 2024, I heard about the LattePanda Mu Free Trial Event presumably through DFRobot on Discord. I signed up for the event with the idea being MediaCow Touch 2 with the idea that there was a+ tiny chance my idea would be accepted for receiving the development kit for free. On June 20, 2024, to my surprise, I received an email about interest in the project idea, asking for more information about how the module is used.

## Feature Overview
This section covers what features are available to the device.

### System
The device makes use of the LattePanda Mu System on Module (SoM). The LattePanda Mu was released recently, in early-mid 2024 for US$139. The processor it makes use of, the Intel Processor N100, was released in Q1 2023.

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

Like wireless, a switch is provided to disable the webcam and microphone entirely.

### Audio 
A standard 3.5mm headphone jack would be provided. The addition of line-in and/or line-out would likely be determined by what audio IC is used. 

There may be the addition of two speakers in the device for cases where this would be useful such as video calls.

### Power
Like all tablet devices, MediaCow Touch 2 would use a battery.

## Hardware
This section covers details about how hardware features described in the above section are implemented.

The PC-like hardware of the MediaCow Touch 2 has many features easy to implement as I can look at existing laptop and tablet PC designs while there are many ICs availble made specifically for what I am trying to achieve.

### Embedded Controllers
Like many laptops, including Chromebooks, MediaCow Touch 2 makes use of microcontrollers separate from the main CPU/SoC called an embedded controller.

#### IOEC
IOEC, known as IO Embedded Controller, is a microcontroller that controls load switches for USB ports, reads from the button panel and controls power for wireless connectivity and the webcam.

As this microcontroller is expected to be online only when the rest of the system is online, power usage of the microcontroller is not much of a concern.

#### PMEC
PMEC, known as Power Management Embedded Controller, is a microcontroller that directly manages the battery charger IC, reads data from the battery pack and manages power states of the system.

A requirement of the specific microcontroller part used as PMEC is very low power usage as the microcontroller is always online even when the device is powered off. An STMicroelectronics STM32L4-series microcontroller may be used for this purpose as it is known to support very low power states.

Two of the buttons, Power and Reset are connected to PMEC instead of the IO expander connected to IOEC. This is because PMEC must know the state of these buttons at all times.

### External Connectivity

#### USB
Every USB port has a load switch that is controlled and monitored by IOEC. 

##### USB 3.0 Type-A
Four USB 3.0 Type-A ports are made available with a [Renesas uPD720201 USB 3.0 Host Controller IC](https://www.renesas.com/us/en/products/interface/usb-switches-hubs/upd720201-usb-30-host-controller). I originally considered using the [Texas Instruments TUSB7340](https://www.ti.com/product/TUSB7340) though sourcing of that part is difficult and more costly. I likely have experience with the specific Renesas part with a USB 3.0 card used in the desktop PC, ["Polyethylene"](../wbpc_pe/), under a Linux environment where it was functional out-of-box without any concern about drivers. 

### Internal Connectivity

#### PCIe
PCIe is the main connection standard used for high-speed connections between components on the carrier board. 

This is the expected assignemnt of PCIe lanes:

| Pin Names  | Lane   | Function |
| ---------- | ------ | -------- |
| HSIO2      | Lane 0 | NVMe     |
| HSIO3      | Lane 1 | NVMe     |
| HSIO5      | Lane 2 | NVMe     |
| HSIO6      | Lane 3 | NVMe     |
| HSIO7      | Lane 4 | Wi-Fi    |
| HSIO8      | Lane 5 | USB 3    |
| HSIO9      | Lane 6 | Ethernet |
| HSIO10     | Lane 7 | N/A      |
| HSIO11     | Lane 8 | N/A      |

HSIO0-HSIO1 is used for USB 3.2 on the LattePanda Mu Lite Carrier Board while HSIO4 does not seem to exist.

#### USB
USB 2.0 connections are expected to be used for connections to some devices, namely the front-facing camera (webcam) and Bluetooth through the M.2 Key E slot.

USB 3.0 is not expected to be used for connections between internal components.

#### I2C and SMBus
I2C and SMBus are expected to be used extensively for communication between embedded controllers and some devices.

### Audio
A separate audio CODEC IC would be used. As of July 15, 2024, the CODEC IC currently selected is the Tempo Semiconductor 92HD65C. The audio codec would make use of the HD Audio interface provided by the Intel CPU. 

It is currently unknown if the CODEC would be connected with HD Audio or with USB. Many devices with similar hardware use HD Audio or I2S for connecting the audio CODEC, for example, the Valve Steam Deck uses the [Cirrus Logic CS35L41B I2S/TDM audio DSP/Amplifier](https://www.cirrus.com/products/cs35l41/) according to an [iFixit guide](https://www.ifixit.com/Guide/Steam+Deck+Chip+ID/147811).

## Software
This section covers software and firmware that the device would use.

### BIOS
As result of how module interfaces are used and the closed-source nature of the BIOS images provided by LattePanda, Coreboot is expected to be ported to the LattePanda Mu by myself.

For easier flashing of the BIOS, a SOIC-8 socket would be on the carrier board making use of the external SPI flash interface of the module. This would make loading BIOS easier as the SPI flash IC can be removed without desoldering and be flashed with another device. 

### Operating System
Currently, the plan for the device is for it to use a customized Linux distribution.

## Development Process
Development of MediaCow Touch 2 officially started on June 24, 2024. H

aving a proper order in how the device is developed is important for the success of the project.

On July 16, 2024, I realized that I was doing this project in the incorrect order by skipping directly to the case design and hardware block diagram while I should have been writing (typing) a plan for the project. As result, I came up with a process with the order of what is to be done.

This is the process in how I plan to design the device:

1. Brainstorming ideas in written form
2. Hardware
   1. Case design
   2. Hardware block diagram
   3. Schematic
   4. PCB
   5. Peer review of schematic and PCB
   6. Hardware design publishing
   7. Part acquisition
   8. Assembly
   9. Testing
3. Software
   1. Embedded controller firmware development
   2. Embedded controller firmware testing
   3. System firmware (BIOS) development
   4. System firmware testing
   5. Final software testing
4. Completion

It is cruical that I complete steps 1 through 2.6 by September 1, 2024, preferably by August 26, 2024. 

