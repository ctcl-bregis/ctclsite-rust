This document covers general information about the MediaCow series of mobile hardware projects.

## Origin of the name
The name "MediaCow" was first used in presumably November 2018 to describe a mobile portable media player device. Other ideas for the name were "MediaBarn" and "Digital Media Locker Portable (DMLP)". 

The name and theme of MediaCow is mostly inspired by my early childhood where I was still in Central New York within the Syracuse and Oneida areas. Cows were a common theme due to the dairy industry. A local business, Byrne Dairy had role in the theming of the device due to its prevalence in my early childhood. 

The design of the first device that used solid wood boards from 2018 to 2019 is likely inspired by a dream or part of a movie that I saw when I was between two (2006) and six years (2010) old where there was this device with the case made out of solid boards and nails and there was a picture of a pasture with Holstein cattle on screen.

The design choice of using real wooden boards was used up to MediaCow Touch (1) "Nashville" in November 2020. MediaCow Touch 2 would use PETG. 

## History

### Digital Media Locker
The idea of MediaCow started after the success of Digital Media Locker from September to October 2018. Digital Media Locker consisted of a 12v battery, Raspberry Pi 2 Model B, power inverter, various USB chargers, a 7" LCD, USB HDD dock and a HDMI LCD controller board taped inside a locker at my high school. Though simple and having an alarming appearance, it was praised by many.

After the required teardown in late October 2018, mostly about safety concerns with the lead-acid battery I was using, I later came up with an idea to fit all of the parts from the locker into a tablet-like device that ended up becoming MediaCow. 

### MediaCow (2019)
In late 2018, with my near-zero knowledge of electronics, I started to work on the original MediaCow. I first used small boards held together with screws in a rectangle shape and then put a transparent polycarbonate sheet over the screen. The case itself was very strong and could likely stayed together if thrown off a 10-story building. 

By 2019, attempts to put the Raspberry Pi 2 Model B into the case along with a battery pack, LCD and the LCD controller failed. I later substituted the Raspberry Pi for an Amazon FireTV stick which somehow worked and I was able to use the device for short periods of time without being plugged in. There was no audio so I had to use bluetooth for sound. I was able to watch the Castlevania anime series on it during this time. For myself in 2019, this was an amazing feat.

### MediaCow Touch (2020)
MediaCow Touch, later referred to as MediaCow Touch 1, is a project I started planning in October 2020. By this time, I had a somewhat better understanding of electronics and software after introducing myself to Linux early that year and my employment at 2nd Life. I still did not know nearly anything about circuit design, battery management or CAD so the design did not end up like I envisioned it but it was still somewhat useful. 

### 2021-2022: The Touch 2, Micros and Minis
From 2021 to 2022, with the success of MediaCow Touch 1, I came up with many different ideas for devices I could design. These ideas were unrealistic or had unrealistic deadlines for my limited skillset, especially at the time.

These ideas included:
- MediaCow Touch 2 and 3
- MediaCow Touch Mini series
- MediaCow Micro series

### MediaCow Touch 2 (2021-2024)
In May 2024, I have found out about the [LattePanda Mu](https://www.lattepanda.com/lattepanda-mu) System on Module. This module uses an x86-64 processor instead of an ARM64 SoC. [According to Intel](https://ark.intel.com/content/www/us/en/ark/products/231803/intel-processor-n100-6m-cache-up-to-3-40-ghz.html), the N100 is targeted towards use in mobile devices.

Using an x86 processor has its benefits including better software support and laptop-like performance.

Ideas for MediaCow Touch 2 date back to March 2021. The initial idea at the time surrounded the use of a soldered-down Rockchip RK3588 SoC with ten DDR4 DRAM ICs, specifically Nanya NT5AD1024M8A3, for 8GB of memory with ECC. This would have been almost impossible if not impossible for me to design at the time so I later decided to use a System on Module.

In late 2021 to early 2022, I decided to use the Graperain GR3399 System on Module with the Rockchip RK3399 and 2GB of DDR3 RAM (4x Samsung K4B4G1649E-BCMA). The development kit including the SoM was ordered in February 2022 but ended up never being used for the project. I preferred to use the MXM3.0 ("Gold Finger") format of the System on Module over the "Stamp hole" G3399 so during the assembly stage, I could just remove a couple screws to remove the module from the development kit and install the module into the prototype without having to desolder and risk damaging the module and/or development board.

Later in 2022, I found out about various System on Modules that became available from Banana Pi, who supplied the board for MediaCow Touch 1, that used the newer and more powerful Rockchip RK3588.

Throughout 2022 and 2023, I stopped working on hardware design projects due to their complexity and focused on software instead.

In May 2024, I signed up for the LattePanda Mu Free Trial Event to receive a LattePanda Mu System on Module along with its Lite Carrier Board. I did not expect the idea to be approved and for me to receive the development hardware. Compared to anything I have done before, this is significantly more complex. 