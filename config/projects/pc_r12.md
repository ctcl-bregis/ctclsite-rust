"Dichlorodifluoromethane" (shorthand "R12") is the small Intel desktop system currently used as SIGN1 with the current main purpose of running the [ToDoKiosk software](../todokiosk) to display tasks on a monitor on the bookshelf next to my desk.

This system was put into production on May 13, 2023 while the system was officially given a codename on January 6, 2024.

<figure>
    <img src="/static/projects/r12_todokiosk.webp" alt="&quot;Dichlorodifluoromethane&quot; running ToDoKiosk 0.3.2 - January 6, 2024 taken on ASUS Zenfone 9, edited in GIMP">
    <figcaption>"Dichlorodifluoromethane" running ToDoKiosk 0.3.2 - January 6, 2024 taken on ASUS Zenfone 9, edited in GIMP</figcaption>
</figure>

## Configuration
Current specifications as of January 6, 2024:

- Motherboard: ASRock AD2550B-ITX
- CPU: Intel Atom D2550 @ 1.86GHz
- Memory: 2GB (1x2GB) DDR3-1066 non-ECC SODIMM Qimonda IMSH2GS13A1F1C-1 (Presumably Nanya A-die through Inotera)
- Storage: 128GB Micron Technology RealSSD M550 SATA SSD
- Power Supply: 430W Thermaltake 80+
- Fan: 120mm fan from a DIYPC Silence-BK-Window case
- Case: Captec Custom Mini-ITX
- OS: Debian 11 + XFCE
- Monitor: HannStar Display HANNS.G HL193ABBUFWK1


The use of an x86 system over a common ARM-based single-board computers such as ones from Raspberry Pi or Banana Pi was because these boards often lack a VGA video output. The monitor has a VGA port for its only input. Though I could have just used a video converter, this was done more for fun.

The power usage, while not actually have been measured, is expected to be very low compared to other desktop systems making this system suitable for embedded, single-use applications such as running ToDoKiosk.