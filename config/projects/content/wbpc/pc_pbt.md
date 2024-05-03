"Polybutylene Terephthalate" (shorthand "PBT") is the codename given to the desktop computer system that currently serves as WBPC1 for general purpose workloads. The system is considered to be the "main PC" and almost all work and entertainment (gaming) is done on it. It is normally used from eight (8) to fourteen (14) hours a day.

Unlike former systems which used Intel Xeon CPUs from 2010-2013, this build made use of modern computer components. It makes use of the ECC memory support of certain AMD Ryzen desktop CPUs and motherboards that support it. In the past I avoided using Ryzen processors due to the uncertainty of ECC memory support. This time I was able to find a motherboard that I could ensure that ECC memory is supported. Others online were able to help me find out that the CPU's memory controller was configured for it with a feature within CPU-Z on Windows.

The "rebuild" of the system with the Cooler Master N200 case was mostly to make the system more portable while the motherboard was already in a microATX format. The idea to have the system more portable is from how there is the possibility of going on another long business trip or vacation. Though, bringing the desktop on trips may only be possible if traveling by road as it would be too difficult to transport the system on aircraft due to its weight and fragility.

# Configuration
Current specifications as of May 2, 2024:

- Motherboard: ASRock B550M Pro4
- CPU: AMD Ryzen 5 5600
- Memory: 32GB (2x16GB) DDR4-3200 1Rx8 Unbuffered ECC Micron MTA9ASF2G72AZ-3G2B1
- Graphics: ASRock Challenger D AMD Radeon RX 6700 XT 12GB GDDR6
- Storage, Windows and Linux root: 1TB NVMe PCIe 4.0 SSD SK hynix Platinum P41
- Storage, Linux mounted as /home: 512GB NVMe PCIe 3.0 SSD Samsung PM9A1
- Power Supply: Corsair RM850x
- CPU Cooler: Cooler Master Hyper 212 EVO V2, with a single 120mm Protechnic Magic MGT12012ZB-W25
- Case Fans: 3x 120mm Protechnic Magic MGT12012ZB-W25
- Case: Cooler Master N200

## Part Selection

### Motherboard
Motherboard selection was critical for the success of the build as support for ECC memory is uncertain. I chose the ASRock B550M Pro4 and found one on eBay for less than US$100.

### CPU
The AMD Ryzen 5 5600 was chosen as a budget option while maintaining support for ECC memory as the [5500 does not support ECC memory along with PCIe 4.0](https://www.amd.com/en/product/11811).

### Memory
In January 2023, when I built the system, I did not have any Unbuffered ECC modules on hand so a couple 8GB Corsair Vengeance LPX ver8.31 (Nanya Technology NT5AD1024M8B3) modules were used to test the configuration. Later on, two Samsung 8GB Samsung M391A1K43BB1-CRC DDR4-2400 were installed in order to test the use of ECC memory on the system, the modules were chosen mainly due to their low cost on eBay. Later, two SK hynix HMA81GU7CJR8N-V DDR4-2666 modules replaced the memory module configuration for a slight performance improvement. Soon after, another two of the same modules were added, bringing memory to 32GB. In August 2023, the memory configuration was changed yet again for the now current configuration of two 16GB Micron MTA9ASF2G72AZ-3G2B1 1Rx8 DDR4-3200 Unbuffered ECC, in certain games, this brought a siginificant improvement in performance.

The current configuration using two Micron 16GB modules was the first time I have ever bought memory modules new. The modules were acquired directly from Micron through Crucial. For memory modules with this speed grade and chip density, it would have cost less to get them new than to find an equivalent on eBay.

### Graphics
At first, an HP RX 580 that I have used in former builds was used to test the configuration.

Later on, at an unknown date in early 2023, I found a new-in-box ASRock Challenger D AMD Radeon RX 6700 XT for about US$300 on eBay. While support on Windows was fine, I could not use the card on Debian due to an outdated AMDGPU driver. This later lead to me switching back to Linux Mint because it has a newer version of the driver. I later moved other laptop and desktop systems back to Linux Mint from Debian while continuing to use Debian in server environments.

### Storage
At first, I used a Samsung PM9A1 found new on eBay along with a 1TB WD Enterprise Storage HDD up to January 2024.

In January 2024, I got a 1TB SK hynix Platinum P41 to take place of the PM9A1 for Linux root and Windows storage. The PM9A1 was configured to serve as user storage mounted as /home on Linux Mint while being unused by Windows. I was originally going to use a 1TB Micron Crucial T700 instead of the P41 but none were in stock at the time. 

### Power Supply
The Corsair RM850x was used out of the [Polyethylene](../pc_pe/) build. It may not need an 850 watt power supply and the power supply may end up being used in another build that requires it.

### CPU Cooler
At first, an included AMD Wraith Stealth cooler was used. Due to noise and cooling concerns, a Cooler Master Hyper 212 EVO V2 was later installed with one of the extra case fans taking place of the included fan.

### Case Fans
Four 120mm Protechnic Magic MGT12012ZB-W25 fans were found on eBay. Three were installed on the case while one was used for the CPU cooler.

These fans are highly effective but are loud. If I decide to, I may switch the fans for ones that are more quiet and use the current fans in servers or other projects.

### Case
As described in the first section, the case was chosen for its small size, low cost and lack of a side window.

## Changes
Some changes in the system's configuration was done over time.

- From January 24, 2023 to January 27, 2023, the system used two 8GB Corsair Vengeance LPX ver8.31 (8x Nanya Technology NT5AD1024M8B3) in order to test the system as I did not have any Unbuffered ECC DDR4 modules yet.
- On January 27, 2023, two 8GB Samsung M391A1K43BB1-CRC 1Rx8 DDR4-2400 Unbuffered ECC modules were installed.
- On February 21, 2023, the system was rebuilt into a mini tower case, the Cooler Master N200.
- On February 23, 2023, four 8GB SK hynix HMA81GU7CJR8N-VK 1Rx8 DDR4-2666 Unbuffered ECC modules replaced the current configuration, bringing RAM capacity to 32GB. The M391A1K43BB1-CRC were later used to build ["Polycarbonate"](../pc_pc/).
- On August 30, 2023, the memory configuration was replaced with two 16GB Micron Technology MTA9ASF2G72AZ-3G2B1 1Rx8 DDR4-3200 Unbuffered ECC because of performance concerns with the previous memory configuration. This memory upgrade brought significant and noticable performance improvements, especially with gaming. The modules used beforehand may be used in other projects.
- On January 13, 2024, a SK hynix Platinum P41 was added alongside the Samsung PM9A1 while the 1TB WD Enterprise Storage HDD was removed. This switched the system over to a "all-flash" storage configuration. For workloads that relied on storage, this brought a siginifcant performance improvement.