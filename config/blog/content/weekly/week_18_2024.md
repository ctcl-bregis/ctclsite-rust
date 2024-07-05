# Projects
This week I have been trying to balance software and hardware projects.

## Software
This week I have explored options for non-tabular (NoSQL) databases for use in projects along with text editors though licensing has been a concern.

## Hardware
I have worked on some hardware this week.

### EasyEDA to KiCad
This week, I have started to migrate from EasyEDA to KiCad for hardware development due to multiple concerns.

Another concern that I have had with EasyEDA is performance, the software seems to slow down once circuit designs get somewhat complicated. For example, likely on ["Polyethylene Terephthalate"](../../projects/pc_pet/) when it was using an Intel Xeon E5-1650 v2, EasyEDA slowed down when I was working on a former version of MediaCow Micro E (which I have later scrapped the design since I did not know what I was doing back then) that was using an ESP32-S3-WROOM series module. This is likely due to EasyEDA being browser-base. It is already impressive that it is able to run as smoothly in the first place, even on other systems like an ASUS C100P Chromebook that used a Rockchip RK3288. EasyEDA is better for smaller, hobby projects than large, complex designs though it is capable of the latter if one is patient. 

A concern that I also have is long-term use. EasyEDA is an internet service and may become unavailable in the future. KiCad is open source software that runs locally on the client system. 

Another concern is footprint and symbols, EasyEDA has a massive amount of footprints and symbols available while I find myself having to draw them myself in KiCad. This has been the largest holdback for my transistion to KiCad. However, I have seen footprints/symbols disappear from EasyEDA when the product is removed from LCSC, for example, the Ingenic X2000 and X1000 series.

### Flipper Zero Audio Module
The first design I would do in KiCad would be redesigning the Flipper Zero Audio Module. From May 3, 2024 to May 4, 2024, I have completed the schematic and most of the PCB design while improving both. This was a quick any easy process since I looked over what I had on EasyEDA and the design is quite simple to begin with.

# Personal
This was an uneventful week for myself personally.