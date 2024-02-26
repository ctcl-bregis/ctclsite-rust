MediaCow Touch 2, codenamed "Paris", was an idea for a project about building an Android tablet but with custom PCB design.

I attempted to start working on the device's design in late 2021 to early 2022. The global chip shortage at the time made the project much more difficult to plan and develop. Along with difficulties with component sourcing, the project mainly suffered from my significant overestimation of my skills, this situation being called the Dunning-Kruger effect. 

The theming and codename is inspired by a certain online friend that I met in late 2020.

As of February 4, 2024, there may be a chance that I would start the development of MediaCow Touch 2 if my current software projects are successful.

# Hardware

## System on Module
For MediaCow Touch 2 "Paris", I did not plan to design around the SoC itself and instead I made use of readily available System on Modules (SoM).

The carrier board, that the System on Module connects to, would be designed by myself. 

### Original - RK3399
In February 2022, I acquired the development board for [Shenzhen Graperain Technology's GR3399 "gold-finger" (MXM3.0 format) System on Module](https://www.graperain.com/ARM-Embedded-RK3399-Development-Board/). I used the GR3399 over the "Stamp hole" G3399 so during the assembly stage, I could just remove a couple screws to remove the module from the development kit and install the module into the prototype without having to desolder and risk damaging the module and/or development board.

The SoC used by the GR3399 System on Module is the Rockchip RK3399 SoC (System on a Chip) featuring four ARM Cortex-A53 cores and two ARM Cortex-A72 cores in a big.LITTLE configuration.

### Later Idea - RK3588
Later in 2022, I found out that Banana Pi has developed multiple System on Module devices utilizing the more recent Rockchip RK3588 SoC.

It was more preferrable to use the RK3588 over the RK3399 for multiple reasons. 

I may end up using the 'BPI-RK3588 Gold finger interface core board' from Banana Pi for the project if I ever decide to start working on the device design again. The [system on module specifically is described](https://wiki.banana-pi.org/BPI-RK3588_Core_board_and_development_Kit) to use at least 2GB of LPDDR4X memory (typo as LPDDR4C on the wiki).

My current development board from Graperain using the RK3399 SoC may be used for another, similar project if not used for MediaCow Touch 2. 

## Storage
Along with the eMMC storage on the System on Modules described above, I may add support for a M.2 format SSD using the PCIe interface provided by the RK3588 and RK3399.

## Display
Since 2022, I planned to use a 10.1 inch display with a MIPI DSI interface, capacitive touch and a resolution of at least 1280x800.

## Case
Former plans from 2021 to 2022 described the case being 3D printed from white PETG (polyethylene terephthalate glycol) plastic.

## Connectivity

### Ports
Like MediaCow Touch "Nashville", the device would have full-size USB Type-A ports and an RJ-45 Ethernet jack.

Full list of connectors:

- At least one USB Type-A with USB 3.0 support
- At least one USB Type-A with USB 2.0 support
- Micro SD card slot
- Debug UART (serial)
- RJ-45 10/100/1000 Ethernet
- Standard 3.5mm headphone jack

The device may have a single USB Type-C port for charging and data. It is most likely that I would implement USB Power Delivery for higher voltage charging.

### Wi-Fi and Bluetooth
Like many laptop computers, especially older ones, MediaCow Touch 2 may provide Wi-Fi and Bluetooth only through a standard M.2 module connected to the system by likely USB or another interface.

This allows the Wi-Fi and Bluetooth radios to be swapped or disconnected for user security and privacy.

## Camera
MediaCow Touch 2 may include a single back-facing camera connected using the MIPI CSI interface.

# Software
Like MediaCow Touch "Nashville", the device would run the Android mobile operating system with a version that is at least Android 12. 

Most likely, a customized Android image would have to be built for use on the device.