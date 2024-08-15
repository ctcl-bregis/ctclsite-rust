Since I have started considering this project in May 2024, I have been aware of the fact that it would be unavoidable for myself or someone else to have to make a port of Coreboot for the LattePanda Mu as the provided firmware images are closed-source. Porting Coreboot to the Mu allows for others to modify the firmware to their needs.

## Development
I do not plan on starting development of a Coreboot port for the LattePanda Mu and project-specific configurations until I have an assembled prototype of the device. The Lite Carrier that I was provided does not have any way to access the SPI interface for an external SPI flash IC for storing the BIOS. 

MediaCow Touch 2 will have an SOIC-8 socket on-board connected to the external BIOS SPI interface that would make development siginificantly easier. 