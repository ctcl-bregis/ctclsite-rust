"Polybutylene Terephthalate" (shorthand "PBT") is the codename given to the desktop computer system that currently serves as WBPC1 for general purpose workloads. The system is considered to be the "main PC" and almost all work and entertainment (gaming) is done on it. It is normally used from eight (8) to fourteen (14) hours a day.

Unlike former systems which used Intel Xeon CPUs from 2010-2013, this build made use of modern computer components. It makes use of the ECC memory support of certain AMD Ryzen desktop CPUs and motherboards that support it. In the past I avoided using Ryzen processors due to the uncertainty of ECC memory support, this time I was able to find a motherboard that I could ensure that it supports ECC memory. Others online were able to help me find out that the CPU's memory controller was configured for it with a feature within CPU-Z on Windows.

The "rebuild" of the system with the Cooler Master N200 case was mostly to make the system more portable while the motherboard was already in a microATX format. This was related to the idea that I could possibly go on another business trip that lasts a week or longer, like the business trip to Fort Myers Beach, Florida in November 2022 or the road trip to Nashville, Tennessee in May 2020. In a situation like that, I could bring the desktop with me. However, this would not apply to situations that I would travel by air as it would be too difficult to travel with something that heavy and fragile.

# Configuration
Current specifications as of March 29, 2024:

- Motherboard: ASRock B550M Pro4
- CPU: AMD Ryzen 5 5600
- Memory: 32GB (2x16GB) DDR4-3200 1Rx8 Unbuffered ECC Micron MTA9ASF2G72AZ-3G2B1
- Graphics: ASRock Challenger D AMD Radeon RX 6700 XT 12GB GDDR6 (Samsung)
- Storage, Windows and Linux root: 1TB NVMe PCIe 4.0 SSD SK hynix Platinum P41
- Storage, Linux mounted as /home: 512GB NVMe PCIe 3.0 SSD Samsung PM9A1
- Power Supply: Corsair RM850x
- CPU Cooler: Cooler Master Hyper 212 EVO V2, with 120mm Protechnic Magic MGT12012ZB-W25
- Case: Cooler Master N200
- Case Fans: 3x 120mm Protechnic Magic MGT12012ZB-W25

The configuration has changed slightly over time

- From January 24, 2023 to January 27, 2023, the system used two 8GB Corsair Vengeance LPX ver8.31 (Nanya Technology NT5AD1024M8B3) in order to test the system as I did not have any Unbuffered ECC DDR4 modules yet.
- On January 27, 2023, two 8GB Samsung M391A1K43BB1-CRC 1Rx8 DDR4-2400 Unbuffered ECC modules were installed.
- On February 21, 2023, the system was rebuilt into a mini tower case, the Cooler Master N200.
- On February 23, 2023, four 8GB SK hynix HMA81GU7CJR8N-VK 1Rx8 DDR4-2666 Unbuffered ECC modules replaced the current configuration, bringing RAM capacity to 32GB. The M391A1K43BB1-CRC were later used to build ["Polycarbonate"](../pc_pc/).
- On August 30, 2023, the memory configuration was replaced with two 16GB Micron Technology MTA9ASF2G72AZ-3G2B1 1Rx8 DDR4-3200 Unbuffered ECC because of performance concerns with the previous memory configuration. This memory upgrade brought significant and noticable performance improvements, especially with gaming.
- On January 13, 2024, a SK hynix Platinum P41 was added alongside the Samsung PM9A1 while the 1TB WD Enterprise Storage HDD was removed. This switched the system over to a "all-flash" storage configuration, bringing a significant performance in nearly every workload.