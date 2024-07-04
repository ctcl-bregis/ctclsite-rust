## Introduction
This is the first post of a series of blog posts about the development of MediaCow Touch 2 "Paris".

### Background
MediaCow Touch 2 "Paris" is a project to build a tablet computer device using custom circuit design (PCBs). The idea existed since March 2021 but not until May-June 2024, I have actually planned to design and build it. 

## The LattePanda Mu
On July 1, 2024, I received the LattePanda Mu System on Module (SoM) along with its Lite Carrier Board. I did not expect the LattePanda to be as small as it is with a surface area smaller than Raspberry Pi format boards.

On July 2, 2024, I have assembled the carrier board with its acrylic plate and the LattePanda Mu. As part of the Codename Guide, the hardware, the SoM specifically, received the codename "Cyclobutane".

When I booted up the LattePanda Mu, it was much more powerful than i expected it to be. It came with Windows 11 which I kept briefly to test gaming performance. This was the first time I ever used Windows 11 on my hardware and the first time I have used it for more than 10 minutes at a time.

During this short period of time, I tested two games: ULTRAKILL and OneShot, which are realitively lightweight. Both games ran flawlessly at 60 FPS, ULTRAKILL ran better on the Intel N100 than the dual Intel Xeon X5670 and AMD Radeon RX 580 in ["Polyethylene"](../../projects/wbpc_pe/). OneShot's performance was not surprising it is a top-down 2D game made with RPG Maker XP. 

The former hardware that I planned on using for the device, the Graperain GR3399 using the Rockchip RK3399 ARM System on Chip (SoC), would have not been nearly as powerful as the LattePanda Mu. 

## Challenges
MediaCow Touch 2 is an incredibly ambitious project for someone with no formal education in the field and [minimal experience with circuit design](../../projects/mathpad/). 

### Display
As of July 3, 2024, I still await the 11.6" display along with the extra heatsinks that I ordered separately on June 30, 2024. The display is expected to be a panel made by BOE. It is still the lowest cost option for the project and it is verified to function with the LattePanda Mu specifically. The expected date of delivery is July 8, 2024 according to DHL. 

As a size comparison, I looked at my Samsung XE500C13 Chromebook ("Chromium Sulfate") that also has an 11.6" display and determined that 11.6" is well suitable for the project. 7" was already too small for the original MediaCow Touch "Nashville" project so the lower cost 7" eDP display also provided by DFRobot for the LattePanda board series was not considered.

### Battery
Battery management has historically (since the original MediaCow idea in 2018), for myself, been the most difficult part of developing these devices.

#### Charging
Finding a charger IC for the project was easy and there are many options from both Texas Instruments and Analog Devices/Maxim Integrated for use in specifically Laptop and Tablet PC systems.

#### Packs and Fuel Gauges
Battery selection still remains a major challenge. It appears that many laptops and other devices of this class with 4S battery packs do not have the battery fuel gauge on the system and instead the fuel gauge IC is located on the battery pack itself. 

There seems to be a minimal amount of 4S1P/4S2P battery packs that have the fuel gauge built in that are not meant for a specific laptop. I have considered developing with an internal battery pack out of a ThinkPad but most of the ThinkPad systems that I have with internal battery packs use a 3S configuration that are not a high enough voltage for use with the Mu. 

There are fuel gauge ICs that support 4S configurations and can be used "system-side" but accuracy is possibly a concern with these ICs (I do not know if it makes any difference). 




