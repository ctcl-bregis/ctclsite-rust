SVCS, nicknamed "Dracula's Castle", is the name given to any platform that I use for virtualization.

# Current
In early May 2023, I was given an HP BladeSystem C3000 chassis with three HP ProLiant BL460c G8 systems installed along with many other computer parts.

The HP BladeSystem setup is sometimes referred to as "The Amphetamines" in relation to the codename scheme specific to the server blades.

## SVCS1
SVCS1 uses the platform designated "Levoamphetamine" (shorthand "LVA"). The system is currently used for general purpose virtualization.

Specifications:

- System: HP ProLiant BL460c G8
- CPUs: 2x Intel Xeon E5-2697 v2
- Memory: 128GB (8x16GB) Micron Technology PC3-12800R 2Rx4 Registered ECC
- Storage: 1x HGST 600GB 10000RPM 6Gb/s SAS

The codename for this device was formerly "Methamphetamine" but was changed to "Levoamphetamine" on February 6, 2024 as a guideline was put into place for codenames. See [Week 6, 2024 blog post](../../blog/8/).

## SVCS2
SVCS2 uses the hardware platform "Lisdexamfetamine" (shorthand "LDX"). This is the oldest BL460c G8 in use. From June 8, 2023 to October 9, 2023, the system reported a motherboard error that was later discovered to just be the BIOS version not supporting the Intel Xeon Ivy Bridge CPUs that I attempted to install on June 8, 2023. After the original E5-2650 Sandy Bridge CPUs were installed, the system operated as normal. This configuration may be used indefinitely.

Currently, as of February 21, 2024, the system is used for ancillary virtualization services including [SLAG](../slag/) and a small-scale Minecraft server.

Specifications:

- System: HP ProLiant BL460c G8
- CPUs: 2x Intel Xeon E5-2650
- Memory: 128GB (8x16GB) SK hynix PC3-12800R 2Rx4 Registered ECC
- Storage: 1x HGST 600GB 10000RPM 6Gb/s SAS

## SVCS3
SVCS3 uses the hardware platform "Dextroamphetamine" (shorthand "DXA"). I could not actually get the system to boot from the hard drive. Since SVCS1 "Levoamphetamine" and SVCS2 "Lisdexamfetamine" are more than enough for my current needs, I have not put this system in production yet.

On February 11, 2024, the E5-2643 v2 CPUs where switched for E5-2667 v2 CPUs due to the higher core count and slight increase in single-threaded performance. It may be used for remote code compilation if I start to use the Rust programming language again.

Specifications:

- System: HP ProLiant BL460c G8
- CPUs: 2x Intel Xeon E5-2667 v2
- Memory: 64GB (8x8GB) Nanya Technology NT4GC72B4PB2NL-DI PC3-12800R 1Rx4 Registered ECC
- Storage: 1x HGST 600GB 10000RPM 6Gb/s SAS

# Former
From February 2023 to the introduction of the systems above, the motherboard ["Polyethylene"](../pc_pe/) in a Supermicro chassis was used.

Before "Polyethylene", since late 2020, various 1U/2U Dell PowerEdge and HP ProLiant series servers were used.