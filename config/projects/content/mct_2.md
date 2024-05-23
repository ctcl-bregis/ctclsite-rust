MediaCow Touch 2, codenamed "Paris", was an idea for a project about building an Android tablet but with custom PCB design.

I attempted to start working on the device's design in late 2021 to early 2022. The global chip shortage at the time made the project much more difficult to plan and develop. Along with difficulties with component sourcing, the project mainly suffered from my significant overestimation of my skills, this situation being called the Dunning-Kruger effect. 

The theming and codename is inspired by a certain online friend that I met in late 2020.

As of February 4, 2024, there may be a chance that I would start the development of MediaCow Touch 2 if my current software projects are successful.

# Hardware
MediaCow Touch 2's hardware plans has changed over time with the latest plan now using an x86-64 processor.

## System on Module
For MediaCow Touch 2 "Paris", I did not plan to design around the SoC itself and instead I made use of readily available System on Modules (SoM).

The carrier board, that the System on Module connects to, would be designed by myself. 

### Former ideas
These are former plans on what System on Module to use. 

#### Rockchip RK3399
In February 2022, I acquired the development board for [Shenzhen Graperain Technology's GR3399 "gold-finger" (MXM3.0 format) System on Module](https://www.graperain.com/ARM-Embedded-RK3399-Development-Board/). I used the GR3399 over the "Stamp hole" G3399 so during the assembly stage, I could just remove a couple screws to remove the module from the development kit and install the module into the prototype without having to desolder and risk damaging the module and/or development board.

The SoC (System on a Chip) used by the GR3399 System on Module is the Rockchip RK3399 featuring four ARM Cortex-A53 cores and two ARM Cortex-A72 cores in a big.LITTLE configuration.

#### Rockchip RK3588
Later in 2022, I found out that Banana Pi has developed multiple System on Module devices utilizing the more recent Rockchip RK3588 SoC.

It was more preferrable to use the RK3588 over the RK3399 for multiple reasons. 

I may end up using the 'BPI-RK3588 Gold finger interface core board' from Banana Pi for the project if I ever decide to start working on the device design again. The [system on module specifically is described](https://wiki.banana-pi.org/BPI-RK3588_Core_board_and_development_Kit) to use at least 2GB of LPDDR4X memory (typo as LPDDR4C on the wiki).

My current development board from Graperain using the RK3399 SoC may be used for another, similar project if not used for MediaCow Touch 2. 

### Current: Intel N100
In May 2024, I have found out about the [LattePanda Mu](https://www.lattepanda.com/lattepanda-mu) System on Module. This module uses an x86-64 processor instead of ARM64. [According to Intel](https://ark.intel.com/content/www/us/en/ark/products/231803/intel-processor-n100-6m-cache-up-to-3-40-ghz.html), the N100 is targeted towards use in mobile devices.

Using an x86 processor has its benefits including better software support and laptop-like performance.

## Carrier Board
The most complex part of the project is the custom carrier board for the compute module. The carrier board may have four to six layers. 

### Storage
The device may make use of an NVMe SSD using a standard M.2 connector for boot and user storage.

### Connectivity
A feature that I plan to have unique with MediaCow Touch 2 to off-the-shelf tablet devices is more physical connectors. 

#### Ports
Like MediaCow Touch "Nashville", the device would have full-size USB Type-A ports and an RJ-45 Ethernet jack.

Full list of connectors:

- At least one USB Type-A with USB 3.0 support
- Micro SD card slot
- RS232 connector
- RJ-45 10/100/1000 Ethernet
- Standard 3.5mm headphone jack
- **More TBD**

The device may have a single USB Type-C port for charging and data. It is most likely that I would implement USB Power Delivery for higher voltage charging.

#### Wi-Fi and Bluetooth
Like many laptop computers, especially older ones, MediaCow Touch 2 may provide Wi-Fi and Bluetooth only through standard M.2 modules.

This allows the Wi-Fi and Bluetooth radios to be swapped or disconnected for user security and privacy.

### Audio
As mentioned under Ports, the device would include a 3.5mm headphone jack. 

A separate audio CODEC IC would be used, there are many options for this IC and currently one has not been chosen yet.

### Camera and Microphone
There may be one or two cameras connected with USB to the system on module. These cameras may be found in existing laptop devices. The camera modules found in laptops often have microphones.

There would be hardware shut-off switches for the cameras and microphones.

## Display
Since 2022, I planned to use a 10.1 inch display with capacitive touch and a resolution of at least 1280x800.

Likely the display would have an LVDS interface and there would be a bridge IC on the carrier board for converting DisplayPort to LVDS. ICs such as the PTN3460I and ADV7613 can be used for this purpose. The display currently chosen is the Riverdi RVT101HVLFWCA0. I prefer to use a 10.1 inch display over 7" or 11.6". 

## Case
The current plan is to use white 3D printed PETG for the case.

# Software

## Operating System
With the Intel N100 CPU, there is the possibility of having a dual-boot configuration.

## BIOS
Likely I would make the effort to port Coreboot to the LattePanda Mu as currently, the [BIOS images provided are closed source](https://github.com/LattePandaTeam/LattePanda-Mu/tree/main/Softwares/BIOS). The main concern with these BIOS images being closed source is that there would be no ability to have a customized boot splash image, custom hardware layouts and other features that I would have to tweak for use in a tablet device. 