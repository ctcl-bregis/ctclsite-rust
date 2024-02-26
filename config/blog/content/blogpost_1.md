Since 2020, I have been exploring the potential uses for the SP7021 System in Package from Sunplus Technology and Tibbo Technology.

In 2020, I have acquired a Banana Pi F2P Single Board Computer utilizing the SP7021. At first I planned to use the board for data acquistion on the NetKart automotive project but I later decided to use it for development purposes only.

# What is the SP7021
The [SP7021](https://www.sunplus.com/products/plus1.asp) is a System in Package device. The package contains two dies, one for the DDR3 DRAM and one for the ARM processor cores. Depending on the version, the SP7021 either ships with a 4 Gbit DDR3 die for 512 MByte of RAM or an 1 Gbit DDR3 die for 128 MByte of RAM.

So far, the SP7021 SIP appears to be the only Linux-capable processor that is in a TQFP package and has integrated DRAM.

The IC is available from:

- [DigiKey](https://www.digikey.com/en/products/detail/tibbo/SP7021-IF/13918547)
- [Tibbo Technology](https://tibbo.com/store/plus1.html)

# Resources

## What I created
In May 2021, to teach myself about putting together my own embedded Linux images, I have made a [Debian 10 GNU/Linux image for the Banana Pi F2P board](https://forum.banana-pi.org/t/bpi-f2p-debian-10-armhf-linux-kernel-5-4-35/12286). This is my first ever attempt at making an embedded image so it may not be in the most usable of states.

Just recently, on July 10, 2023, I have made an EasyEDA footprint and symbol for the SP7021. I used the tool by [Wu Haotian (RigoLigoRLC) for converting the EasyEDA footprint to KiCAD](https://github.com/RigoLigoRLC/LC2KiCad).

The files are available here:

- [EasyEDA footprint](/static/blog/1/PCBLIB_TQFP-176_L20.0-W20.0-P0.40-EP9.6-BL_2023-07-10.json)
- [EasyEDA symbol](/static/blog/1/SCHLIB_SP7021_2023-07-10.json)
- [KiCAD symbol](/static/blog/1/SP7021.lib)

## Banana Pi

Banana Pi provides resources for development around the SP7021 with their wiki pages for the BPI-F2P and BPI-F2S Single Board Computer platforms.

- [BPI-F2S](https://wiki.banana-pi.org/Banana_Pi_BPI-F2S)
- [BPI-F2P](https://wiki.banana-pi.org/Banana_Pi_BPI-F2P)

## Sunplus Documentation

Sunplus Technology provides a public Atlassian Confluence wiki for documentation and resources for developing with the SP7021. There is also GitHub repositories available for building Linux and other kernels.

- [SP7021 Documentation](https://sunplus.atlassian.net/wiki/spaces/doc/overview)
- [sunplus-plus1 at GitHub](https://github.com/sunplus-plus1)
