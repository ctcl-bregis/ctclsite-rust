Starting with this blog post, I am no longer going to add more information to existing blog posts in this series except for grammar and typo fixes. As result, one can see how I learn while I develop the device. 

At this point, it is most likely I would not have a physical prototype of MediaCow Touch 2 prepared by August 26, 2024. My current goal for the project is to get a usable design posted on the LattePanda Forums by then. For the goal of building a working prototype, I plan to have such prototype built by November 20-24, 2024 (actual date in 2020 unknown since I lost all of the photos from the iPod Touch that I photographed it with); the fourth anniversary of the completion of MediaCow Touch 1.

Since MediaCow Touch 2 is open source hardware, there is the possibility that someone could make their own device. 

## Challenges

### USB Type-C
Implementing USB Type-C has been confusing but I feel as I am starting to get enough of an understanding to be comfortable with whatever design I come up with. 

### Battery Management
On July 24, 2024, I switched out the TI BQ25700A for the LTC/Analog Devices LTC4162-LAD. A major part of the complexity of the BQ25700A is that it is a *buck-boost* charger that can charge the battery off from voltages lower than the battery pack target voltage. I envisioned that MediaCow Touch 2 would charge off from a 20v USB PD power supply so the ability to charge off from, for example, 5 volts, is not needed.

Compared to the BQ25700A, the implementation is much less complex as seen by comparing the evaluation board for the [BQ25700A](https://www.ti.com/lit/ug/sluubg6/sluubg6.pdf) and the [LTC4162](https://www.analog.com/en/resources/evaluation-hardware-and-software/evaluation-boards-kits/dc2038a.html#eb-overview) series. 

<figure>
    <img src="/static/blog/mct2_p3/mct2_charge_system_preview_20240725.webp">
    <figurecaption>Battery charge circuit layout. Subject to change. - July 25, 2024</figurecaption>
</figure>

### Battery
The plan with the battery pack has not changed. Getting the pinout of the battery pack is crucial for completing the schematic and PCB design.

### Circuit Design
Circuit design, as expected, will take the longest amount of time.

#### MOSFETs
For the implementation of the LTC4162-LAD, I chose Infineon ISC019N03L5S MOSFETs. I must have looked at the wrong datasheet since I thought that part used the Infineon PG-T**S**DSON-8 package format that was not available in KiCAD so I ended up drawing a PG-TSDSON-8 footprint from scratch which took me up to an hour. After drawing the footprint, I found out the ISC019N03L5S that I am actually using uses the PG-TDSON-8 package that has a footprint already available in KiCAD's standard libraries. 

I would have the footprint I drew available though left unused by the project like many of the other symbols and footprints I have in the "mediacow" library in [the repository](https://github.com/ctcl-bregis/mct2/).


