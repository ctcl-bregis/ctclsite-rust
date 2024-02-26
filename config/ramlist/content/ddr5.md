
## DRAM Vendors

### Current DRAM Vendors
As of September 2023, these are the current vendors of DDR5 silicon.

#### Micron Technology
Micron Technology is an United States-bound manufacturer of DRAM and flash memory.

In late 2021, Micron Technology announced the availability of DDR5 desktop memory modules (DIMM) under the Crucial brand name.[^micron_cruciald5]

##### SpecTek
SpecTek is a division of Micron Technology.[^micron_st]

Micron Technology released a guide for markings on SpecTek DRAM parts: [Laser Mark Guideline](https://media-www.micron.com/-/media/client/global/documents/spectek/support-documents/dram-misc-info/laser-mark-guideline.pdf?la=en&rev=9be87e1f8f6749eab8452f435b2d6d5b)

#### Samsung Electronics
Samsung Electronics, also known as Samsung Semiconductor is a division of the South Korean Samsung group that develops and produces various semiconductor products ranging from DRAM to applications SoCs (System on Chip).

On September 1, 2023. Samsung Electronics announced their 32 Gbit DDR5 SDRAM.[^samsung_32gbit]

#### SK hynix
SK hynix, also known as SK Hynix or Hynix, is a South Korean manufacturer of DRAM and flash memory.

### Other DRAM Vendors
These DRAM vendors do not produce DDR5 just yet and may have plans to introduce their own DDR5 silicon.

#### Nanya Technology
Nanya Technology, a Taiwanese manufacturer of DRAM. The company is a subsidiary of Formosa Plastics Group[^fpg_nanya].

As of September 2023, Nanya Technology has not put DDR5 in production but has plans to do so.[^nanya_2022pb] According to the 2022 Product Brochure, 16 Gbit DDR5 is planned for release into production in Quarter 1, 2024.[^nanya_2022pb]

### Sources
[^micron_sp]: https://www.micron.com/support/spectek-support
[^samsung_32gbit]: https://news.samsung.com/global/samsung-electronics-unveils-industrys-first-and-highest-capacity-12nm-class-32gb-ddr5-dram-ideal-for-the-ai-era
[^fpg_nanya]: https://www.fpg.com.tw/tw/group/8
[^nanya_2022pb]: https://www.nanya.com/Files/1187
[^micron_cruciald5]: https://investors.micron.com/news-releases/news-release-details/microns-new-crucial-ddr5-memory-delivers-blazing-speeds-and

## Module Vendors

### Corsair
Corsair uses "ver" numbers for describing the parts used in memory modules.[^csr_csr1]

On a [help article from Corsair](https://help.corsair.com/hc/en-us/articles/8526931418381-What-integrated-circuits-ICs-are-used-on-my-CORSAIR-memory-), just three different DRAM vendors are listed.

For DDR5, the format is as follows: verA.BC.DD

- A: The first number is the DRAM vendor, this seems to follow the same numbering scheme as DDR3 and DDR4
  - 3: Micron Technology either branded as Micron or SpecTek[^tpu_csr_mic1] [^tpu_csr_mic2] [^csr_csr1]
  - 4: Samsung[^tpu_csr_sam1] [^tom_csr_sam1]
  - 5: SK hynix[^tpu_csr_skh1] [^tpu_csr_skh2] [^tom_csr_skh1]
- B: The second number refers to the capacity per die
  - 4: 16 Gbit[^tpu_csr_skh2] [^tpu_csr_skh1] [^tpu_csr_mic2] [^tpu_csr_sam1] [^tom_csr_sam1] [^tom_csr_skh1]
  - 5: 24 Gbit[^tpu_csr_mic1]
- C: Currently, what this number represents is unknown. If you know what this number represents and you have a source, [contact me]("/")
- DD: The third set of numbers appear to represent the die revision
  - 01: A-die/Rev.A[^tpu_csr_mic2] [^tpu_csr_skh1] [^tom_csr_skh1]
  - 02: B-die/Rev.B[^tpu_csr_mic1] [^tpu_csr_sam1] [^tom_csr_sam1]
  - 13: M-die/Rev.M[^tpu_csr_skh2]


#### Sources
[^csr_csr1]: https://help.corsair.com/hc/en-us/articles/8526931418381-What-integrated-circuits-ICs-are-used-on-my-CORSAIR-memory-
[^tpu_csr_mic1]: https://www.techpowerup.com/review/corsair-vengeance-rgb-ddr5-5600-cl40-2x-24-gb/
[^tpu_csr_mic2]: https://www.techpowerup.com/review/corsair-vengeance-ddr5-5200-2x-16-gb/3.html
[^tpu_csr_sam1]: https://www.techpowerup.com/review/corsair-vengeance-rgb-ddr5-6000-2x-16-gb/3.html
[^tpu_csr_skh1]: https://www.techpowerup.com/review/corsair-vengeance-rgb-ddr5-7000-cl34-2x-16-gb/3.html
[^tpu_csr_skh2]: https://www.techpowerup.com/review/corsair-dominator-platinum-rgb-ddr5-6600-cl32-2x-16-gb/3.html
[^tom_csr_sam1]: https://www.tomshardware.com/reviews/corsair-vengeance-rgb-ddr5-6000-c36-review
[^tom_csr_skh1]: https://www.tomshardware.com/reviews/corsair-vengeance-rgb-ddr5-7000-c34-review