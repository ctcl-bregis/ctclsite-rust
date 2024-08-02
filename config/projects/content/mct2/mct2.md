## Introduction
MediaCow Touch 2, codenamed "Paris", is a project idea about designing and building a tablet computer device.

The theming and codename is inspired by a certain online friend that I met in late 2020.

### Disclaimer
The following is preliminary data and may change at any time during development. This document is used both to showcase my ideas for the device along being used as the plan.

### Brief History
For an in-depth history on MediaCow, see the [MediaCow page](../mediacow/).

#### 2021-2022
In March 2021, I came up with ideas to develop another tablet device after the success with MediaCow Touch "Nashville" in November 2020. At first, I had some overcomplicated, unrelastic ideas at the beginning. The initial idea at the time surrounded the use of a soldered-down Rockchip RK3588 SoC with ten DDR4 DRAM ICs, specifically Nanya NT5AD1024M8A3, for 8GB of memory with ECC. This would have been extremely difficult if not impossible for me to design at the time so I later decided to use a System on Module, a format of computers I just heard of recently at that time.

Block diagrams and physical layout diagrams have been attempted throughout the rest of 2021.

In late 2021 to early 2022, I decided to use the Graperain GR3399 System on Module with the Rockchip RK3399. In February 2022, I ordered the GR3399 Gold-Finger development kit from Graperain. The module was quite large and had 2GB of DDR3 RAM (4x Samsung K4B4G1649E-BCMA). I preferred to use the MXM3.0 ("Gold Finger") format of the System on Module over the "Stamp hole" G3399 so during the assembly stage, I could just remove a couple screws to remove the module from the development kit and install the module into the prototype without having to desolder and risk damaging the module and/or development board.

I attempted to start working on the device's design in late 2021 to early 2022. The global chip shortage at the time made the project much more difficult to plan and develop. Along with difficulties with component sourcing, the project mainly suffered from my overestimation of my skills. Basically, when I tried to start working on the project, I had no idea what I was doing then I gave up shortly after since there was no push for me to complete the project.

#### 2023
In 2023, for the most part, I decided to stop working on hardware and focus on software development due to the failed attempts at the project in prior years and there being no need for the device.

#### 2024
In May 2024, I heard about the LattePanda Mu Free Trial Event presumably through DFRobot on Discord. I signed up for the event with the idea being MediaCow Touch 2 with the idea that there was a tiny chance my idea would be accepted for receiving the development kit for free. On June 20, 2024, to my surprise, I received an email about interest in the project idea, asking for more information about how the module is used.

## Development
Development of MediaCow Touch 2 officially started on June 24, 2024.

MediaCow Touch 2 is the first hardware project that uses GitHub for peer review and syncing progress across devices. It is also the first device, with compute elements, that I would design in KiCAD instead of EasyEDA.

### Process
Having a proper order in how the device is developed is important for the success of the project.

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
   8. In-House Assembly
   9. Testing
3. Software
   1. Embedded controller firmware development
   2. Embedded controller firmware testing
   3. System firmware (BIOS) development
   4. System firmware testing
   5. Final software testing
4. Completion

It is cruical that I complete steps 1 through 2.6 by September 1, 2024, preferably by August 26, 2024.
