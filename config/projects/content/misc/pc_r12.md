"Dichlorodifluoromethane" (shorthand "R12") is the small Intel desktop system currently used as SIGN1 with the current main purpose of running the [ToDoKiosk software](../todokiosk/) to display tasks on a monitor on the bookshelf next to my desk.

This system was put into production on May 13, 2023 while the system was officially given a codename on January 6, 2024.

On May 14, 2024, I have started to plan to have an ARM single-board computer take place of this system for showing the to-do list.

<figure>
    <img src="/static/projects/r12_todokiosk.webp">
    <figcaption>"Dichlorodifluoromethane" running ToDoKiosk 0.3.2 - January 6, 2024</figcaption>
</figure>

## Configuration
Current specifications as of January 6, 2024:

- Motherboard: ASRock AD2550B-ITX
- CPU: Intel Atom D2550 @ 1.86GHz
- Memory: 2GB (1x2GB) DDR3-1066 non-ECC SODIMM Qimonda IMSH2GS13A1F1C-1
- Storage: 128GB Micron Technology RealSSD M550 SATA SSD
- Power Supply: 430W Thermaltake 80+
- Fan: 120mm fan from a DIYPC Silence-BK-Window case
- Case: Captec Custom Mini-ITX
- OS: Debian 11 XFCE
- Monitor: HannStar Display HANNS.G HL193ABBUFWK1

The use of an x86 system over a common ARM-based single-board computers such as ones from Raspberry Pi or Banana Pi was because these boards often lack a VGA video output. The monitor has a VGA port for its only input. 
Though video converters exist for HDMI to VGA, being a much lower cost option than building a dedicated x86 system, this was done for more of a learning experience.

The power usage, while not actually have been measured, is expected to be very low compared to other desktop systems making this system suitable for embedded, single-use applications such as running ToDoKiosk.