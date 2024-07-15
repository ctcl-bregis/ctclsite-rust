MediaCow Touch 2, codenamed "Paris", was an idea for a project about building an Android tablet but with custom PCB design.

I attempted to start working on the device's design in late 2021 to early 2022. The global chip shortage at the time made the project much more difficult to plan and develop. Along with difficulties with component sourcing, the project mainly suffered from my significant overestimation of my skills; the Dunning-Kruger effect.

The theming and codename is inspired by a certain online friend that I met in late 2020.

On June 24, 2024, development of MediaCow Touch 2 has started as part of the LattePanda Mu Free Trial Event.

## Disclaimers
The following is preliminary data and may change at any time during the development stage.

The following may require a technical knowledge in computer hardware.

## Hardware
MediaCow Touch 2's hardware plans has changed over time with the latest plan now using an x86-64 processor.

For an in-depth history on MediaCow, see the [MediaCow page](../mediacow/).

### System on Module
For MediaCow Touch 2 "Paris", I did not plan to design around the SoC itself and instead I made use of readily available System on Modules (SoM).

The carrier board, that the System on Module connects to, would be designed by myself.

Currently, the System on Module chosen for the project is the LattePanda Mu with the Intel N100.

### Carrier Board
The most complex part of the project is the custom carrier board for the compute module. The carrier board may have four to sx layers.

#### Embedded Controllers
Two embedded controllers in the form of microcontroller ICs may be present on the carrier board. Though this may change during the circuit design stage, the microcontrollers used may be ones part of the STMicroelectronics STM32 series due to their popularity and my experience with the STM32 series.

These microcontrollers run independently from the system on module

##### Power Management Embedded Controller
Known as the PMEC, this microcontroller connects directly to the battery fuel gauge and charger ICs.

Two of the buttons on the side of this device connect to this microcontroller instead of the IO expander connected to IOEC, Power and Reset. This is done as PMEC is always online even when the tablet is powered off. 

The requirement of the microcontroller IC used as PMEC is that it uses a very low amount of power since, as mentioned before, is always online. An STMicroelectronics **STM32L4**-series microcontroller may be used for this purpose.

PMEC would also control a single RGB LED next to the USB Type-C connector to indicate charge and power states.

##### IO Embedded Controller
IOEC, also known as the **IO Embedded Controller**, manages load switches for all USB Type-A ports, controls most buttons, Wi-Fi/BT power switch and the webcam power switch.

Unlike PMEC, this microcontroller may be active only when the system is powered on.

##### Storage
The device may make use of an NVMe SSD using a standard M.2 connector for boot and user storage.

Due to the size of the device, M.2 SSDs with lengths up to 2280 can be used while the option to have smaller SSDs persists like many computer motherboards.

##### PCIe
Up to five reference clock signals are available from the Mu and a total of nine PCIe lanes are available.

The assignment of PCIe lanes are as follows:

| Lane   | Function |
| ------ | -------- |
| Lane 0 | NVMe     |
| Lane 1 | NVMe     |
| Lane 2 | NVMe     |
| Lane 3 | NVMe     |
| Lane 4 | Wi-Fi    |
| Lane 5 | USB 3    |
| Lane 6 | Ethernet |
| Lane 7 | N/A      |
| Lane 8 | N/A      |

#### Connectivity
A feature that I plan to have MediaCow Touch 2 unquie to off-the-shelf tablet devices is more physical connectors. Like MediaCow Touch "Nashville", the device would have an RJ-45 Ethernet jack.

Summary:

- 1x USB Type-C + PD
- 4x USB Type-A 3.x
- 4x USB Type-A 2.0
- HDMI connector
- 1x 10/100/1000 Ethernet
- 3.5mm headphone output
- 3.5mm line in

Internal connectors include:

- M.2 Key E 2230/2242
- M.2 Key M 2230/2242/2260/2280

##### USB Type-C
The USB Type-C Power Delivery port would probably only be used for power though this may change in development.

##### USB Type-A ports

With the use of an xHCI host controller IC such as the TI TUSB7340, four USB 3 Type-A ports would be available to the user.

Four of the USB 2.0 links from the module would be available.

##### Wi-Fi and Bluetooth
Like many laptop computers, especially older ones, MediaCow Touch 2 may provide Wi-Fi and Bluetooth only through standard M.2 modules.

This allows the Wi-Fi and Bluetooth radios to be swapped or disconnected for user security and privacy.

#### Audio
As mentioned under Ports, the device would include a 3.5mm headphone jack.

A separate audio CODEC IC would be used. As of July 15, 2024, the CODEC IC currently selected is the Tempo Semiconductor 92HD65C.

It is currently unknown if the CODEC would be connected with HD Audio or with USB. Many devices with similar hardware use HD Audio or I2S for connecting the audio CODEC, for example, the Valve Steam Deck uses the [Cirrus Logic CS35L41B I2S/TDM audio DSP/Amplifier](https://www.cirrus.com/products/cs35l41/) according to an [iFixit guide](https://www.ifixit.com/Guide/Steam+Deck+Chip+ID/147811).

#### Camera and Microphone
There may be one or two cameras connected with USB to the system on module. These cameras may be found in existing laptop devices. The camera modules found in laptops often have microphones on the same PCB.

There would be hardware shut-off switches for the cameras and microphones.

### Display
Since 2022, I planned to use a 10.1 inch display with capacitive touch and a resolution of at least 1280x800. Though this changed later on in June 2024 to a 1920x1080 11.6" display.

In the diagram sent to the LattePanda team on June 20, 2024, the display mentioned is the [DFRobot 11.6" 1920x1080 eDP LCD](https://www.dfrobot.com/product-2794.html). Use of the DFRobot display would make development much easier and lower the implementation cost.

### Case
The current plan is to use white 3D printed PETG for the case. With the size of the display, the case cannot be printed in one piece on the Ender V2 printer that I have currently.

On July 8, 2024, I considered making the case out of plywood like I did with MediaCow Touch 1 due to the similar or better density of various species of wood. Wood would likely make the manufacture of the case parts more difficult. With wood being more difficult and wood is not part of theme (white PET plastic), it is more likely I would stick to the original plan of using 3D printed white PETG.

## Software
Software may be a complex task in the development of the device especially due to the amount of custom drivers required because of the embedded controllers.

### Operating System
Custom drivers may be required for some hardware on the device. This may have a Linux distribution like Linux Mint be used on the device instead of Windows. It is not too important that Windows can be used on the device as it is not expected for me to play much games or use Windows-specific software (e.g. Xactimate) on the device.

### BIOS
Likely I would make the effort to port Coreboot to the LattePanda Mu as currently, the [BIOS images provided are closed source](https://github.com/LattePandaTeam/LattePanda-Mu/tree/main/Softwares/BIOS). The main concern with these BIOS images being closed source is that there would be no ability to have a customized boot splash image, custom hardware layouts and other features that I would have to tweak for use in a tablet device.

Instead of flashing the chip on the Mu, a socket for standard SOIC-8 SPI EEPROM memory ICs would be on the carrier board.

## Development Process
Development officially started on June 20, 2024.

It is important that the case is designed *before* before circuit design as the case would determine the dimensions of the PCB.
