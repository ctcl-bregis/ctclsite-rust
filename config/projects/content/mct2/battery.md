
## Introduction
Battery and power management has historically been the main challenge for these projects.

## Battery Management
My goal for MediaCow Touch 2's design is to feel safe about traveling with the device. MediaCow Touch "Nashvlle" was actually dangerous to travel with due to how the battery was mounted.

### Battery Pack
After the extensive search for an off-the-shelf battery pack that would be suitable for this project, I eventually decided to use a replacement battery meant for a specific laptop. Making use of a laptop battery usually means that a critical part, the fuel gauge IC, is integrated in the battery pack along with protection and cell balancing circuitry. As result, the only IC needed to make use of specifically the battery is the charger.

Currently, the battery pack that would be used is one that is compatible with the HP FM08 battery pack. The FM08 uses either a 12-pin or 16-pin connector, MediaCow Touch 2 is designed for use with the 16-pin version.

The layout is 4S2P; 4-serial, 2-parallel. This would likely yield a voltage range of 12v to 16.8v. The actual voltage range of the battery pack is unknown and I am unsure what target per-cell voltage the battery charger IC should be set to. I do not have any laptop that makes use of the battery pack so I am unable to measure the voltages myself. Due to the fact I do not have one of these battery packs on hand, the pinout and connector type is also unknown.

### Battery Charger Subsystem
The battery charger IC selected is the LTC/Analog Devices LTC4162-LAD. 


## Power Management
*See [Embedded Controllers](../ec/) for details on the microcontroller used*

The carrier board makes extensive use of power load switches. 

