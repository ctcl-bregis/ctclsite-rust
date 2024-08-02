
## Wired Connectivity
A unique aspect of MediaCow Touch 2 is that it has connectors not commonly seen on tablets. 

* 2x USB Type-C with PD and DisplayPort alternate mode
* 2x USB 3.2 Type-A
* 4x USB 2.0 Type-A
* Mini-HDMI
* 10/100/1000 Ethernet RJ-45

### Wireless Connectivity
Like most tablet computers, MediaCow Touch 2 has Wi-Fi and Bluetooth capabilities. 

Wi-Fi and Bluetooth is provided through a M.2 module. This is an important security and privacy feature as wireless connectivity can be disabled altogether by removing the module. Along with modularity, there is a switch on the side of the device to shut off the module; a wireless kill switch.

### Cameras and Microphones
MediaCow Touch 2 would contain up to two cameras. 

#### Front-Facing Camera
The front-facing camera would be in the form of a laptop webcam pulled from either a Chromebook or a laptop. The front-facing camera along with the microphone would be useful in cases such as video chat, video logs and selfie photography (though the device would likely require two hands to hold).

Internally, the webcam module connects to the system through a USB 2.0 link though the microphone would have an output for a PDM signal that would be used by the audio CODEC IC on the carrier board.

#### Back-Facing Camera
The back-facing camera would be in the form of a camera module with a MIPI CSI interface. I discovered that the pinout on the LattePanda Mu is the same as the 22-pin camera connector found on some Raspberry Pi single-board computers. As result, I would most likely use a camera module intended for such model of Raspberry Pi.

Since the camera would be connected directly to the LattePanda Mu System on Module, it is most likely that the hardware kill switch for cameras and microphones would not effect the back-facing camera. This is done to simplify the design of the first carrier board revision. If the ability to disable the back-facing camera is needed in the future, there can be another board revision that uses the other MIPI CSI interface available to the carrier board.

### Storage
The device has a M.2 Key M slot with support for 2230, 2242, 2260 and 2280 size SSDs.

### Display
The display used in MediaCow Touch 2 is a 11.6" capacitive touchscreen with a resolution of 1920x1080. The display was provided by DFRobot and the panel itself likely manufactured by BOE. 

The display connects directly to the LattePanda Mu with eDP (Embedded DisplayPort). 

### Power
MediaCow Touch 2 makes use of an internal battery pack. The battery is charged through USB Type-C PD.

The battery pack would likely be one meant for an existing laptop. Currently, the battery pack planned to be used is an HP FM08 16-pin battery pack that is meant for older versions of the HP Omen 17 laptop. 

A goal for the power subsystem is for me to be sure that it is safe for air travel; I would have not brought MediaCow Touch 1 anywhere near an airport.

### Case
As part of the theme, the case would be made of white PET(G). The case would be 3D printed.
